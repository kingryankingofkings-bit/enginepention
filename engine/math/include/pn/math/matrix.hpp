// Pention Engine -  math/matrix.hpp
// Requirement: PN-WLD-002, docs/conventions.md (column-major storage)
// Decision:    ADR-0003

#ifndef PN_MATH_MATRIX_HPP
#define PN_MATH_MATRIX_HPP

#include "pn/math/scalar.hpp"
#include "pn/math/vector.hpp"

#include <array>
#include <cstddef>

namespace pn::math {

/// Column-major 4x4 matrix. Element (row, col) lives at `data[col][row]`.
///
/// Column-major matches the shader-language convention, so a matrix can be
/// copied into a uniform buffer with no transpose. A transpose hidden in an
/// upload path only misbehaves under non-uniform scale or a non-symmetric
/// projection, which is why it survives months of testing before being found.
template <Scalar T>
struct Mat4T {
    /// data[column][row]
    std::array<std::array<T, 4>, 4> data{};

    constexpr Mat4T() = default;

    [[nodiscard]] constexpr T& at(std::size_t row, std::size_t col) noexcept {
        return data[col][row];
    }
    [[nodiscard]] constexpr const T& at(std::size_t row, std::size_t col) const noexcept {
        return data[col][row];
    }

    [[nodiscard]] constexpr Vec4T<T> column(std::size_t col) const noexcept {
        return Vec4T<T>{data[col][0], data[col][1], data[col][2], data[col][3]};
    }

    [[nodiscard]] constexpr Vec4T<T> row(std::size_t r) const noexcept {
        return Vec4T<T>{data[0][r], data[1][r], data[2][r], data[3][r]};
    }

    [[nodiscard]] static constexpr Mat4T identity() noexcept {
        Mat4T m;
        m.data[0][0] = T{1};
        m.data[1][1] = T{1};
        m.data[2][2] = T{1};
        m.data[3][3] = T{1};
        return m;
    }

    [[nodiscard]] static constexpr Mat4T zero() noexcept { return Mat4T{}; }

    friend constexpr bool operator==(const Mat4T&, const Mat4T&) = default;

    template <Scalar U>
    [[nodiscard]] constexpr Mat4T<U> cast() const noexcept {
        Mat4T<U> out;
        for (std::size_t col = 0; col < 4; ++col) {
            for (std::size_t r = 0; r < 4; ++r) {
                out.data[col][r] = static_cast<U>(data[col][r]);
            }
        }
        return out;
    }
};

template <Scalar T>
[[nodiscard]] constexpr Mat4T<T> operator*(const Mat4T<T>& a, const Mat4T<T>& b) noexcept {
    Mat4T<T> out;
    for (std::size_t col = 0; col < 4; ++col) {
        for (std::size_t r = 0; r < 4; ++r) {
            T sum{};
            for (std::size_t k = 0; k < 4; ++k) {
                sum += a.data[k][r] * b.data[col][k];
            }
            out.data[col][r] = sum;
        }
    }
    return out;
}

template <Scalar T>
[[nodiscard]] constexpr Vec4T<T> operator*(const Mat4T<T>& m, const Vec4T<T>& v) noexcept {
    return Vec4T<T>{
        m.data[0][0] * v.x + m.data[1][0] * v.y + m.data[2][0] * v.z + m.data[3][0] * v.w,
        m.data[0][1] * v.x + m.data[1][1] * v.y + m.data[2][1] * v.z + m.data[3][1] * v.w,
        m.data[0][2] * v.x + m.data[1][2] * v.y + m.data[2][2] * v.z + m.data[3][2] * v.w,
        m.data[0][3] * v.x + m.data[1][3] * v.y + m.data[2][3] * v.z + m.data[3][3] * v.w};
}

template <Scalar T>
[[nodiscard]] constexpr Mat4T<T> transpose(const Mat4T<T>& m) noexcept {
    Mat4T<T> out;
    for (std::size_t col = 0; col < 4; ++col) {
        for (std::size_t r = 0; r < 4; ++r) {
            out.data[r][col] = m.data[col][r];
        }
    }
    return out;
}

template <Scalar T>
[[nodiscard]] constexpr Mat4T<T> translation(const Vec3T<T>& t) noexcept {
    Mat4T<T> m = Mat4T<T>::identity();
    m.data[3][0] = t.x;
    m.data[3][1] = t.y;
    m.data[3][2] = t.z;
    return m;
}

template <Scalar T>
[[nodiscard]] constexpr Mat4T<T> scaling(const Vec3T<T>& s) noexcept {
    Mat4T<T> m;
    m.data[0][0] = s.x;
    m.data[1][1] = s.y;
    m.data[2][2] = s.z;
    m.data[3][3] = T{1};
    return m;
}

/// Transforms a point (implicit w = 1), performing the perspective divide only
/// when the caller asks for it via transform_projected.
template <Scalar T>
[[nodiscard]] constexpr Vec3T<T> transform_point(const Mat4T<T>& m, const Vec3T<T>& p) noexcept {
    return Vec3T<T>{
        m.data[0][0] * p.x + m.data[1][0] * p.y + m.data[2][0] * p.z + m.data[3][0],
        m.data[0][1] * p.x + m.data[1][1] * p.y + m.data[2][1] * p.z + m.data[3][1],
        m.data[0][2] * p.x + m.data[1][2] * p.y + m.data[2][2] * p.z + m.data[3][2]};
}

/// Transforms a direction (implicit w = 0), so translation does not apply.
template <Scalar T>
[[nodiscard]] constexpr Vec3T<T> transform_direction(const Mat4T<T>& m, const Vec3T<T>& d) noexcept {
    return Vec3T<T>{
        m.data[0][0] * d.x + m.data[1][0] * d.y + m.data[2][0] * d.z,
        m.data[0][1] * d.x + m.data[1][1] * d.y + m.data[2][1] * d.z,
        m.data[0][2] * d.x + m.data[1][2] * d.y + m.data[2][2] * d.z};
}

/// Inverse of a rigid transform (rotation plus translation, no scale).
///
/// A dedicated entry point because it is the common case for view matrices and
/// is exact and cheap, where a general inverse is neither.
template <Scalar T>
[[nodiscard]] constexpr Mat4T<T> inverse_rigid(const Mat4T<T>& m) noexcept {
    Mat4T<T> out = Mat4T<T>::identity();
    // Transpose the rotation block.
    for (std::size_t col = 0; col < 3; ++col) {
        for (std::size_t r = 0; r < 3; ++r) {
            out.data[r][col] = m.data[col][r];
        }
    }
    // Rotated negative translation.
    const Vec3T<T> t{m.data[3][0], m.data[3][1], m.data[3][2]};
    out.data[3][0] = -(out.data[0][0] * t.x + out.data[1][0] * t.y + out.data[2][0] * t.z);
    out.data[3][1] = -(out.data[0][1] * t.x + out.data[1][1] * t.y + out.data[2][1] * t.z);
    out.data[3][2] = -(out.data[0][2] * t.x + out.data[1][2] * t.y + out.data[2][2] * t.z);
    return out;
}

using Mat4 = Mat4T<float>;
using DMat4 = Mat4T<double>;

}  // namespace pn::math

#endif  // PN_MATH_MATRIX_HPP
