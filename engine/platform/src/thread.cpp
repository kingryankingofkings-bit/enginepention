// Pention Engine - platform/thread.cpp
// Requirement: PN-PLT-012
// Decision:    ADR-0004

#include "pn/platform/thread.hpp"

#include <algorithm>
#include <array>
#include <cstring>
#include <thread>

#if defined(_WIN32)
#  define NOMINMAX
#  define WIN32_LEAN_AND_MEAN
#  include <windows.h>
#elif defined(__linux__)
#  include <pthread.h>
#endif

namespace pn::platform {

std::size_t hardware_thread_count() noexcept {
    const unsigned reported = std::thread::hardware_concurrency();
    return reported == 0 ? std::size_t{1} : static_cast<std::size_t>(reported);
}

void set_current_thread_name(std::string_view name) noexcept {
#if defined(_WIN32)
    // Windows wants UTF-16. Names are ASCII engine-side, so the widening is a
    // byte-wise copy rather than a real conversion.
    std::array<wchar_t, 64> wide{};
    const std::size_t count = std::min(name.size(), wide.size() - 1);
    for (std::size_t i = 0; i < count; ++i) {
        wide[i] = static_cast<wchar_t>(name[i]);
    }
    static_cast<void>(::SetThreadDescription(::GetCurrentThread(), wide.data()));
#elif defined(__linux__)
    // pthread_setname_np caps the name at 16 bytes including the terminator and
    // fails outright on a longer one, so truncate rather than lose the name.
    std::array<char, 16> buffer{};
    const std::size_t count = std::min(name.size(), buffer.size() - 1);
    std::memcpy(buffer.data(), name.data(), count);
    buffer[count] = '\0';
    static_cast<void>(::pthread_setname_np(::pthread_self(), buffer.data()));
#else
    static_cast<void>(name);
#endif
}

void yield_current_thread() noexcept {
    std::this_thread::yield();
}

}  // namespace pn::platform
