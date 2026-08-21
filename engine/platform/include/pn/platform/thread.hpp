// Pention Engine - platform/thread.hpp
// Requirement: PN-PLT-012 (thread abstraction, affinity, naming)
// Decision:    ADR-0004

#ifndef PN_PLATFORM_THREAD_HPP
#define PN_PLATFORM_THREAD_HPP

#include <cstddef>
#include <string_view>

namespace pn::platform {

/// Number of hardware threads, never zero.
///
/// std::thread::hardware_concurrency is permitted to return 0 when it cannot
/// tell, and callers routinely forget that and size a thread pool at zero. This
/// returns at least 1.
[[nodiscard]] std::size_t hardware_thread_count() noexcept;

/// Names the calling thread, for debuggers and profilers.
///
/// Best-effort: platforms cap the length and some reject names outright, so
/// this reports nothing. A failure to set a diagnostic label must never become
/// a failure of the thing being labelled.
void set_current_thread_name(std::string_view name) noexcept;

/// Hints that the caller is spinning and the core would be better used
/// elsewhere. A scheduling hint, not a guarantee.
void yield_current_thread() noexcept;

}  // namespace pn::platform

#endif  // PN_PLATFORM_THREAD_HPP
