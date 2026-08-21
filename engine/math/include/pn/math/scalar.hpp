// Pention Engine -  math/scalar.hpp
// Requirement: PN-WLD-001 (precision policy), docs/conventions.md (units)
// Decision:    ADR-0003

#ifndef PN_MATH_SCALAR_HPP
#define PN_MATH_SCALAR_HPP

#include <cmath>
#include <concepts>
#include <limits>

namespace pn::math {

/// Scalar types the engine's math is instantiated for.
///
/// Deliberately restricted to float and double. Long double varies by platform
/// and has no role here, and permitting it would silently widen the ABI of
/// every templated math type.
template <typename T>
concept Scalar = std::same_as<T, float> || std::same_as<T, double>;

template <Scalar T>
struct Constants {
    static constexpr T pi = static_cast<T>(3.14159265358979323846L);
    static constexpr T two_pi = static_cast<T>(6.28318530717958647692L);
    static constexpr T half_pi = static_cast<T>(1.57079632679489661923L);
    static constexpr T inv_pi = static_cast<T>(0.31830988618379067154L);
};

/// Degrees exist only in editor UI and human-facing serialized text; angles are
/// radians everywhere inside the engine (docs/conventions.md).
template <Scalar T>
[[nodiscard]] constexpr T radians_from_degrees(T degrees) noexcept {
    return degrees * (Constants<T>::pi / static_cast<T>(180));
}

template <Scalar T>
[[nodiscard]] constexpr T degrees_from_radians(T radians) noexcept {
    return radians * (static_cast<T>(180) / Constants<T>::pi);
}

/// Absolute-tolerance comparison.
///
/// The engine states its precision requirements in absolute units -  millimetres
/// of position error at a stated distance -  so comparisons are made in those
/// terms. A relative comparison would hide exactly the quantity being measured.
template <Scalar T>
[[nodiscard]] constexpr bool nearly_equal(T lhs, T rhs, T tolerance) noexcept {
    const T difference = lhs > rhs ? lhs - rhs : rhs - lhs;
    return difference <= tolerance;
}

template <Scalar T>
[[nodiscard]] constexpr T clamp(T value, T low, T high) noexcept {
    if (value < low) return low;
    if (value > high) return high;
    return value;
}

template <Scalar T>
[[nodiscard]] constexpr T lerp(T from, T to, T t) noexcept {
    return from + (to - from) * t;
}

/// Unit in the last place at the given magnitude.
///
/// Exists so the precision claims in docs/conventions.md are computed and
/// asserted rather than quoted from a table that could drift away from reality.
template <Scalar T>
[[nodiscard]] T ulp_at(T magnitude) noexcept {
    const T positive = magnitude < T{0} ? -magnitude : magnitude;
    if (positive == T{0}) {
        return std::numeric_limits<T>::denorm_min();
    }
    int exponent = 0;
    static_cast<void>(std::frexp(positive, &exponent));
    // frexp yields a mantissa in [0.5, 1), so the represented exponent is one
    // less than the value it returns.
    return std::ldexp(T{1}, exponent - 1 - (std::numeric_limits<T>::digits - 1));
}

}  // namespace pn::math

#endif  // PN_MATH_SCALAR_HPP
