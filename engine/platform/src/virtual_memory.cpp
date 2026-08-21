// Pention Engine - platform/virtual_memory.cpp
// Requirement: PN-PLT-005, PN-PLT-008
// Decision:    ADR-0007
//
// Two implementations behind one interface. The Windows path is written but
// cannot be compiled or executed in the current environment (BLOCK-001), so it
// is IMPLEMENTED_UNVERIFIED and is labelled as such in the traceability matrix
// rather than counted as working.

#include "pn/platform/virtual_memory.hpp"

#include <utility>

#if defined(_WIN32)
#  define NOMINMAX
#  define WIN32_LEAN_AND_MEAN
#  include <windows.h>
#else
#  include <sys/mman.h>
#  include <unistd.h>
#endif

namespace pn::platform {
namespace {

using core::ErrorCategory;
using core::Unexpected;

std::size_t query_page_size() noexcept {
#if defined(_WIN32)
    SYSTEM_INFO info{};
    ::GetSystemInfo(&info);
    return static_cast<std::size_t>(info.dwPageSize);
#else
    const long size = ::sysconf(_SC_PAGESIZE);
    return size > 0 ? static_cast<std::size_t>(size) : std::size_t{4096};
#endif
}

}  // namespace

std::size_t page_size() noexcept {
    static const std::size_t cached = query_page_size();
    return cached;
}

std::size_t round_up_to_pages(std::size_t bytes) noexcept {
    const std::size_t page = page_size();
    return ((bytes + page - 1) / page) * page;
}

Expected<VirtualMemory, Error> VirtualMemory::reserve(std::size_t bytes) noexcept {
    if (bytes == 0) {
        return Unexpected{Error{ErrorCategory::invalid_argument, "zero-byte reservation"}};
    }
    const std::size_t rounded = round_up_to_pages(bytes);

    VirtualMemory result;
#if defined(_WIN32)
    void* base = ::VirtualAlloc(nullptr, rounded, MEM_RESERVE, PAGE_NOACCESS);
    if (base == nullptr) {
        return Unexpected{Error{ErrorCategory::out_of_memory, "VirtualAlloc reserve failed"}};
    }
#else
    // PROT_NONE so a touch of reserved-but-uncommitted memory faults
    // immediately, at the point of the mistake, rather than silently working
    // and failing later under memory pressure.
    void* base = ::mmap(nullptr, rounded, PROT_NONE,
                        MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE, -1, 0);
    if (base == MAP_FAILED) {
        return Unexpected{Error{ErrorCategory::out_of_memory, "mmap reserve failed"}};
    }
#endif
    result.base_ = static_cast<std::byte*>(base);
    result.reserved_bytes_ = rounded;
    result.committed_bytes_ = 0;
    return result;
}

Expected<VirtualMemory, Error> VirtualMemory::reserve_and_commit(std::size_t bytes) noexcept {
    PN_TRY_ASSIGN(auto memory, reserve(bytes));
    PN_TRY_VOID(memory.commit(bytes));
    return memory;
}

VirtualMemory::VirtualMemory(VirtualMemory&& other) noexcept
    : base_(std::exchange(other.base_, nullptr)),
      reserved_bytes_(std::exchange(other.reserved_bytes_, 0)),
      committed_bytes_(std::exchange(other.committed_bytes_, 0)) {}

VirtualMemory& VirtualMemory::operator=(VirtualMemory&& other) noexcept {
    if (this != &other) {
        PN_IGNORE_RESULT(release());
        base_ = std::exchange(other.base_, nullptr);
        reserved_bytes_ = std::exchange(other.reserved_bytes_, 0);
        committed_bytes_ = std::exchange(other.committed_bytes_, 0);
    }
    return *this;
}

VirtualMemory::~VirtualMemory() {
    PN_IGNORE_RESULT(release());
}

Expected<void, Error> VirtualMemory::commit(std::size_t bytes) noexcept {
    if (base_ == nullptr) {
        return Unexpected{Error{ErrorCategory::invalid_state, "no reservation"}};
    }
    if (bytes > reserved_bytes_) {
        return Unexpected{Error{ErrorCategory::out_of_range,
                                "commit exceeds the reservation"}};
    }
    const std::size_t rounded = round_up_to_pages(bytes);
    if (rounded <= committed_bytes_) {
        return {};  // already backed
    }

#if defined(_WIN32)
    if (::VirtualAlloc(base_, rounded, MEM_COMMIT, PAGE_READWRITE) == nullptr) {
        return Unexpected{Error{ErrorCategory::out_of_memory, "VirtualAlloc commit failed"}};
    }
#else
    if (::mprotect(base_, rounded, PROT_READ | PROT_WRITE) != 0) {
        return Unexpected{Error{ErrorCategory::out_of_memory, "mprotect commit failed"}};
    }
#endif
    committed_bytes_ = rounded;
    return {};
}

Expected<void, Error> VirtualMemory::decommit_beyond(std::size_t bytes) noexcept {
    if (base_ == nullptr) {
        return Unexpected{Error{ErrorCategory::invalid_state, "no reservation"}};
    }
    const std::size_t keep = round_up_to_pages(bytes);
    if (keep >= committed_bytes_) {
        return {};
    }
    const std::size_t release_bytes = committed_bytes_ - keep;

#if defined(_WIN32)
    if (::VirtualFree(base_ + keep, release_bytes, MEM_DECOMMIT) == 0) {
        return Unexpected{Error{ErrorCategory::internal, "VirtualFree decommit failed"}};
    }
#else
    // Back to PROT_NONE and drop the backing pages, matching the reserved state
    // so a stale pointer into the released region faults rather than reading
    // whatever the allocator hands out next.
    if (::mprotect(base_ + keep, release_bytes, PROT_NONE) != 0) {
        return Unexpected{Error{ErrorCategory::internal, "mprotect decommit failed"}};
    }
    static_cast<void>(::madvise(base_ + keep, release_bytes, MADV_DONTNEED));
#endif
    committed_bytes_ = keep;
    return {};
}

Expected<void, Error> VirtualMemory::release() noexcept {
    if (base_ == nullptr) {
        return {};
    }
#if defined(_WIN32)
    const bool ok = ::VirtualFree(base_, 0, MEM_RELEASE) != 0;
#else
    const bool ok = ::munmap(base_, reserved_bytes_) == 0;
#endif
    base_ = nullptr;
    reserved_bytes_ = 0;
    committed_bytes_ = 0;
    if (!ok) {
        return Unexpected{Error{ErrorCategory::internal, "releasing the reservation failed"}};
    }
    return {};
}

}  // namespace pn::platform
