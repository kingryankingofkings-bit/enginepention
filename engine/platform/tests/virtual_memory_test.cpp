// Pention Engine - platform/tests/virtual_memory_test.cpp
// Requirement: PN-PLT-005, PN-PLT-008
// Decision:    ADR-0007

#include "pn/core/error.hpp"
#include "pn/core/memory.hpp"
#include "pn/platform/virtual_memory.hpp"
#include "pn/testing/test.hpp"

#include <cstddef>
#include <utility>

namespace {
using pn::core::Arena;
using pn::core::ErrorCategory;
using pn::core::MemoryTag;
using pn::platform::VirtualMemory;
}  // namespace

PN_TEST(virtual_memory, page_size_is_a_sane_power_of_two) {
    const std::size_t page = pn::platform::page_size();
    PN_CHECK_GE(page, 1024u);
    PN_CHECK(pn::core::is_power_of_two(page));
}

PN_TEST(virtual_memory, rounding_to_pages_is_exact) {
    const std::size_t page = pn::platform::page_size();
    PN_CHECK_EQ(pn::platform::round_up_to_pages(1), page);
    PN_CHECK_EQ(pn::platform::round_up_to_pages(page), page);
    PN_CHECK_EQ(pn::platform::round_up_to_pages(page + 1), page * 2);
}

PN_TEST(virtual_memory, reserve_does_not_commit) {
    auto reservation = VirtualMemory::reserve(1024 * 1024);
    PN_REQUIRE(reservation.has_value());
    PN_CHECK(reservation.value().valid());
    PN_CHECK_GE(reservation.value().reserved_bytes(), 1024u * 1024u);
    PN_CHECK_EQ(reservation.value().committed_bytes(), 0u);
    PN_CHECK_EQ(reservation.value().committed().size(), 0u);
}

PN_TEST(virtual_memory, committed_memory_is_writable) {
    auto reservation = VirtualMemory::reserve(64 * 1024);
    PN_REQUIRE(reservation.has_value());
    PN_REQUIRE(reservation.value().commit(4096).has_value());

    auto bytes = reservation.value().committed();
    PN_REQUIRE(bytes.size() >= 4096u);

    // Touch the whole committed range. If commit were a no-op this faults, and
    // under ASan it is reported rather than silently corrupting.
    for (std::size_t i = 0; i < bytes.size(); ++i) {
        bytes[i] = static_cast<std::byte>(i & 0xFF);
    }
    PN_CHECK_EQ(bytes[0], static_cast<std::byte>(0));
    PN_CHECK_EQ(bytes[255], static_cast<std::byte>(255));
}

PN_TEST(virtual_memory, commit_beyond_the_reservation_is_rejected) {
    auto reservation = VirtualMemory::reserve(4096);
    PN_REQUIRE(reservation.has_value());
    const auto result = reservation.value().commit(1024 * 1024 * 64);
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::out_of_range));
}

PN_TEST(virtual_memory, recommitting_is_idempotent) {
    auto reservation = VirtualMemory::reserve(64 * 1024);
    PN_REQUIRE(reservation.has_value());
    PN_REQUIRE(reservation.value().commit(8192).has_value());
    const std::size_t first = reservation.value().committed_bytes();

    PN_REQUIRE(reservation.value().commit(4096).has_value());
    // A smaller commit must not shrink the committed region.
    PN_CHECK_EQ(reservation.value().committed_bytes(), first);

    PN_REQUIRE(reservation.value().commit(8192).has_value());
    PN_CHECK_EQ(reservation.value().committed_bytes(), first);
}

