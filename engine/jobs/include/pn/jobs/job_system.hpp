// Pention Engine - jobs/job_system.hpp
// Requirement: PN-PLT-013 (work-stealing job graph with priorities and dependencies),
//              PN-PLT-014 (synchronization, cancellation, deadlock diagnostics)
// Decision:    ADR-0008

#ifndef PN_JOBS_JOB_SYSTEM_HPP
#define PN_JOBS_JOB_SYSTEM_HPP

#include "pn/core/error.hpp"
#include "pn/core/expected.hpp"
#include "pn/jobs/work_stealing_deque.hpp"

#include <array>
#include <atomic>
#include <condition_variable>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <mutex>
#include <new>
#include <thread>
#include <type_traits>
#include <utility>
#include <vector>

namespace pn::jobs {

using core::Error;
using core::ErrorCategory;
using core::Expected;

/// Bytes available for a job's captured state.
///
/// Sized so a Job occupies two cache lines. Captures larger than this are a
/// compile error rather than a silent heap allocation: an allocation on the
/// per-frame scheduling path is exactly what this design exists to avoid, and
/// hiding one here would make it invisible.
inline constexpr std::size_t kJobPayloadBytes = 96;

/// A unit of work, plus its place in the dependency graph.
///
/// `unfinished` counts this job plus every child not yet complete, which is what
/// makes waiting on a parent wait for the whole subtree without the parent
/// having to know how many children exist.
struct alignas(64) Job {
    using Function = void (*)(Job&) noexcept;

    Function function = nullptr;
    Job* parent = nullptr;
    std::atomic<std::int32_t> unfinished{0};
    std::atomic<bool> cancelled{false};
    alignas(16) std::array<std::byte, kJobPayloadBytes> payload{};
};

/// Work-stealing scheduler.
///
/// One deque per worker. A worker runs its own work from the bottom and steals
/// from the top of a random victim when it runs dry, so the common path touches
/// only the worker's own end of its own deque.
///
/// The calling thread participates: `wait` runs available work rather than
/// blocking. That is what keeps a job that waits on its own children from
/// deadlocking the pool, which is otherwise the standard way a thread pool of
/// this shape dies.
class JobSystem {
public:
    /// Jobs allocatable per worker before indices wrap.
    ///
    /// Job storage is a per-worker ring. Allocating more than this many jobs on
    /// one worker while earlier ones are still live recycles a job that is still
    /// in use. The limit is deliberately large and checked in debug builds.
    static constexpr std::size_t kJobsPerWorker = 4096;

    // Both defined in the translation unit that completes Worker: a defaulted
    // constructor here would instantiate the members' destructors for
    // construction cleanup, and unique_ptr<Worker> cannot do that against an
    // incomplete type.
    JobSystem();
    ~JobSystem();

    JobSystem(const JobSystem&) = delete;
    JobSystem& operator=(const JobSystem&) = delete;

    /// Starts `worker_count` worker threads, or one per hardware thread minus
    /// the calling thread when zero.
    [[nodiscard]] Expected<void, Error> start(std::size_t worker_count = 0);

    /// Signals shutdown and joins every worker. Safe to call more than once.
    void stop() noexcept;

    [[nodiscard]] bool running() const noexcept {
        return running_.load(std::memory_order_acquire);
    }
    /// Number of worker threads. The thread that called start() participates in
    /// `wait` but is not counted here.
    [[nodiscard]] std::size_t worker_count() const noexcept { return threads_.size(); }

    /// Index of the calling thread within the pool. The thread that called
    /// start() is index `worker_count()`; it participates in `wait` but is never
    /// given work directly.
    [[nodiscard]] std::size_t this_thread_index() const noexcept;

    /// Creates a job. It does not run until scheduled.
    template <typename F>
    [[nodiscard]] Job* create(F&& callable) noexcept {
        return create_child(nullptr, std::forward<F>(callable));
    }

    /// Creates a job whose completion is counted by `parent`.
    ///
    /// Waiting on the parent waits for this job too. Children must be created
    /// before the parent is waited on, or the wait may observe the parent
    /// complete before the child is registered.
    template <typename F>
    [[nodiscard]] Job* create_child(Job* parent, F&& callable) noexcept {
        using Callable = std::decay_t<F>;
        static_assert(sizeof(Callable) <= kJobPayloadBytes,
                      "job capture is too large; pass a pointer to caller-owned storage");
        static_assert(alignof(Callable) <= 16, "job capture is over-aligned");
        static_assert(std::is_trivially_destructible_v<Callable>,
                      "job storage is recycled without running destructors; capture only "
                      "trivially destructible state");

        Job* job = allocate_job();
        if (job == nullptr) {
            return nullptr;
        }

        job->parent = parent;
        job->unfinished.store(1, std::memory_order_relaxed);
        job->cancelled.store(false, std::memory_order_relaxed);

        if (parent != nullptr) {
            parent->unfinished.fetch_add(1, std::memory_order_relaxed);
        }

        ::new (static_cast<void*>(job->payload.data())) Callable(std::forward<F>(callable));
        job->function = [](Job& self) noexcept {
            auto* stored = std::launder(reinterpret_cast<Callable*>(self.payload.data()));
            (*stored)();
        };
        return job;
    }

