// Pention Engine - platform/virtual_memory.hpp
// Requirement: PN-PLT-005 (virtual memory reservation and commit),
//              PN-PLT-008 (out-of-memory handling)
// Decision:    ADR-0007 (platform acquires storage; core's allocators consume it)

#ifndef PN_PLATFORM_VIRTUAL_MEMORY_HPP
#define PN_PLATFORM_VIRTUAL_MEMORY_HPP

#include "pn/core/error.hpp"
#include "pn/core/expected.hpp"

#include <cstddef>
#include <span>

namespace pn::platform {

using core::Error;
using core::Expected;

/// Page size of this system, queried once.
[[nodiscard]] std::size_t page_size() noexcept;

/// Rounds a byte count up to a whole number of pages.
[[nodiscard]] std::size_t round_up_to_pages(std::size_t bytes) noexcept;

/// A reserved range of address space, committed in whole or in part.
///
/// Reserve and commit are separated because that separation is the entire point
/// of using virtual memory directly rather than malloc: a subsystem can reserve
/// a large, stable address range up front - so pointers into it never move - and
/// commit physical pages only as far as it actually grows.
///
/// Non-copyable and move-only. The range is released on destruction.
class VirtualMemory {
public:
    VirtualMemory() = default;

    /// Reserves address space without committing physical memory.
    ///
    /// Reserved-but-uncommitted pages must not be touched; commit() first.
    [[nodiscard]] static Expected<VirtualMemory, Error> reserve(std::size_t bytes) noexcept;

    /// Reserves and immediately commits the whole range.
    [[nodiscard]] static Expected<VirtualMemory, Error> reserve_and_commit(std::size_t bytes) noexcept;

    VirtualMemory(const VirtualMemory&) = delete;
    VirtualMemory& operator=(const VirtualMemory&) = delete;

    VirtualMemory(VirtualMemory&& other) noexcept;
    VirtualMemory& operator=(VirtualMemory&& other) noexcept;

    ~VirtualMemory();

    /// Backs the first `bytes` of the reservation with physical memory.
    ///
    /// Rounded up to a page boundary. Committing a range already committed is
    /// permitted and is a no-op on both supported platforms.
    [[nodiscard]] Expected<void, Error> commit(std::size_t bytes) noexcept;

    /// Releases physical backing for everything beyond `bytes`, keeping the
    /// address reservation. Pointers into the decommitted region remain valid
    /// addresses but must not be read or written until recommitted.
    [[nodiscard]] Expected<void, Error> decommit_beyond(std::size_t bytes) noexcept;

    /// Releases the reservation. Called by the destructor; explicit release is
    /// for callers that need the failure reported.
    [[nodiscard]] Expected<void, Error> release() noexcept;

    /// The committed region, as a span suitable for handing to an allocator.
    ///
    /// This is the join between this module and core's allocators (ADR-0007):
    /// platform acquires, core consumes.
    [[nodiscard]] std::span<std::byte> committed() noexcept {
        return std::span<std::byte>{base_, committed_bytes_};
    }

    [[nodiscard]] std::size_t reserved_bytes() const noexcept { return reserved_bytes_; }
    [[nodiscard]] std::size_t committed_bytes() const noexcept { return committed_bytes_; }
    [[nodiscard]] bool valid() const noexcept { return base_ != nullptr; }

private:
    std::byte* base_ = nullptr;
    std::size_t reserved_bytes_ = 0;
    std::size_t committed_bytes_ = 0;
};

}  // namespace pn::platform

#endif  // PN_PLATFORM_VIRTUAL_MEMORY_HPP
