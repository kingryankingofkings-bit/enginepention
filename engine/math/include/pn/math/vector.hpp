// Pention Engine -  math/vector.hpp
// Requirement: PN-WLD-001, PN-WLD-002
// Decision:    ADR-0003 (right-handed, Y-up, -Z forward; f64 world, f32 render)

#ifndef PN_MATH_VECTOR_HPP
#define PN_MATH_VECTOR_HPP

#include "pn/math/scalar.hpp"

#include <cmath>
#include <cstddef>

namespace pn::math {

template <Scalar T>
struct Vec2T {
    T x{};
    T y{};

    constexpr Vec2T() = default;
    constexpr Vec2T(T x_, T y_) noexcept : x(x_), y(y_) {}

    [[nodiscard]] constexpr T operator[](std::size_t i) const noexcept {
        return i == 0 ? x : y;
    }

    friend constexpr bool operator==(const Vec2T&, const Vec2T&) = default;
};

template <Scalar T>
struct Vec3T {
    T x{};
    T y{};
    T z{};

    constexpr Vec3T() = default;
    constexpr Vec3T(T x_, T y_, T z_) noexcept : x(x_), y(y_), z(z_) {}

    [[nodiscard]] constexpr T operator[](std::size_t i) const noexcept {
        return i == 0 ? x : (i == 1 ? y : z);
    }

    friend constexpr bool operator==(const Vec3T&, const Vec3T&) = default;

    /// Narrowing conversion to another scalar width.
    ///
    /// Explicit and named rather than an implicit conversion, because the
    /// f64-world to f32-render narrowing is exactly the operation ADR-0003
    /// requires be visible at every site it happens.
    template <Scalar U>
    [[nodiscard]] constexpr Vec3T<U> cast() const noexcept {
        return Vec3T<U>{static_cast<U>(x), static_cast<U>(y), static_cast<U>(z)};
    }
};

template <Scalar T>
struct Vec4T {
    T x{};
    T y{};
    T z{};
    T w{};

    constexpr Vec4T() = default;
    constexpr Vec4T(T x_, T y_, T z_, T w_) noexcept : x(x_), y(y_), z(z_), w(w_) {}
    constexpr Vec4T(const Vec3T<T>& v, T w_) noexcept : x(v.x), y(v.y), z(v.z), w(w_) {}

    [[nodiscard]] constexpr T operator[](std::size_t i) const noexcept {
        return i == 0 ? x : (i == 1 ? y : (i == 2 ? z : w));
    }

    [[nodiscard]] constexpr Vec3T<T> xyz() const noexcept { return Vec3T<T>{x, y, z}; }

    friend constexpr bool operator==(const Vec4T&, const Vec4T&) = default;
};

// --- Vec3 arithmetic -------------------------------------------------------

template <Scalar T>
[[nodiscard]] constexpr Vec3T<T> operator+(const Vec3T<T>& a, const Vec3T<T>& b) noexcept {
    return Vec3T<T>{a.x + b.x, a.y + b.y, a.z + b.z};
}

template <Scalar T>
[[nodiscard]] constexpr Vec3T<T> operator-(const Vec3T<T>& a, const Vec3T<T>& b) noexcept {
    return Vec3T<T>{a.x - b.x, a.y - b.y, a.z - b.z};
}

template <Scalar T>
[[nodiscard]] constexpr Vec3T<T> operator-(const Vec3T<T>& v) noexcept {
    return Vec3T<T>{-v.x, -v.y, -v.z};
}

template <Scalar T>
[[nodiscard]] constexpr Vec3T<T> operator*(const Vec3T<T>& v, T s) noexcept {
    return Vec3T<T>{v.x * s, v.y * s, v.z * s};
}

template <Scalar T>
[[nodiscard]] constexpr Vec3T<T> operator*(T s, const Vec3T<T>& v) noexcept {
    return v * s;
}

template <Scalar T>
[[nodiscard]] constexpr Vec3T<T> operator/(const Vec3T<T>& v, T s) noexcept {
    return Vec3T<T>{v.x / s, v.y / s, v.z / s};
}

template <Scalar T>
[[nodiscard]] constexpr T dot(const Vec3T<T>& a, const Vec3T<T>& b) noexcept {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

/// Right-handed cross product.
///
/// docs/conventions.md fixes the basis as right-handed Y-up with -Z forward,
/// which requires cross(+X, +Y) == +Z. A test asserts precisely that, because
/// getting it backwards produces mirrored normals and inverted culling that are
/// individually plausible and collectively baffling.
template <Scalar T>
[[nodiscard]] constexpr Vec3T<T> cross(const Vec3T<T>& a, const Vec3T<T>& b) noexcept {
    return Vec3T<T>{a.y * b.z - a.z * b.y,
                    a.z * b.x - a.x * b.z,
                    a.x * b.y - a.y * b.x};
}

template <Scalar T>
[[nodiscard]] constexpr T length_squared(const Vec3T<T>& v) noexcept {
    return dot(v, v);
}

template <Scalar T>
[[nodiscard]] T length(const Vec3T<T>& v) noexcept {
    return std::sqrt(length_squared(v));
}

template <Scalar T>
[[nodiscard]] T distance(const Vec3T<T>& a, const Vec3T<T>& b) noexcept {
    return length(b - a);
}

/// Normalizes, returning the zero vector for a zero-length input.
///
/// Returning zero rather than producing NaN keeps a degenerate input from
/// silently poisoning every downstream computation. Callers that require a
/// genuine direction check the length themselves.
template <Scalar T>
[[nodiscard]] Vec3T<T> normalize(const Vec3T<T>& v) noexcept {
    const T len_squared = length_squared(v);
    if (len_squared <= T{0}) {
        return Vec3T<T>{};
    }
    return v / std::sqrt(len_squared);
}

template <Scalar T>
[[nodiscard]] constexpr Vec3T<T> lerp(const Vec3T<T>& a, const Vec3T<T>& b, T t) noexcept {
    return a + (b - a) * t;
}

// --- Vec4 arithmetic -------------------------------------------------------

template <Scalar T>
[[nodiscard]] constexpr Vec4T<T> operator+(const Vec4T<T>& a, const Vec4T<T>& b) noexcept {
    return Vec4T<T>{a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w};
}

template <Scalar T>
[[nodiscard]] constexpr Vec4T<T> operator*(const Vec4T<T>& v, T s) noexcept {
    return Vec4T<T>{v.x * s, v.y * s, v.z * s, v.w * s};
}

template <Scalar T>
[[nodiscard]] constexpr T dot(const Vec4T<T>& a, const Vec4T<T>& b) noexcept {
    return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;
}

// --- Canonical basis, in the engine's convention ---------------------------

template <Scalar T>
struct Axis {
    static constexpr Vec3T<T> right{T{1}, T{0}, T{0}};   ///< +X
    static constexpr Vec3T<T> up{T{0}, T{1}, T{0}};      ///< +Y
    static constexpr Vec3T<T> forward{T{0}, T{0}, T{-1}}; ///< -Z, into the screen
};

// --- Aliases ---------------------------------------------------------------
//
// Render space is f32; world space is f64 (ADR-0003).

using Vec2 = Vec2T<float>;
using Vec3 = Vec3T<float>;
using Vec4 = Vec4T<float>;

using DVec2 = Vec2T<double>;
using DVec3 = Vec3T<double>;
using DVec4 = Vec4T<double>;

}  // namespace pn::math

#endif  // PN_MATH_VECTOR_HPP