    /// Makes a job available to run.
    [[nodiscard]] bool schedule(Job* job) noexcept;

    /// Runs available work until `job` and its whole subtree are complete.
    ///
    /// The waiting thread executes jobs rather than blocking, so waiting never
    /// removes a thread from the pool.
    void wait(Job* job) noexcept;

    /// Marks a job cancelled. A cancelled job's body is skipped, but the job
    /// still completes so anything waiting on it is released.
    ///
    /// Cancellation does not propagate to children: a child already scheduled
    /// runs unless cancelled itself. Propagating would require walking a
    /// structure that is being mutated concurrently, and silently half-doing it
    /// would be worse than not doing it.
    static void cancel(Job* job) noexcept {
        if (job != nullptr) {
            job->cancelled.store(true, std::memory_order_release);
        }
    }

    [[nodiscard]] static bool is_complete(const Job* job) noexcept {
        return job == nullptr || job->unfinished.load(std::memory_order_acquire) <= 0;
    }

    /// Splits [0, count) into chunks of at most `grain` and runs `body(begin,
    /// end)` on each. Returns when every chunk has completed.
    template <typename F>
    void parallel_for(std::size_t count, std::size_t grain, F&& body) noexcept {
        if (count == 0) {
            return;
        }
        if (grain == 0) {
            grain = 1;
        }

        // Running inline when there is nothing to gain avoids paying scheduling
        // cost to save less than it costs.
        if (worker_count() == 0 || count <= grain) {
            body(std::size_t{0}, count);
            return;
        }

        Job* root = create([] {});
        if (root == nullptr) {
            body(std::size_t{0}, count);
            return;
        }

        // `body` is referenced, not copied, so a large functor does not have to
        // fit the payload. It outlives every chunk because this function does
        // not return until they are all complete.
        auto* body_pointer = std::addressof(body);
        using Body = std::remove_reference_t<F>;

        for (std::size_t begin = 0; begin < count; begin += grain) {
            const std::size_t end = (begin + grain < count) ? begin + grain : count;
            Job* chunk = create_child(root, [body_pointer, begin, end]() noexcept {
                (*static_cast<Body*>(body_pointer))(begin, end);
            });
            if (chunk == nullptr || !schedule(chunk)) {
                // Out of job slots or deque space: run this chunk inline rather
                // than dropping the work.
                body(begin, end);
                if (chunk != nullptr) {
                    finish(chunk);
                }
                continue;
            }
        }

        if (!schedule(root)) {
            finish(root);
        }
        wait(root);
    }

    /// Diagnostics. Counters are relaxed and approximate under concurrency.
    struct Statistics {
        std::uint64_t executed = 0;
        std::uint64_t stolen = 0;
        std::uint64_t steal_attempts = 0;
        std::uint64_t schedule_failures = 0;
    };
    [[nodiscard]] Statistics statistics() const noexcept;

private:
    struct Worker;

    [[nodiscard]] Worker* worker_at(std::size_t index) noexcept;
    [[nodiscard]] Job* allocate_job() noexcept;
    [[nodiscard]] Job* find_work(std::size_t worker_index) noexcept;
    void execute(Job* job) noexcept;
    void finish(Job* job) noexcept;
    void worker_main(std::size_t worker_index) noexcept;

    std::vector<std::thread> threads_;

    // One entry per worker thread, plus a final entry for the thread that
    // called start(). The submitting thread's deque must be a steal target like
    // any other: work submitted from the main thread would otherwise be visible
    // only to the main thread, and the pool would sit idle while it ran
    // everything itself.
    std::vector<std::unique_ptr<Worker>> workers_;

    std::atomic<bool> running_{false};
    std::atomic<bool> stopping_{false};

    // Idle workers sleep rather than spin: on a small core count, spinning
    // workers steal cycles from the ones doing useful work.
    std::mutex sleep_mutex_;
    std::condition_variable sleep_signal_;
    std::atomic<std::uint32_t> work_epoch_{0};

    std::atomic<std::uint64_t> executed_{0};
    std::atomic<std::uint64_t> stolen_{0};
    std::atomic<std::uint64_t> steal_attempts_{0};
    std::atomic<std::uint64_t> schedule_failures_{0};
};

}  // namespace pn::jobs

#endif  // PN_JOBS_JOB_SYSTEM_HPP
