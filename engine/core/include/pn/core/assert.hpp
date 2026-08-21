// Pention Engine - core/assert.hpp
// Requirement: PN-PLT-009 (bounds-checked in debug), PN-PLT-016
// Decision:    ADR-0002 (error handling), ADR-0004

#ifndef PN_CORE_ASSERT_HPP
#define PN_CORE_ASSERT_HPP

#include <cstdio>
#include <cstdlib>
#include <source_location>

namespace pn::core {

/// Reports a failed assertion and terminates.
///
/// Not `throw`: ADR-0002 makes the engine exception-independent, and half the
/// build matrix compiles with `-fno-exceptions`. Not an error return either -
/// an assertion marks a state the program has no defined behaviour for, and a
/// caller cannot handle what the callee has already established is impossible.
///
/// Writes to `stderr` with `fwrite` rather than a stream, so the message
/// survives a corrupted heap: this is often the last output before a crash, and
/// a reporting path that allocates can fail exactly when it is most needed.
[[noreturn]] inline void assertion_failed(
    const char* expression,
    const char* message,
    std::source_location where = std::source_location::current()) noexcept {
    std::fprintf(stderr, "\nAssertion failed: %s\n", expression);
    if (message != nullptr && message[0] != '\0') {
        std::fprintf(stderr, "  %s\n", message);
    }
    std::fprintf(stderr, "  at %s:%u in %s\n", where.file_name(), where.line(),
                 where.function_name());
    std::fflush(stderr);
    std::abort();
}

}  // namespace pn::core

/// Whether assertions are compiled in.
///
/// Tied to `NDEBUG` by default so a Release build pays nothing, and overridable
/// so a Release build that wants them can have them - a shipping build with
/// assertions is a legitimate choice for a beta, and forcing a rebuild of the
/// whole engine to make it is not.
#ifndef PN_ENABLE_ASSERTS
#  ifdef NDEBUG
#    define PN_ENABLE_ASSERTS 0
#  else
#    define PN_ENABLE_ASSERTS 1
#  endif
#endif

#if PN_ENABLE_ASSERTS
/// Aborts if `expression_` is false.
#  define PN_ASSERT(expression_)                                                    \
      ((expression_) ? void(0)                                                      \
                     : ::pn::core::assertion_failed(#expression_, nullptr))
/// Aborts if `expression_` is false, reporting `message_`.
#  define PN_ASSERT_MSG(expression_, message_)                                      \
      ((expression_) ? void(0)                                                      \
                     : ::pn::core::assertion_failed(#expression_, (message_)))
#else
// The expression is still parsed and type-checked, but not evaluated: an
// assertion that stops compiling in Release is an assertion that will be found
// broken by the release build rather than by the developer who wrote it.
#  define PN_ASSERT(expression_) (void(sizeof(bool((expression_)))))
#  define PN_ASSERT_MSG(expression_, message_) \
      (void(sizeof(bool((expression_)))), void(sizeof(message_)))
#endif

#endif  // PN_CORE_ASSERT_HPP
