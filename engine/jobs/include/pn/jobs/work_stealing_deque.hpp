// Pention Engine - jobs/work_stealing_deque.hpp
// Requirement: PN-PLT-013 (work-stealing job graph)
// Decision:    ADR-0008
//
// A bounded single-producer, multi-consumer double-ended queue.
//
// The owning worker pushes and pops at the bottom; thieves take from the top.
// That asymmetry is the point: the owner's common path touches only its own end
// and never contends with thieves, so the scheduler's hot path is
// uncontended in the usual case and only pays for synchronization when work is
// actually being stolen.
//
// Implemented from the published description of the algorithm (Chase and Lev,
// "Dynamic Circular Work-Stealing Deque"), from the prose and the invariants
// rather than from any reference implementation - see PROVENANCE_LEDGER.md.
// Bounded rather than growable: capacity is a budget here, as everywhere else
// in this engine, and a deque that reallocates under concurrent theft is a much
// harder object to reason about for a benefit this scheduler does not need.
//
// The memory ordering is the entire correctness argument, so each fence and
// each ordering below carries the reason it is there. Getting one of them wrong
// produces a bug that appears only under contention, only on weakly ordered
// hardware, and never in a debugger.

#ifndef PN_JOBS_WORK_STEALING_DEQUE_HPP
#define PN_JOBS_WORK_STEALING_DEQUE_HPP

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <vector>

namespace pn::jobs {

/// Bounded work-stealing deque of pointers.
///
/// One thread - the owner - may call push_bottom and pop_bottom. Any number of
/// other threads may call steal_top concurrently with those and with each
/// other. Calling push_bottom or pop_bottom from more than one thread is
/// undefined.
template <typename T>
class WorkStealingDeque {
public:
    /// @param capacity rounded up to a power of two so indexing is a mask
    explicit WorkStealingDeque(std::size_t capacity) {
        std::size_t rounded = 1;
        while (rounded < capacity) {
            rounded <<= 1;
        }
        mask_ = rounded - 1;
        buffer_ = std::vector<std::atomic<T*>>(rounded);
        for (auto& slot : buffer_) {
            slot.store(nullptr, std::memory_order_relaxed);
        }
    }

    WorkStealingDeque(const WorkStealingDeque&) = delete;
    WorkStealingDeque& operator=(const WorkStealingDeque&) = delete;

    /// Owner only. Returns false when the deque is full.
    [[nodiscard]] bool push_bottom(T* item) noexcept {
        const std::int64_t bottom = bottom_.load(std::memory_order_relaxed);
        // Acquire: we must observe steals that have already completed, or we
        // will believe the deque is fuller than it is and spuriously fail.
        const std::int64_t top = top_.load(std::memory_order_acquire);

        if (bottom - top > static_cast<std::int64_t>(mask_)) {
            return false;  // full
        }

        buffer_[static_cast<std::size_t>(bottom) & mask_].store(item, std::memory_order_relaxed);

        // Release store rather than a release fence plus a relaxed store. The
        // two are equivalent here by the standard's fence rules, but a release
        // store is modelled precisely by race detectors while a standalone
        // fence is not - ThreadSanitizer reports the publication of the job
        // payload as a race against the fence formulation. Given the choice
        // between a formulation a tool can verify and one it cannot, on the
        // single most concurrency-critical object in the engine, the verifiable
        // one wins.
        bottom_.store(bottom + 1, std::memory_order_release);
        return true;
    }

    /// Owner only. Returns nullptr when empty, or when a thief won the race for
    /// the last remaining item.
    [[nodiscard]] T* pop_bottom() noexcept {
        const std::int64_t bottom = bottom_.load(std::memory_order_relaxed) - 1;

        // Both operations are sequentially consistent, and that is the crux of
        // the algorithm: the decremented bottom must be visible to thieves
        // before we read top, and their increments of top must be visible to
        // us. Anything weaker lets the owner and a thief both conclude they
        // took the same last item.
        //
        // Expressed as seq_cst operations rather than relaxed operations around
        // a seq_cst fence. The two are equivalent under the standard, but GCC
        // refuses outright to compile std::atomic_thread_fence under
        // -fsanitize=thread, and ThreadSanitizer models standalone fences
        // imprecisely in any case. A formulation that both supported compilers
        // can build and both sanitizers can verify is worth more here than the
        // marginal instruction it might save on some architectures - especially
        // as this host cannot measure that difference.
        bottom_.store(bottom, std::memory_order_seq_cst);
        std::int64_t top = top_.load(std::memory_order_seq_cst);

        if (top > bottom) {
            bottom_.store(bottom + 1, std::memory_order_relaxed);  // restore; empty
            return nullptr;
        }

        T* item = buffer_[static_cast<std::size_t>(bottom) & mask_].load(std::memory_order_relaxed);

        if (top != bottom) {
            return item;  // more than one item remains; no thief can contend
        }

        // Exactly one item left, and a thief may be taking it right now. Settle
        // it with a compare-exchange on top: whoever moves top wins.
        bool won = top_.compare_exchange_strong(top, top + 1,
                                                std::memory_order_seq_cst,
                                                std::memory_order_relaxed);
        bottom_.store(bottom + 1, std::memory_order_relaxed);  // deque is now empty either way
        return won ? item : nullptr;
    }

    /// Any thread. Returns nullptr when empty or when the race was lost.
    ///
    /// A lost race is reported as an ordinary empty result: the caller retries
    /// against another victim, which is cheaper than spinning here.
    [[nodiscard]] T* steal_top() noexcept {
        // Sequentially consistent, pairing with pop_bottom so the owner and a
        // thief cannot both take the final item. The load of bottom_ is also
        // the acquire that orders the buffer read below against the owner's
        // release store in push_bottom.
        std::int64_t top = top_.load(std::memory_order_seq_cst);
        const std::int64_t bottom = bottom_.load(std::memory_order_seq_cst);

        if (top >= bottom) {
            return nullptr;  // empty
        }

        T* item = buffer_[static_cast<std::size_t>(top) & mask_].load(std::memory_order_relaxed);

        if (!top_.compare_exchange_strong(top, top + 1,
                                          std::memory_order_seq_cst,
                                          std::memory_order_relaxed)) {
            return nullptr;  // another thief, or the owner, got there first
        }
        return item;
    }

    /// Approximate. Correct only when observed by the owner with no theft in
    /// flight; for diagnostics, never for control flow.
    [[nodiscard]] std::size_t size_approx() const noexcept {
        const std::int64_t bottom = bottom_.load(std::memory_order_relaxed);
        const std::int64_t top = top_.load(std::memory_order_relaxed);
        return bottom > top ? static_cast<std::size_t>(bottom - top) : 0;
    }

    [[nodiscard]] std::size_t capacity() const noexcept { return mask_ + 1; }

    [[nodiscard]] bool empty_approx() const noexcept { return size_approx() == 0; }

private:
    // top_ and bottom_ are written by different threads on the hot path, so
    // they are placed in separate cache lines. Sharing one line would make every
    // owner push invalidate the line each thief is polling.
    alignas(64) std::atomic<std::int64_t> top_{0};
    alignas(64) std::atomic<std::int64_t> bottom_{0};

    std::vector<std::atomic<T*>> buffer_;
    std::size_t mask_ = 0;
};

}  // namespace pn::jobs

#endif  // PN_JOBS_WORK_STEALING_DEQUE_HPP
