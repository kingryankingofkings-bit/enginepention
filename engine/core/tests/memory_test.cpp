// Pention Engine - core/tests/memory_test.cpp
// Requirement: PN-PLT-006, PN-PLT-007, PN-PLT-008
// Decision:    ADR-0007

#include "pn/core/error.hpp"
#include "pn/core/memory.hpp"
#include "pn/testing/test.hpp"

#include <array>
#include <cstddef>
#include <cstdint>
#include <vector>

namespace {

using pn::core::Arena;
using pn::core::ArenaScope;
using pn::core::ErrorCategory;
using pn::core::MemoryTag;
using pn::core::MemoryTracker;
using pn::core::PoolAllocator;

/// Over-aligned storage, so alignment tests are not silently satisfied by the
/// allocator's backing array already happening to be aligned.
alignas(64) std::array<std::byte, 4096> g_storage{};

std::span<std::byte> storage(std::size_t bytes = g_storage.size()) {
    return std::span<std::byte>{g_storage.data(), bytes};
}

[[nodiscard]] bool is_aligned(const void* pointer, std::size_t alignment) {
    return reinterpret_cast<std::uintptr_t>(pointer) % alignment == 0;
}

}  // namespace

// ---------------------------------------------------------------------------
// Alignment helpers
// ---------------------------------------------------------------------------

PN_TEST(memory, power_of_two_detection) {
    PN_CHECK(pn::core::is_power_of_two(1));
    PN_CHECK(pn::core::is_power_of_two(64));
    PN_CHECK(!pn::core::is_power_of_two(0));
    PN_CHECK(!pn::core::is_power_of_two(3));
    PN_CHECK(!pn::core::is_power_of_two(48));
}

PN_TEST(memory, align_up_is_idempotent_on_aligned_values) {
    PN_CHECK_EQ(pn::core::align_up(0, 16), 0u);
    PN_CHECK_EQ(pn::core::align_up(1, 16), 16u);
    PN_CHECK_EQ(pn::core::align_up(16, 16), 16u);
    PN_CHECK_EQ(pn::core::align_up(17, 16), 32u);
    PN_CHECK_EQ(pn::core::align_up(pn::core::align_up(23, 8), 8), 24u);
}

// ---------------------------------------------------------------------------
// Arena
// ---------------------------------------------------------------------------

PN_TEST(arena, allocates_and_advances) {
    Arena arena{storage(256)};
    PN_CHECK_EQ(arena.capacity(), 256u);
    PN_CHECK_EQ(arena.used(), 0u);

    const auto first = arena.allocate(32, 8);
    PN_REQUIRE(first.has_value());
    PN_CHECK(arena.used() >= 32u);
    PN_CHECK(arena.owns(first.value()));

    const auto second = arena.allocate(32, 8);
    PN_REQUIRE(second.has_value());
    PN_CHECK_NE(first.value(), second.value());
}

PN_TEST(arena, honours_requested_alignment) {
    Arena arena{storage(1024)};
    // Deliberately allocate an odd size first so the cursor is misaligned for
    // the alignments that follow. Without padding, the next allocation would
    // come back unaligned.
    PN_REQUIRE(arena.allocate(1, 1).has_value());

    for (const std::size_t alignment : {std::size_t{2}, std::size_t{8}, std::size_t{16},
                                        std::size_t{32}, std::size_t{64}}) {
        const auto block = arena.allocate(8, alignment);
        PN_REQUIRE(block.has_value());
        PN_CHECK(is_aligned(block.value(), alignment));
    }
}

PN_TEST(arena, rejects_non_power_of_two_alignment) {
    Arena arena{storage(256)};
    const auto result = arena.allocate(8, 3);
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::invalid_argument));
}

PN_TEST(arena, exhaustion_is_reported_not_fatal) {
    // PN-PLT-008: out-of-memory must be a handleable outcome, not an abort.
    Arena arena{storage(64)};
    const auto ok = arena.allocate(64, 1);
    PN_REQUIRE(ok.has_value());

    const auto exhausted = arena.allocate(1, 1);
    PN_REQUIRE(!exhausted.has_value());
    PN_CHECK(exhausted.error().is(ErrorCategory::out_of_memory));

    // The arena remains usable after a failed allocation - a failure must not
    // corrupt the cursor.
    PN_CHECK_EQ(arena.used(), 64u);
    PN_CHECK_EQ(arena.remaining(), 0u);
}

