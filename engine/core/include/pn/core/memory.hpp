// Pention Engine - core/memory.hpp
// Requirement: PN-PLT-006 (tagged allocators, arenas, pools, alignment),
//              PN-PLT-007 (memory tracking and leak detection),
//              PN-PLT-008 (out-of-memory handling)
// Decision:    ADR-0007 (allocators take storage; they do not acquire it)

#ifndef PN_CORE_MEMORY_HPP
#define PN_CORE_MEMORY_HPP

#include "pn/core/error.hpp"
#include "pn/core/expected.hpp"

#include <atomic>
#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <memory>
#include <new>
#include <span>
#include <string_view>
#include <type_traits>
#include <utility>

namespace pn::core {

// ---------------------------------------------------------------------------
// Alignment
// ---------------------------------------------------------------------------

[[nodiscard]] constexpr bool is_power_of_two(std::size_t value) noexcept {
    return value != 0 && (value & (value - 1)) == 0;
}

[[nodiscard]] constexpr std::size_t align_up(std::size_t value, std::size_t alignment) noexcept {
    // Caller guarantees a power-of-two alignment; checked at the call sites that
    // take it from user input.
    return (value + alignment - 1) & ~(alignment - 1);
}

[[nodiscard]] constexpr std::size_t align_padding(std::size_t value, std::size_t alignment) noexcept {
    return align_up(value, alignment) - value;
}

// ---------------------------------------------------------------------------
// Tagged attribution
// ---------------------------------------------------------------------------

/// Attribution category for an allocation.
///
/// The point is answering "what is using the memory" without a profiler
/// attached. A budget that cannot be attributed cannot be enforced, and every
/// large-world budget in this project is per-subsystem.
enum class MemoryTag : std::uint16_t {
    general = 0,
    scratch,
    string,
    container,
    scene,
    assets,
    geometry,
    texture,
    audio,
    physics,
    animation,
    world_streaming,
    render,
    scripting,
    editor,
    count_,  ///< sentinel; not a tag
};

inline constexpr std::size_t kMemoryTagCount = static_cast<std::size_t>(MemoryTag::count_);

[[nodiscard]] constexpr std::string_view to_string(MemoryTag tag) noexcept {
    switch (tag) {
        case MemoryTag::general:         return "general";
        case MemoryTag::scratch:         return "scratch";
        case MemoryTag::string:          return "string";
        case MemoryTag::container:       return "container";
        case MemoryTag::scene:           return "scene";
        case MemoryTag::assets:          return "assets";
        case MemoryTag::geometry:        return "geometry";
        case MemoryTag::texture:         return "texture";
        case MemoryTag::audio:           return "audio";
        case MemoryTag::physics:         return "physics";
        case MemoryTag::animation:       return "animation";
        case MemoryTag::world_streaming: return "world_streaming";
        case MemoryTag::render:          return "render";
        case MemoryTag::scripting:       return "scripting";
        case MemoryTag::editor:          return "editor";
        case MemoryTag::count_:          break;
    }
    return "invalid";
}

/// Live and peak byte counts for one tag.
struct MemoryStats {
    std::size_t live_bytes = 0;
    std::size_t peak_bytes = 0;
    std::size_t total_allocations = 0;
    std::size_t live_allocations = 0;
};

/// Process-wide tagged memory accounting.
///
/// Counters are atomic because allocation happens on worker threads. They are
/// relaxed: the numbers are diagnostics, and ordering them against the
/// allocations themselves would cost more than the information is worth.
///
/// `peak_bytes` is what budgets are actually set from - a live figure sampled at
/// an arbitrary moment says very little about whether a budget holds.
class MemoryTracker {
public:
    static void record_allocation(MemoryTag tag, std::size_t bytes) noexcept;
    static void record_deallocation(MemoryTag tag, std::size_t bytes) noexcept;

    [[nodiscard]] static MemoryStats stats(MemoryTag tag) noexcept;
    [[nodiscard]] static MemoryStats total() noexcept;

    /// Zeroes every counter. Test-only; calling this in a running engine
    /// destroys the accounting it exists to provide.
    static void reset_all() noexcept;

