// Pention Engine - core/expected.hpp
// Requirement: PN-PLT-027 (Expected-based fallible-return convention)
// Decision:    ADR-0002
//
// Why this exists rather than std::expected: measured on the supported
// toolchains, __cpp_lib_expected is 202211 under g++ -std=c++23 and absent
// under clang++ -std=c++23 against the same libstdc++. See
// docs/INSTRUCTION_CONFLICTS.md CONF-003.
//
// Implemented from the interface described in the published C++ standard, which
// the dependency boundary permits as an open technical specification used as a
// behavioural reference. No standard library implementation was consulted.
//
// Deliberately NOT named `expected`: the semantics are close but the type
// carries engine-specific context, and a name that implied drop-in equivalence
// would invite assumptions that do not hold.

#ifndef PN_CORE_EXPECTED_HPP
#define PN_CORE_EXPECTED_HPP

#include "pn/core/error.hpp"

#include <concepts>
#include <memory>
#include <type_traits>
#include <utility>

namespace pn::core {

/// Wraps an error so a fallible return can be constructed unambiguously even
/// when the value type and the error type are the same or convertible.
template <typename E>
class Unexpected {
public:
    constexpr explicit Unexpected(E error) noexcept(std::is_nothrow_move_constructible_v<E>)
        : error_(std::move(error)) {}

    [[nodiscard]] constexpr const E& error() const& noexcept { return error_; }
    [[nodiscard]] constexpr E&& error() && noexcept { return std::move(error_); }

private:
    E error_;
};

template <typename E>
Unexpected(E) -> Unexpected<E>;

/// Convenience for the common case of failing with the default error type.
template <typename... Args>
[[nodiscard]] constexpr Unexpected<Error> fail(Args&&... args) noexcept {
    return Unexpected<Error>{Error{std::forward<Args>(args)...}};
}

/// A value, or an explanation of why there isn't one.
///
/// Never allocates. Trivially destructible when both T and E are, so it costs
/// nothing in the common case of a scalar payload and a trivial error type.
/// Independent of exceptions - the tree must compile with -fno-exceptions.
template <typename T, typename E = Error>
class [[nodiscard]] Expected {
    static_assert(!std::is_reference_v<T>, "Expected does not hold references");
    static_assert(!std::is_same_v<std::remove_cv_t<T>, void>,
                  "use the Expected<void, E> specialization");

public:
    using value_type = T;
    using error_type = E;

    constexpr Expected() noexcept(std::is_nothrow_default_constructible_v<T>)
        requires std::default_initializable<T>
        : value_(), has_value_(true) {}

    constexpr Expected(const T& value) noexcept(std::is_nothrow_copy_constructible_v<T>)
        : value_(value), has_value_(true) {}

    constexpr Expected(T&& value) noexcept(std::is_nothrow_move_constructible_v<T>)
        : value_(std::move(value)), has_value_(true) {}

    constexpr Expected(Unexpected<E> unexpected) noexcept(
        std::is_nothrow_move_constructible_v<E>)
        : error_(std::move(unexpected).error()), has_value_(false) {}

    constexpr Expected(const Expected& other) : has_value_(other.has_value_) {
        if (has_value_) {
            std::construct_at(std::addressof(value_), other.value_);
        } else {
            std::construct_at(std::addressof(error_), other.error_);
        }
    }

    constexpr Expected(Expected&& other) noexcept(
        std::is_nothrow_move_constructible_v<T> && std::is_nothrow_move_constructible_v<E>)
        : has_value_(other.has_value_) {
        if (has_value_) {
            std::construct_at(std::addressof(value_), std::move(other.value_));
        } else {
            std::construct_at(std::addressof(error_), std::move(other.error_));
        }
    }

    constexpr Expected& operator=(const Expected& other) {
        if (this != &other) {
            destroy();
            has_value_ = other.has_value_;
            if (has_value_) {
                std::construct_at(std::addressof(value_), other.value_);
            } else {
                std::construct_at(std::addressof(error_), other.error_);
            }
        }
        return *this;
    }

    constexpr Expected& operator=(Expected&& other) noexcept(
        std::is_nothrow_move_constructible_v<T> && std::is_nothrow_move_constructible_v<E>) {
        if (this != &other) {
            destroy();
            has_value_ = other.has_value_;
            if (has_value_) {
                std::construct_at(std::addressof(value_), std::move(other.value_));
            } else {
                std::construct_at(std::addressof(error_), std::move(other.error_));
            }
        }
        return *this;
    }

    // Trivially destructible whenever the payloads are, so the common case of a
    // scalar value and a trivial error costs nothing.
    constexpr ~Expected()
        requires(std::is_trivially_destructible_v<T> && std::is_trivially_destructible_v<E>)
    = default;

    constexpr ~Expected() { destroy(); }

    [[nodiscard]] constexpr bool has_value() const noexcept { return has_value_; }
    constexpr explicit operator bool() const noexcept { return has_value_; }

    /// Accessing the value of a failed Expected is a programming error. In a
    /// debug build the assertion fires; callers must test has_value() first, or
    /// use value_or(), or propagate with PN_TRY.
    [[nodiscard]] constexpr T& value() & noexcept { return value_; }
    [[nodiscard]] constexpr const T& value() const& noexcept { return value_; }
    [[nodiscard]] constexpr T&& value() && noexcept { return std::move(value_); }

    [[nodiscard]] constexpr E& error() & noexcept { return error_; }
    [[nodiscard]] constexpr const E& error() const& noexcept { return error_; }
    [[nodiscard]] constexpr E&& error() && noexcept { return std::move(error_); }