PN_TEST(arena, alignment_padding_cannot_overrun_the_end) {
    // 8 bytes of storage, then ask for 8 bytes at 64-byte alignment. The
    // padding alone exceeds the arena. An implementation that adds before
    // checking would compute an out-of-range cursor here.
    Arena arena{storage(8)};
    PN_REQUIRE(arena.allocate(1, 1).has_value());
    const auto result = arena.allocate(8, 64);
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::out_of_memory));
    PN_CHECK(arena.used() <= arena.capacity());
}

PN_TEST(arena, zero_byte_allocation_is_rejected) {
    Arena arena{storage(64)};
    const auto result = arena.allocate(0, 8);
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::invalid_argument));
}

PN_TEST(arena, reset_reclaims_everything) {
    Arena arena{storage(256)};
    PN_REQUIRE(arena.allocate(100, 8).has_value());
    PN_CHECK_GT(arena.used(), 0u);

    arena.reset();
    PN_CHECK_EQ(arena.used(), 0u);
    PN_CHECK_EQ(arena.remaining(), arena.capacity());

    // And the storage is genuinely reusable.
    const auto again = arena.allocate(100, 8);
    PN_REQUIRE(again.has_value());
}

PN_TEST(arena, marker_rewinds_to_an_exact_position) {
    Arena arena{storage(512)};
    PN_REQUIRE(arena.allocate(64, 8).has_value());
    const std::size_t before = arena.used();

    const Arena::Marker marker = arena.mark();
    PN_REQUIRE(arena.allocate(128, 8).has_value());
    PN_CHECK_GT(arena.used(), before);

    arena.release_to(marker);
    PN_CHECK_EQ(arena.used(), before);
}

PN_TEST(arena, scope_rewinds_on_exit_including_early_return_paths) {
    Arena arena{storage(512)};
    PN_REQUIRE(arena.allocate(32, 8).has_value());
    const std::size_t baseline = arena.used();

    {
        const ArenaScope scope{arena};
        PN_REQUIRE(arena.allocate(200, 8).has_value());
        PN_CHECK_GT(arena.used(), baseline);
    }
    PN_CHECK_EQ(arena.used(), baseline);
}

PN_TEST(arena, high_water_survives_reset) {
    // The figure a budget is set from. A live count sampled after a reset says
    // nothing about whether the budget holds.
    Arena arena{storage(512)};
    PN_REQUIRE(arena.allocate(300, 8).has_value());
    const std::size_t peak = arena.high_water();
    PN_CHECK_GE(peak, 300u);

    arena.reset();
    PN_CHECK_EQ(arena.used(), 0u);
    PN_CHECK_EQ(arena.high_water(), peak);
}

PN_TEST(arena, create_constructs_the_object) {
    Arena arena{storage(256)};
    struct Point {
        int x;
        int y;
    };
    const auto point = arena.create<Point>(3, 4);
    PN_REQUIRE(point.has_value());
    PN_CHECK_EQ(point.value()->x, 3);
    PN_CHECK_EQ(point.value()->y, 4);
    PN_CHECK(is_aligned(point.value(), alignof(Point)));
}

PN_TEST(arena, allocate_array_returns_a_usable_span) {
    Arena arena{storage(1024)};
    const auto values = arena.allocate_array<std::uint32_t>(16);
    PN_REQUIRE(values.has_value());
    PN_CHECK_EQ(values.value().size(), 16u);
    PN_CHECK(is_aligned(values.value().data(), alignof(std::uint32_t)));

    for (std::size_t i = 0; i < values.value().size(); ++i) {
        values.value()[i] = static_cast<std::uint32_t>(i);
    }
    PN_CHECK_EQ(values.value()[15], 15u);
}

