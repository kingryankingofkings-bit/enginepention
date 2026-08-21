// Pention Engine - core/assert.hpp
// Requirement: PN-PLT-009 (bounds-checked in debug), PN-PLT-016
// Decision:    ADR-0002 (error handling), ADR-0004

#ifndef PN_CORE_ASSERT_HPP
#define PN_CORE_ASSERT_HPP

#include <cstdio>
#include <cstdlib>
#include <source_location>

namespace pn::core {

/// What a failed assertion reports.
struct AssertionFailure {
    /// The expression as written, from the stringised macro argument.
    const char* expression = nullptr;
    /// The optional explanation, or an empty string.
    const char* message = nullptr;
    std::source_location where{};
};

/// A handler for a failed assertion.
///
/// Returning from a handler lets execution continue. That is not a loophole for
/// ignoring assertions - the default handler terminates, and it is what ships -
/// but two callers genuinely need to observe a failure without dying: a test
/// that checks an assertion fires at all, and a crash reporter that wants to
/// record state before the process goes down.
using AssertionHandler = void (*)(const AssertionFailure&);

/// Terminates after reporting, which is what an assertion means by default.
///
/// Writes to `stderr` with `fwrite` rather than a stream, so the message
/// survives a corrupted heap: this is often the last output before a crash, and
/// a reporting path that allocates can fail exactly when it is most needed.
///
/// Not `throw`: ADR-0002 makes the engine exception-independent, and half the
/// build matrix compiles with `-fno-exceptions`. Not an error return either - an
/// assertion marks a state the program has no defined behaviour for, and a
/// caller cannot handle what the callee has already established is impossible.
[[noreturn]] inline void terminating_assertion_handler(const AssertionFailure& failure) {
    std::fprintf(stderr, "\nAssertion failed: %s\n",
                 failure.expression != nullptr ? failure.expression : "(none)");
    if (failure.message != nullptr && failure.message[0] != '\0') {
        std::fprintf(stderr, "  %s\n", failure.message);
    }
    std::fprintf(stderr, "  at %s:%u in %s\n", failure.where.file_name(),
                 failure.where.line(), failure.where.function_name());
    std::fflush(stderr);
    std::abort();
}

namespace detail {

/// The installed handler.
///
/// A plain function pointer rather than a `std::function`: this is read on a
/// path that runs when things are already going wrong, and an indirect call
/// through an allocation is the wrong thing to depend on there.
inline AssertionHandler& assertion_handler() noexcept {
    static AssertionHandler handler = &terminating_assertion_handler;
    return handler;
}

}  // namespace detail

/// Replaces the handler and returns the previous one, so a caller can restore
/// it. A test that leaves a capturing handler installed disarms every assertion
/// in every test that runs after it.
inline AssertionHandler set_assertion_handler(AssertionHandler handler) noexcept {
    AssertionHandler previous = detail::assertion_handler();
    detail::assertion_handler() = handler != nullptr ? handler : &terminating_assertion_handler;
    return previous;
}

/// Reports a failed assertion through the installed handler.
inline void assertion_failed(
    const char* expression,
    const char* message,
    std::source_location where = std::source_location::current()) {
    detail::assertion_handler()(AssertionFailure{expression, message, where});
}

}  // namespace pn::core

#endif  // PN_CORE_ASSERT_HPP

// ---------------------------------------------------------------------------
// The macros live **outside** the include guard, on purpose.
//
// Including this header again after changing PN_ENABLE_ASSERTS re-derives them,
// which lets one translation unit compile some code with assertions and some
// without. That is not a curiosity: it is how the disabled form gets tested at
// all. A test that checks "a disabled assertion does not evaluate its
// expression" has to have both forms in the same binary, and writing a copy of
// the disabled macro for the test would check the copy rather than the macro.
//
// The declarations above are still guarded normally; only the macros repeat.
// ---------------------------------------------------------------------------

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

#undef PN_ASSERT
#undef PN_ASSERT_MSG

#if PN_ENABLE_ASSERTS
/// Reports through the installed handler if `expression_` is false.
#  define PN_ASSERT(expression_)                                                    \
      ((expression_) ? void(0)                                                      \
                     : ::pn::core::assertion_failed(#expression_, nullptr))
/// Reports through the installed handler if `expression_` is false, with a
/// message.
#  define PN_ASSERT_MSG(expression_, message_)                                      \
      ((expression_) ? void(0)                                                      \
                     : ::pn::core::assertion_failed(#expression_, (message_)))
#else
// The expression is still parsed and type-checked, but not evaluated: an
// assertion that stops compiling in Release is an assertion that will be found
// broken by the release build rather than by the developer who wrote it.
//
// The `? 1 : 0` rather than a `bool(...)` cast is not a style choice. A cast to
// bool on an expression that is already bool is a useless cast, which this
// project builds with as an error. The first version used the cast and was
// latent for a commit: every user was a member of a class template, where the
// expression is dependent and the diagnostic waits for instantiation. The first
// non-template caller surfaced it immediately. A ternary asks for the same
// contextual conversion and casts nothing.
#  define PN_ASSERT(expression_) (void(sizeof((expression_) ? 1 : 0)))
#  define PN_ASSERT_MSG(expression_, message_) \
      (void(sizeof((expression_) ? 1 : 0)), void(sizeof(message_)))
#endif