    /// True if any tag still reports live bytes.
    ///
    /// Named for what it detects rather than what it hopes: at shutdown a live
    /// count is a leak, and a test that asserts this is how the leak is found
    /// rather than reported by a user.
    [[nodiscard]] static bool has_live_allocations() noexcept;

private:
    struct Counters {
        std::atomic<std::size_t> live_bytes{0};
        std::atomic<std::size_t> peak_bytes{0};
        std::atomic<std::size_t> total_allocations{0};
        std::atomic<std::size_t> live_allocations{0};
    };

    static Counters& counters(MemoryTag tag) noexcept;
};

// ---------------------------------------------------------------------------
// Arena
// ---------------------------------------------------------------------------

/// Bump allocator over storage it does not own.
///
/// Allocation is a pointer increment. There is no individual free: the whole
/// arena is reset at once, or rewound to a marker. That makes it the right tool
/// for per-frame and per-task scratch, where the lifetime of everything in it
/// ends at the same moment, and the wrong tool for anything outliving that
/// moment.
///
/// It never grows (ADR-0007). Exhaustion returns an error rather than aborting,
/// so out-of-memory is a handleable outcome.
class Arena {
public:
    /// A position in the arena, for rewinding.
    class Marker {
    public:
        Marker() = default;

    private:
        friend class Arena;
        explicit Marker(std::size_t offset) noexcept : offset_(offset) {}
        std::size_t offset_ = 0;
    };

    Arena() = default;

    explicit Arena(std::span<std::byte> storage, MemoryTag tag = MemoryTag::scratch) noexcept
        : storage_(storage), tag_(tag) {}

    Arena(const Arena&) = delete;
    Arena& operator=(const Arena&) = delete;
    Arena(Arena&&) noexcept = default;
    Arena& operator=(Arena&&) noexcept = default;

    /// Allocates `bytes` with the given alignment.
    ///
    /// Returns `out_of_memory` when the remaining space cannot satisfy the
    /// request including the alignment padding, and `invalid_argument` for a
    /// non-power-of-two alignment.
    [[nodiscard]] Expected<void*, Error> allocate(std::size_t bytes,
                                                  std::size_t alignment = alignof(std::max_align_t)) noexcept {
        if (!is_power_of_two(alignment)) {
            return fail(ErrorCategory::invalid_argument, "alignment must be a power of two");
        }
        if (bytes == 0) {
            return fail(ErrorCategory::invalid_argument, "zero-byte allocation");
        }

        const std::size_t padding = align_padding(offset_, alignment);
        // Checked before the addition so a pathological size cannot wrap.
        if (padding > storage_.size() - offset_) {
            return fail(ErrorCategory::out_of_memory, "arena exhausted by alignment padding");
        }
        const std::size_t aligned_offset = offset_ + padding;
        if (bytes > storage_.size() - aligned_offset) {
            return fail(ErrorCategory::out_of_memory, "arena exhausted");
        }

        std::byte* result = storage_.data() + aligned_offset;
        offset_ = aligned_offset + bytes;
        if (offset_ > high_water_) {
            high_water_ = offset_;
        }
        MemoryTracker::record_allocation(tag_, bytes);
        live_bytes_ += bytes;
        return static_cast<void*>(result);
    }

    /// Allocates and constructs one T.
    ///
    /// Restricted to trivially destructible types. An arena reset does not run
    /// destructors, so permitting a type that needs one would make resource
    /// leaks the default behaviour rather than a mistake.
    template <typename T, typename... Args>
    [[nodiscard]] Expected<T*, Error> create(Args&&... args) noexcept {
        static_assert(std::is_trivially_destructible_v<T>,
                      "Arena does not run destructors on reset; use a different allocator "
                      "for types that need one");
        PN_TRY_ASSIGN(void* raw, allocate(sizeof(T), alignof(T)));
        return ::new (raw) T(std::forward<Args>(args)...);
    }

    /// Allocates an uninitialized array of T.
    template <typename T>
    [[nodiscard]] Expected<std::span<T>, Error> allocate_array(std::size_t count) noexcept {
        static_assert(std::is_trivially_destructible_v<T>,
                      "Arena does not run destructors on reset");
        if (count == 0) {
            return fail(ErrorCategory::invalid_argument, "zero-length array");
        }
        if (count > (static_cast<std::size_t>(-1) / sizeof(T))) {
            return fail(ErrorCategory::invalid_argument, "array size overflows");
        }
        PN_TRY_ASSIGN(void* raw, allocate(sizeof(T) * count, alignof(T)));
        return std::span<T>{static_cast<T*>(raw), count};
    }

