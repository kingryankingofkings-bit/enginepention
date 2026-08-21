// Pention Engine - jobs/job_system.cpp
// Requirement: PN-PLT-013, PN-PLT-014
// Decision:    ADR-0008

#include "pn/jobs/job_system.hpp"

#include "pn/platform/thread.hpp"

#include <chrono>
#include <cstdio>
#include <random>
#include <string>

namespace pn::jobs {
namespace {

/// Index of the calling thread within its JobSystem, or kNotAWorker.
///
/// thread_local rather than a lookup: it is read on every allocation and every
/// steal, and a map lookup there would dominate the cost of small jobs.
constexpr std::size_t kNotAWorker = static_cast<std::size_t>(-1);
thread_local std::size_t t_worker_index = kNotAWorker;
thread_local const void* t_owning_system = nullptr;

}  // namespace

/// Per-thread scheduler state.
///
/// One per worker plus one for the thread that called start(), so that thread
/// can allocate jobs and participate in `wait` without a special case
/// everywhere.
struct JobSystem::Worker {
    explicit Worker(std::size_t index, std::size_t deque_capacity)
        : deque(deque_capacity),
          storage(std::make_unique<Job[]>(kJobsPerWorker)),
          rng(static_cast<std::uint32_t>(index * 2654435761u + 1u)) {}