PN_TEST(virtual_memory, growing_the_commit_keeps_earlier_contents) {
    // The reason reserve and commit are separate: the address range is stable,
    // so growing the commit must not move or disturb what is already there.
    auto reservation = VirtualMemory::reserve(1024 * 1024);
    PN_REQUIRE(reservation.has_value());
    PN_REQUIRE(reservation.value().commit(4096).has_value());

    auto first_view = reservation.value().committed();
    std::byte* original_base = first_view.data();
    first_view[0] = static_cast<std::byte>(0xAB);
    first_view[4095] = static_cast<std::byte>(0xCD);

    PN_REQUIRE(reservation.value().commit(64 * 1024).has_value());
    auto grown = reservation.value().committed();

    PN_CHECK_EQ(grown.data(), original_base);  // the base address did not move
    PN_CHECK_GE(grown.size(), 64u * 1024u);
    PN_CHECK_EQ(grown[0], static_cast<std::byte>(0xAB));
    PN_CHECK_EQ(grown[4095], static_cast<std::byte>(0xCD));
}

PN_TEST(virtual_memory, decommit_shrinks_the_committed_region) {
    auto reservation = VirtualMemory::reserve(1024 * 1024);
    PN_REQUIRE(reservation.has_value());
    PN_REQUIRE(reservation.value().commit(64 * 1024).has_value());
    PN_REQUIRE(reservation.value().decommit_beyond(8192).has_value());

    PN_CHECK_LE(reservation.value().committed_bytes(), 8192u + pn::platform::page_size());
    // The reservation itself is retained.
    PN_CHECK_GE(reservation.value().reserved_bytes(), 1024u * 1024u);
    PN_CHECK(reservation.value().valid());

    // What remains committed is still writable.
    auto bytes = reservation.value().committed();
    PN_REQUIRE(bytes.size() > 0u);
    bytes[0] = static_cast<std::byte>(7);
    PN_CHECK_EQ(bytes[0], static_cast<std::byte>(7));
}

PN_TEST(virtual_memory, zero_byte_reservation_is_rejected) {
    const auto result = VirtualMemory::reserve(0);
    PN_REQUIRE(!result.has_value());
    PN_CHECK(result.error().is(ErrorCategory::invalid_argument));
}

PN_TEST(virtual_memory, move_transfers_ownership_exactly_once) {
    auto source = VirtualMemory::reserve(64 * 1024);
    PN_REQUIRE(source.has_value());
    PN_REQUIRE(source.value().commit(4096).has_value());
    const std::size_t reserved = source.value().reserved_bytes();

    VirtualMemory moved = std::move(source).value();
    PN_CHECK(moved.valid());
    PN_CHECK_EQ(moved.reserved_bytes(), reserved);

    // Writing through the moved-to object proves the mapping survived; a double
    // release would have been caught by the destructor running twice, which
    // ASan reports.
    auto bytes = moved.committed();
    PN_REQUIRE(bytes.size() > 0u);
    bytes[0] = static_cast<std::byte>(1);
    PN_CHECK_EQ(bytes[0], static_cast<std::byte>(1));
}

PN_TEST(virtual_memory, release_is_idempotent) {
    auto reservation = VirtualMemory::reserve(4096);
    PN_REQUIRE(reservation.has_value());
    PN_REQUIRE(reservation.value().release().has_value());
    PN_CHECK(!reservation.value().valid());
    // A second release must be a harmless no-op, not a double unmap.
    PN_CHECK(reservation.value().release().has_value());
}

PN_TEST(virtual_memory, composes_with_a_core_arena) {
    // ADR-0007's join: platform acquires storage, core consumes it. This is the
    // shape every subsystem uses to get an arena.
    auto reservation = VirtualMemory::reserve_and_commit(256 * 1024);
    PN_REQUIRE(reservation.has_value());

    Arena arena{reservation.value().committed(), MemoryTag::world_streaming};
    PN_CHECK_GE(arena.capacity(), 256u * 1024u);

    const auto block = arena.allocate(1024, 64);
    PN_REQUIRE(block.has_value());
    PN_CHECK(arena.owns(block.value()));

    // And the memory handed over is genuinely usable.
    auto* bytes = static_cast<std::byte*>(block.value());
    for (std::size_t i = 0; i < 1024; ++i) {
        bytes[i] = static_cast<std::byte>(i & 0xFF);
    }
    PN_CHECK_EQ(bytes[512], static_cast<std::byte>(0));
}