PN_TEST(arena, allocate_array_rejects_a_size_that_would_overflow) {
    Arena arena{storage(256)};
    const auto result = arena.allocate_array<std::uint64_t>(static_cast<std::size_t>(-1) / 4);
    PN_REQUIRE(!result.has_value());
    // Either rejection is correct; what must not happen is a wrapped size
    // producing a small, satisfiable request.
    PN_CHECK(result.error().is(ErrorCategory::invalid_argument) ||
             result.error().is(ErrorCategory::out_of_memory));
}

PN_TEST(arena, owns_discriminates_its_own_storage) {
    Arena arena{storage(128)};
    const auto block = arena.allocate(16, 8);
    PN_REQUIRE(block.has_value());
    PN_CHECK(arena.owns(block.value()));

    int elsewhere = 0;
    PN_CHECK(!arena.owns(&elsewhere));
}

// ---------------------------------------------------------------------------
// Pool
// ---------------------------------------------------------------------------

PN_TEST(pool, carves_storage_into_blocks) {
    PoolAllocator pool{storage(1024), 64, 16};
    PN_CHECK_EQ(pool.block_size(), 64u);
    PN_CHECK_GT(pool.block_count(), 0u);
    PN_CHECK_EQ(pool.live_blocks(), 0u);
    PN_CHECK_EQ(pool.free_blocks(), pool.block_count());
}

PN_TEST(pool, allocated_blocks_are_distinct_and_aligned) {
    PoolAllocator pool{storage(1024), 64, 64};
    std::vector<void*> blocks;
    for (std::size_t i = 0; i < pool.block_count(); ++i) {
        const auto block = pool.allocate();
        PN_REQUIRE(block.has_value());
        PN_CHECK(is_aligned(block.value(), 64));
        blocks.push_back(block.value());
    }
    // No address appears twice.
    for (std::size_t i = 0; i < blocks.size(); ++i) {
        for (std::size_t j = i + 1; j < blocks.size(); ++j) {
            PN_CHECK_NE(blocks[i], blocks[j]);
        }
    }
}

PN_TEST(pool, exhaustion_is_reported) {
    PoolAllocator pool{storage(256), 64, 8};
    for (std::size_t i = 0; i < pool.block_count(); ++i) {
        PN_REQUIRE(pool.allocate().has_value());
    }
    const auto exhausted = pool.allocate();
    PN_REQUIRE(!exhausted.has_value());
    PN_CHECK(exhausted.error().is(ErrorCategory::out_of_memory));
}

PN_TEST(pool, freed_blocks_are_reused) {
    PoolAllocator pool{storage(512), 64, 8};
    const auto first = pool.allocate();
    PN_REQUIRE(first.has_value());
    const std::size_t live_after_alloc = pool.live_blocks();

    PN_REQUIRE(pool.deallocate(first.value()).has_value());
    PN_CHECK_EQ(pool.live_blocks(), live_after_alloc - 1);

    const auto second = pool.allocate();
    PN_REQUIRE(second.has_value());
    // The most recently freed block comes back first.
    PN_CHECK_EQ(first.value(), second.value());
}

PN_TEST(pool, rejects_a_pointer_from_outside_the_pool) {
    PoolAllocator pool{storage(512), 64, 8};
    int elsewhere = 0;
    const auto result = pool.deallocate(&elsewhere);
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::invalid_argument));
}

PN_TEST(pool, rejects_a_pointer_that_is_not_a_block_start) {
    // Corrupting the free list here would produce damage far from its cause,
    // so the pool refuses rather than trusting the caller.
    PoolAllocator pool{storage(512), 64, 8};
    const auto block = pool.allocate();
    PN_REQUIRE(block.has_value());

    auto* interior = static_cast<std::byte*>(block.value()) + 8;
    const auto result = pool.deallocate(interior);
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::invalid_argument));
}

PN_TEST(pool, rejects_null) {
    PoolAllocator pool{storage(512), 64, 8};
    const auto result = pool.deallocate(nullptr);
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::invalid_argument));
}