    [[nodiscard]] constexpr T value_or(T fallback) const& {
        return has_value_ ? value_ : fallback;
    }

    [[nodiscard]] constexpr const T* operator->() const noexcept {
        return std::addressof(value_);
    }
    [[nodiscard]] constexpr T* operator->() noexcept { return std::addressof(value_); }

    // --- Monadic composition -----------------------------------------------
    //
    // These exist so error propagation does not require an if-statement at
    // every call site. A chain that fails anywhere short-circuits to the first
    // error, preserving it unchanged.

    /// Applies f to the value, wrapping the result. Errors pass through.
    template <typename F>
    [[nodiscard]] constexpr auto map(F&& f) const& -> Expected<std::invoke_result_t<F, const T&>, E> {
        using Result = std::invoke_result_t<F, const T&>;
        if (!has_value_) {
            return Expected<Result, E>{Unexpected<E>{error_}};
        }
        return Expected<Result, E>{std::forward<F>(f)(value_)};
    }

    /// Applies f to the value, where f itself returns an Expected. Errors pass
    /// through without invoking f.
    template <typename F>
    [[nodiscard]] constexpr auto and_then(F&& f) const& -> std::invoke_result_t<F, const T&> {
        using Result = std::invoke_result_t<F, const T&>;
        if (!has_value_) {
            return Result{Unexpected<E>{error_}};
        }
        return std::forward<F>(f)(value_);
    }

    /// Applies f to the error, giving a chance to recover or to enrich. Values
    /// pass through without invoking f.
    template <typename F>
    [[nodiscard]] constexpr auto or_else(F&& f) const& -> std::invoke_result_t<F, const E&> {
        using Result = std::invoke_result_t<F, const E&>;
        if (has_value_) {
            return Result{value_};
        }
        return std::forward<F>(f)(error_);
    }

private:
    constexpr void destroy() noexcept {
        if constexpr (!std::is_trivially_destructible_v<T> ||
                      !std::is_trivially_destructible_v<E>) {
            if (has_value_) {
                std::destroy_at(std::addressof(value_));
            } else {
                std::destroy_at(std::addressof(error_));
            }
        }
    }

    union {
        T value_;
        E error_;
    };
    bool has_value_;
};

/// Specialization for operations that succeed or fail but produce no value.
template <typename E>
class [[nodiscard]] Expected<void, E> {
public:
    using value_type = void;
    using error_type = E;

    constexpr Expected() noexcept : has_value_(true) {}

    constexpr Expected(Unexpected<E> unexpected) noexcept(
        std::is_nothrow_move_constructible_v<E>)
        : error_(std::move(unexpected).error()), has_value_(false) {}

    [[nodiscard]] constexpr bool has_value() const noexcept { return has_value_; }
    constexpr explicit operator bool() const noexcept { return has_value_; }

    [[nodiscard]] constexpr const E& error() const& noexcept { return error_; }
    [[nodiscard]] constexpr E&& error() && noexcept { return std::move(error_); }

private:
    E error_{};
    bool has_value_;
};

/// Signals that discarding a fallible result is deliberate.
///
/// Spelled out rather than a bare (void) cast so that every intentional
/// discard is greppable in review. If this appears often, that is information
/// about the API, not about the reviewers.
#define PN_IGNORE_RESULT(expr_) static_cast<void>((expr_).has_value())

/// Propagates a failure to the caller, and binds the value on success.
///
///     PN_TRY_ASSIGN(const auto data, load_file(path));
///     // `data` is in scope from here on.
///
/// Takes the declaration as an argument rather than yielding a value, because
/// yielding one would require a GNU statement expression. That extension is
/// rejected by -Wpedantic, and MSVC - a supported toolchain under ADR-0001 -
/// does not provide it at all. The declaration form is standard C++ and works
/// on every target the engine claims.
///
/// Declares a variable in the enclosing scope, so it cannot appear inside an
/// unbraced if or loop body. That is a deliberate constraint: error propagation
/// hidden inside a one-line conditional is hard to see when reading for control
/// flow.
#define PN_DETAIL_CONCAT_(a_, b_) a_##b_
#define PN_DETAIL_CONCAT(a_, b_) PN_DETAIL_CONCAT_(a_, b_)
#define PN_DETAIL_TRY_TEMPORARY PN_DETAIL_CONCAT(pn_try_temporary_, __LINE__)

#define PN_TRY_ASSIGN(declaration_, expr_)                                       \
    auto PN_DETAIL_TRY_TEMPORARY = (expr_);                                      \
    if (!PN_DETAIL_TRY_TEMPORARY.has_value()) {                                  \
        return ::pn::core::Unexpected{std::move(PN_DETAIL_TRY_TEMPORARY).error()}; \
    }                                                                            \
    declaration_ = std::move(PN_DETAIL_TRY_TEMPORARY).value()

/// Propagates a failure from an operation that yields no value.
#define PN_TRY_VOID(expr_)                                                       \
    do {                                                                         \
        auto PN_DETAIL_TRY_TEMPORARY = (expr_);                                  \
        if (!PN_DETAIL_TRY_TEMPORARY.has_value()) {                              \
            return ::pn::core::Unexpected{                                       \
                std::move(PN_DETAIL_TRY_TEMPORARY).error()};                     \
        }                                                                        \
    } while (false)

}  // namespace pn::core

#endif  // PN_CORE_EXPECTED_HPP