    WorkStealingDeque<Job> deque;
    // unique_ptr<Job[]> rather than vector<Job>: Job holds atomics, so it is
    // neither copyable nor movable, and every vector growth path needs one or
    // the other. The array is allocated once at its final size anyway.
    std::unique_ptr<Job[]> storage;
    std::size_t next_slot = 0;
    std::minstd_rand rng;
};

JobSystem::JobSystem() = default;

JobSystem::~JobSystem() {
    stop();
}

Expected<void, Error> JobSystem::start(std::size_t worker_count) {
    if (running_.load(std::memory_order_acquire)) {
        return core::Unexpected{Error{ErrorCategory::invalid_state, "job system already started"}};
    }

    if (worker_count == 0) {
        const std::size_t hardware = platform::hardware_thread_count();
        // Leave the calling thread a core. On a single-core machine this yields
        // zero workers, and the system runs everything inline - which is
        // correct, not a degenerate case to guard against.
        worker_count = hardware > 1 ? hardware - 1 : 0;
    }

    stopping_.store(false, std::memory_order_relaxed);

    // worker_count entries for the threads, plus one for the calling thread.
    // Keeping them in one array is what lets a worker steal from the submitting
    // thread's deque without a special case.
    workers_.clear();
    workers_.reserve(worker_count + 1);
    for (std::size_t i = 0; i <= worker_count; ++i) {
        workers_.push_back(std::make_unique<Worker>(i, kJobsPerWorker));
    }

    t_worker_index = worker_count;  // the calling thread owns the last slot
    t_owning_system = this;

    running_.store(true, std::memory_order_release);

    threads_.reserve(worker_count);
    for (std::size_t i = 0; i < worker_count; ++i) {
        threads_.emplace_back([this, i] { worker_main(i); });
    }

    return {};
}

void JobSystem::stop() noexcept {
    if (!running_.exchange(false, std::memory_order_acq_rel)) {
        return;  // never started, or already stopped
    }

    stopping_.store(true, std::memory_order_release);
    {
        // Taking the lock before notifying closes the window where a worker has
        // checked the predicate but not yet waited, which would otherwise miss
        // the wake-up and hang shutdown.
        const std::lock_guard<std::mutex> lock(sleep_mutex_);
        work_epoch_.fetch_add(1, std::memory_order_relaxed);
    }
    sleep_signal_.notify_all();

    for (std::thread& thread : threads_) {
        if (thread.joinable()) {
            thread.join();
        }
    }
    threads_.clear();
    workers_.clear();

    if (t_owning_system == this) {
        t_worker_index = kNotAWorker;
        t_owning_system = nullptr;
    }
}

std::size_t JobSystem::this_thread_index() const noexcept {
    if (t_owning_system != this) {
        return kNotAWorker;
    }
    return t_worker_index;
}

JobSystem::Worker* JobSystem::worker_at(std::size_t index) noexcept {
    return index < workers_.size() ? workers_[index].get() : nullptr;
}

Job* JobSystem::allocate_job() noexcept {
    const std::size_t index = this_thread_index();
    Worker* worker = (index == kNotAWorker) ? nullptr : worker_at(index);
    if (worker == nullptr) {
        return nullptr;
    }

    // Ring allocation. Recycling a slot whose job is still live would be a
    // use-after-free, so the wrap is checked rather than assumed.
    Job& job = worker->storage[worker->next_slot & (kJobsPerWorker - 1)];
    if (job.unfinished.load(std::memory_order_acquire) > 0) {
        return nullptr;  // ring exhausted; caller falls back to inline execution
    }
    ++worker->next_slot;

    job.function = nullptr;
    job.parent = nullptr;
    return &job;
}

bool JobSystem::schedule(Job* job) noexcept {
    if (job == nullptr) {
        return false;
    }

    const std::size_t index = this_thread_index();
    Worker* worker = (index == kNotAWorker) ? nullptr : worker_at(index);

    if (worker == nullptr || !worker->deque.push_bottom(job)) {
        schedule_failures_.fetch_add(1, std::memory_order_relaxed);
        return false;
    }

    // Wake one sleeper. The epoch bump under the lock is what makes the
    // predicate a sleeping worker checks observably change, so a worker that
    // was between check and wait does not sleep through the notification.
    {
        const std::lock_guard<std::mutex> lock(sleep_mutex_);
        work_epoch_.fetch_add(1, std::memory_order_relaxed);
    }
    sleep_signal_.notify_one();
    return true;
}

Job* JobSystem::find_work(std::size_t worker_index) noexcept {
    Worker* self = worker_at(worker_index);
    if (self != nullptr) {
        if (Job* own = self->deque.pop_bottom(); own != nullptr) {
            return own;
        }
    }

    // Every deque is a candidate, including the submitting thread's.
    const std::size_t victim_count = workers_.size();
    if (victim_count <= 1) {
        return nullptr;  // nothing to steal from but ourselves
    }

    // Random victim selection. Round-robin makes several idle workers converge
    // on the same target and repeatedly lose the same race.
    const std::size_t start = (self != nullptr) ? (self->rng() % victim_count) : 0;
    for (std::size_t offset = 0; offset < victim_count; ++offset) {
        const std::size_t victim = (start + offset) % victim_count;
        if (victim == worker_index) {
            continue;
        }
        steal_attempts_.fetch_add(1, std::memory_order_relaxed);
        if (Job* stolen = workers_[victim]->deque.steal_top(); stolen != nullptr) {
            stolen_.fetch_add(1, std::memory_order_relaxed);
            return stolen;
        }
    }
    return nullptr;
}

void JobSystem::execute(Job* job) noexcept {
    if (job->function != nullptr && !job->cancelled.load(std::memory_order_acquire)) {
        job->function(*job);
    }
    executed_.fetch_add(1, std::memory_order_relaxed);
    finish(job);
}

void JobSystem::finish(Job* job) noexcept {
    // The parent pointer is read BEFORE the decrement, and this ordering is
    // load-bearing rather than stylistic.
    //
    // Reaching zero is precisely the signal that makes this slot eligible for
    // recycling: allocate_job() reuses any slot whose count is not positive and
    // immediately overwrites `function` and `parent`. Reading job->parent after
    // the decrement therefore races the owning thread's reuse of the slot -
    // and it is a real race, not a tooling artifact. It reproduced under
    // ThreadSanitizer and the reported pair was exactly this read against
    // allocate_job's write.
    //
    // After the decrement, `job` must be treated as no longer ours.
    Job* const parent = job->parent;

    // acq_rel so the work the job performed happens-before any waiter observing
    // the count reach zero.
    const std::int32_t remaining = job->unfinished.fetch_sub(1, std::memory_order_acq_rel) - 1;

    if (remaining == 0 && parent != nullptr) {
        finish(parent);
    }
}

void JobSystem::wait(Job* job) noexcept {
    if (job == nullptr) {
        return;
    }

    const std::size_t index = this_thread_index();

    while (job->unfinished.load(std::memory_order_acquire) > 0) {
        Job* next = (index == kNotAWorker) ? nullptr : find_work(index);
        if (next != nullptr) {
            execute(next);
        } else {
            // Nothing to run: let another thread have the core rather than
            // burning it. Sleeping here would risk waiting on work that only
            // this thread can produce.
            platform::yield_current_thread();
        }
    }
}

void JobSystem::worker_main(std::size_t worker_index) noexcept {
    t_worker_index = worker_index;
    t_owning_system = this;

    platform::set_current_thread_name("pn-worker-" + std::to_string(worker_index));

    while (!stopping_.load(std::memory_order_acquire)) {
        if (Job* job = find_work(worker_index); job != nullptr) {
            execute(job);
            continue;
        }

        // Nothing found. Sleep until the epoch moves, with a timeout so a lost
        // wake-up costs latency rather than a hang.
        const std::uint32_t observed = work_epoch_.load(std::memory_order_relaxed);
        std::unique_lock<std::mutex> lock(sleep_mutex_);
        sleep_signal_.wait_for(lock, std::chrono::milliseconds(2), [&] {
            return stopping_.load(std::memory_order_acquire) ||
                   work_epoch_.load(std::memory_order_relaxed) != observed;
        });
    }

    t_worker_index = kNotAWorker;
    t_owning_system = nullptr;
}

JobSystem::Statistics JobSystem::statistics() const noexcept {
    return Statistics{
        executed_.load(std::memory_order_relaxed),
        stolen_.load(std::memory_order_relaxed),
        steal_attempts_.load(std::memory_order_relaxed),
        schedule_failures_.load(std::memory_order_relaxed),
    };
}

}  // namespace pn::jobs