    [[nodiscard]] Marker mark() const noexcept { return Marker{offset_}; }

    /// Rewinds to a previously taken marker. Everything allocated after it
    /// becomes invalid; the storage is reused.
    void release_to(Marker marker) noexcept {
        if (marker.offset_ > offset_) {
            return;  // a marker from a later state; rewinding forward is meaningless
        }
        const std::size_t reclaimed = offset_ - marker.offset_;
        offset_ = marker.offset_;
        const std::size_t recorded = reclaimed < live_bytes_ ? reclaimed : live_bytes_;
        MemoryTracker::record_deallocation(tag_, recorded);
        live_bytes_ -= recorded;
    }

    void reset() noexcept { release_to(Marker{0}); }

    [[nodiscard]] std::size_t used() const noexcept { return offset_; }
    [[nodiscard]] std::size_t capacity() const noexcept { return storage_.size(); }
    [[nodiscard]] std::size_t remaining() const noexcept { return storage_.size() - offset_; }

    /// Largest `used()` ever reached. This is the figure a budget is set from.
    [[nodiscard]] std::size_t high_water() const noexcept { return high_water_; }

    [[nodiscard]] MemoryTag tag() const noexcept { return tag_; }

    /// True if the pointer lies within this arena's storage.
    [[nodiscard]] bool owns(const void* pointer) const noexcept {
        const auto* p = static_cast<const std::byte*>(pointer);
        return p >= storage_.data() && p < storage_.data() + storage_.size();
    }

private:
    std::span<std::byte> storage_{};
    std::size_t offset_ = 0;
    std::size_t high_water_ = 0;
    std::size_t live_bytes_ = 0;
    MemoryTag tag_ = MemoryTag::scratch;
};

/// Rewinds an arena to its construction-time position on scope exit.
///
/// The common shape for per-task scratch: take the arena, allocate freely, and
/// have the space reclaimed on the way out without every early return having to
/// remember.
class ArenaScope {
public:
    explicit ArenaScope(Arena& arena) noexcept : arena_(&arena), marker_(arena.mark()) {}

    ArenaScope(const ArenaScope&) = delete;
    ArenaScope& operator=(const ArenaScope&) = delete;
    ArenaScope(ArenaScope&&) = delete;
    ArenaScope& operator=(ArenaScope&&) = delete;

    ~ArenaScope() { arena_->release_to(marker_); }

private:
    Arena* arena_;
    Arena::Marker marker_;
};

// ---------------------------------------------------------------------------
// Pool
// ---------------------------------------------------------------------------

/// Fixed-size block allocator over storage it does not own.
///
/// Unlike an arena, individual blocks are returned. Free blocks hold the free
/// list, so the pool carries no per-block overhead beyond the blocks themselves.
///
/// The link is stored by memcpy rather than by placing a pointer object in the
/// storage, because writing a pointer into raw bytes and reading it back as a
/// different type is exactly the aliasing violation UBSan exists to catch.
class PoolAllocator {
public:
    static constexpr std::uint32_t kNoBlock = 0xFFFF'FFFFu;

    PoolAllocator() = default;

    /// @param storage    memory to carve into blocks
    /// @param block_size bytes per block; rounded up to `alignment`
    /// @param alignment  power-of-two alignment for every block
    PoolAllocator(std::span<std::byte> storage,
                  std::size_t block_size,
                  std::size_t alignment = alignof(std::max_align_t),
                  MemoryTag tag = MemoryTag::general) noexcept
        : storage_(storage), tag_(tag) {
        if (!is_power_of_two(alignment) || block_size == 0) {
            return;  // stays empty; block_count() == 0 reports the failure
        }
        // A block must be able to hold the free-list link.
        const std::size_t minimum = sizeof(std::uint32_t);
        block_size_ = align_up(block_size < minimum ? minimum : block_size, alignment);
        alignment_ = alignment;

        // The caller's storage may not start on the requested alignment, so the
        // first block begins at the next aligned address inside it.
        //
        // std::align rather than hand-rolled pointer arithmetic: computing this
        // by hand needs a uintptr_t-to-size_t conversion that is a genuine
        // narrowing on some platforms and a no-op on others, so no single
        // spelling is both correct everywhere and warning-free everywhere.
        void* aligned_pointer = storage.data();
        std::size_t available = storage.size();
        if (std::align(alignment, block_size_, aligned_pointer, available) == nullptr) {
            return;  // cannot fit even one aligned block; block_count() stays 0
        }
        base_offset_ = static_cast<std::size_t>(
            static_cast<std::byte*>(aligned_pointer) - storage.data());
        block_count_ = available / block_size_;

        // Thread the free list through every block, front to back, so the first
        // allocations walk forward in memory rather than backwards.
        free_head_ = block_count_ == 0 ? kNoBlock : 0u;
        for (std::size_t i = 0; i < block_count_; ++i) {
            const std::uint32_t next =
                (i + 1 < block_count_) ? static_cast<std::uint32_t>(i + 1) : kNoBlock;
            write_link(i, next);
        }
    }

