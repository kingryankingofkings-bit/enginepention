// Pention Engine - core/memory.cpp
// Requirement: PN-PLT-007 (memory tracking and leak detection)
// Decision:    ADR-0007

#include "pn/core/memory.hpp"

#include <array>

namespace pn::core {

MemoryTracker::Counters& MemoryTracker::counters(MemoryTag tag) noexcept {
    // Function-local static so the table is initialized before the first
    // allocation records into it, whatever the translation-unit ordering.
    static std::array<Counters, kMemoryTagCount> table{};
    const auto index = static_cast<std::size_t>(tag);
    return table[index < kMemoryTagCount ? index : 0];
}

void MemoryTracker::record_allocation(MemoryTag tag, std::size_t bytes) noexcept {
    Counters& c = counters(tag);
    const std::size_t live = c.live_bytes.fetch_add(bytes, std::memory_order_relaxed) + bytes;
    c.total_allocations.fetch_add(1, std::memory_order_relaxed);
    c.live_allocations.fetch_add(1, std::memory_order_relaxed);

    // Compare-and-swap rather than a plain store: two threads raising the peak
    // concurrently must not let the smaller of the two win.
    std::size_t observed_peak = c.peak_bytes.load(std::memory_order_relaxed);
    while (live > observed_peak &&
           !c.peak_bytes.compare_exchange_weak(observed_peak, live,
                                               std::memory_order_relaxed,
                                               std::memory_order_relaxed)) {
        // observed_peak is refreshed by compare_exchange_weak on failure.
    }
}

void MemoryTracker::record_deallocation(MemoryTag tag, std::size_t bytes) noexcept {
    Counters& c = counters(tag);
    c.live_bytes.fetch_sub(bytes, std::memory_order_relaxed);
    c.live_allocations.fetch_sub(1, std::memory_order_relaxed);
}

MemoryStats MemoryTracker::stats(MemoryTag tag) noexcept {
    Counters& c = counters(tag);
    return MemoryStats{
        c.live_bytes.load(std::memory_order_relaxed),
        c.peak_bytes.load(std::memory_order_relaxed),
        c.total_allocations.load(std::memory_order_relaxed),
        c.live_allocations.load(std::memory_order_relaxed),
    };
}

MemoryStats MemoryTracker::total() noexcept {
    MemoryStats sum{};
    for (std::size_t i = 0; i < kMemoryTagCount; ++i) {
        const MemoryStats one = stats(static_cast<MemoryTag>(i));
        sum.live_bytes += one.live_bytes;
        sum.peak_bytes += one.peak_bytes;
        sum.total_allocations += one.total_allocations;
        sum.live_allocations += one.live_allocations;
    }
    return sum;
}

void MemoryTracker::reset_all() noexcept {
    for (std::size_t i = 0; i < kMemoryTagCount; ++i) {
        Counters& c = counters(static_cast<MemoryTag>(i));
        c.live_bytes.store(0, std::memory_order_relaxed);
        c.peak_bytes.store(0, std::memory_order_relaxed);
        c.total_allocations.store(0, std::memory_order_relaxed);
        c.live_allocations.store(0, std::memory_order_relaxed);
    }
}

bool MemoryTracker::has_live_allocations() noexcept {
    for (std::size_t i = 0; i < kMemoryTagCount; ++i) {
        if (counters(static_cast<MemoryTag>(i)).live_bytes.load(std::memory_order_relaxed) != 0) {
            return true;
        }
    }
    return false;
}

}  // namespace pn::core