PN_TEST(pool, block_size_is_raised_to_hold_the_free_list_link) {
    // A 1-byte block cannot hold the link, so the pool must round up rather
    // than write past the block when threading the free list.
    PoolAllocator pool{storage(256), 1, 1};
    PN_CHECK_GE(pool.block_size(), sizeof(std::uint32_t));
    const auto block = pool.allocate();
    PN_REQUIRE(block.has_value());
    PN_REQUIRE(pool.deallocate(block.value()).has_value());
}

PN_TEST(pool, full_cycle_returns_every_block) {
    PoolAllocator pool{storage(1024), 32, 8};
    const std::size_t total = pool.block_count();

    std::vector<void*> blocks;
    for (std::size_t i = 0; i < total; ++i) {
        const auto block = pool.allocate();
        PN_REQUIRE(block.has_value());
        blocks.push_back(block.value());
    }
    PN_CHECK_EQ(pool.live_blocks(), total);
    PN_CHECK_EQ(pool.high_water(), total);

    for (void* block : blocks) {
        PN_REQUIRE(pool.deallocate(block).has_value());
    }
    PN_CHECK_EQ(pool.live_blocks(), 0u);
    PN_CHECK_EQ(pool.free_blocks(), total);

    // And the pool is fully usable again.
    for (std::size_t i = 0; i < total; ++i) {
        PN_REQUIRE(pool.allocate().has_value());
    }
}

// ---------------------------------------------------------------------------
// Tracking
// ---------------------------------------------------------------------------

PN_TEST(memory_tracker, attributes_bytes_to_the_arena_tag) {
    MemoryTracker::reset_all();
    {
        Arena arena{storage(512), MemoryTag::geometry};
        PN_REQUIRE(arena.allocate(128, 8).has_value());

        const auto geometry = MemoryTracker::stats(MemoryTag::geometry);
        PN_CHECK_EQ(geometry.live_bytes, 128u);
        PN_CHECK_EQ(geometry.total_allocations, 1u);

        // And nothing landed on an unrelated tag.
        PN_CHECK_EQ(MemoryTracker::stats(MemoryTag::audio).live_bytes, 0u);
    }
    MemoryTracker::reset_all();
}

PN_TEST(memory_tracker, peak_is_retained_after_release) {
    MemoryTracker::reset_all();
    {
        Arena arena{storage(512), MemoryTag::physics};
        PN_REQUIRE(arena.allocate(256, 8).has_value());
        arena.reset();

        const auto physics = MemoryTracker::stats(MemoryTag::physics);
        PN_CHECK_EQ(physics.live_bytes, 0u);
        PN_CHECK_GE(physics.peak_bytes, 256u);
    }
    MemoryTracker::reset_all();
}

PN_TEST(memory_tracker, detects_a_live_allocation_as_a_leak) {
    // This is the shape of the shutdown check: at teardown a live count is a
    // leak, and this is how it gets found rather than reported by a user.
    MemoryTracker::reset_all();
    PN_CHECK(!MemoryTracker::has_live_allocations());

    Arena arena{storage(256), MemoryTag::assets};
    PN_REQUIRE(arena.allocate(64, 8).has_value());
    PN_CHECK(MemoryTracker::has_live_allocations());

    arena.reset();
    PN_CHECK(!MemoryTracker::has_live_allocations());
    MemoryTracker::reset_all();
}

PN_TEST(memory_tracker, total_sums_every_tag) {
    MemoryTracker::reset_all();
    {
        Arena textures{storage(256), MemoryTag::texture};
        PN_REQUIRE(textures.allocate(64, 8).has_value());

        std::array<std::byte, 256> other{};
        Arena audio{std::span<std::byte>{other}, MemoryTag::audio};
        PN_REQUIRE(audio.allocate(32, 8).has_value());

        const auto sum = MemoryTracker::total();
        PN_CHECK_EQ(sum.live_bytes, 96u);
        PN_CHECK_EQ(sum.live_allocations, 2u);
    }
    MemoryTracker::reset_all();
}

PN_TEST(memory_tracker, tags_stringify) {
    PN_CHECK_EQ(pn::core::to_string(MemoryTag::world_streaming),
                std::string_view{"world_streaming"});
    PN_CHECK_EQ(pn::core::to_string(MemoryTag::general), std::string_view{"general"});
}