    PoolAllocator(const PoolAllocator&) = delete;
    PoolAllocator& operator=(const PoolAllocator&) = delete;
    PoolAllocator(PoolAllocator&&) noexcept = default;
    PoolAllocator& operator=(PoolAllocator&&) noexcept = default;

    [[nodiscard]] Expected<void*, Error> allocate() noexcept {
        if (free_head_ == kNoBlock) {
            return fail(ErrorCategory::out_of_memory, "pool exhausted");
        }
        const std::uint32_t index = free_head_;
        free_head_ = read_link(index);
        ++live_blocks_;
        if (live_blocks_ > high_water_) {
            high_water_ = live_blocks_;
        }
        MemoryTracker::record_allocation(tag_, block_size_);
        return static_cast<void*>(block_pointer(index));
    }

    /// Returns a block to the pool.
    ///
    /// A pointer that is not a block start, or lies outside the pool, is
    /// rejected rather than corrupting the free list - which is what makes the
    /// mistake findable instead of producing damage far from its cause.
    [[nodiscard]] Expected<void, Error> deallocate(void* pointer) noexcept {
        if (pointer == nullptr) {
            return Unexpected{Error{ErrorCategory::invalid_argument, "null pointer"}};
        }
        const auto* p = static_cast<std::byte*>(pointer);
        const std::byte* base = storage_.data() + base_offset_;
        if (p < base || p >= base + block_count_ * block_size_) {
            return Unexpected{Error{ErrorCategory::invalid_argument,
                                    "pointer does not belong to this pool"}};
        }
        const auto byte_offset = static_cast<std::size_t>(p - base);
        if (byte_offset % block_size_ != 0) {
            return Unexpected{Error{ErrorCategory::invalid_argument,
                                    "pointer is not the start of a block"}};
        }
        const auto index = static_cast<std::uint32_t>(byte_offset / block_size_);
        write_link(index, free_head_);
        free_head_ = index;
        --live_blocks_;
        MemoryTracker::record_deallocation(tag_, block_size_);
        return {};
    }

    [[nodiscard]] std::size_t block_size() const noexcept { return block_size_; }
    [[nodiscard]] std::size_t block_count() const noexcept { return block_count_; }
    [[nodiscard]] std::size_t live_blocks() const noexcept { return live_blocks_; }
    [[nodiscard]] std::size_t free_blocks() const noexcept { return block_count_ - live_blocks_; }
    [[nodiscard]] std::size_t high_water() const noexcept { return high_water_; }
    [[nodiscard]] std::size_t alignment() const noexcept { return alignment_; }

private:
    [[nodiscard]] std::byte* block_pointer(std::uint32_t index) noexcept {
        return storage_.data() + base_offset_ + static_cast<std::size_t>(index) * block_size_;
    }

    void write_link(std::size_t index, std::uint32_t next) noexcept {
        std::byte* target = storage_.data() + base_offset_ + index * block_size_;
        std::memcpy(target, &next, sizeof(next));
    }

    [[nodiscard]] std::uint32_t read_link(std::uint32_t index) noexcept {
        std::uint32_t next = kNoBlock;
        std::memcpy(&next, block_pointer(index), sizeof(next));
        return next;
    }

    std::span<std::byte> storage_{};
    std::size_t base_offset_ = 0;
    std::size_t block_size_ = 0;
    std::size_t alignment_ = alignof(std::max_align_t);
    std::size_t block_count_ = 0;
    std::size_t live_blocks_ = 0;
    std::size_t high_water_ = 0;
    std::uint32_t free_head_ = kNoBlock;
    MemoryTag tag_ = MemoryTag::general;
};

}  // namespace pn::core

#endif  // PN_CORE_MEMORY_HPP
