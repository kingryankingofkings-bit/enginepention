// Pention Engine -  math/world.hpp
// Requirement: PN-WLD-001, PN-WLD-002 (f64 world space, camera-relative f32 render space)
// Decision:    ADR-0003

#ifndef PN_MATH_WORLD_HPP
#define PN_MATH_WORLD_HPP

#include "pn/math/matrix.hpp"
#include "pn/math/scalar.hpp"
#include "pn/math/vector.hpp"

namespace pn::math {

/// A position in world space. Always f64.
///
/// A distinct type rather than an alias for DVec3, so that a world position and
/// a render-space position cannot be mixed by accident. ADR-0003 makes that
/// distinction load-bearing: mixing them is not a style issue but a precision bug
/// that only appears tens of kilometres from the origin, long after the code
/// that caused it was written.
struct WorldPosition {
    DVec3 value{};

    constexpr WorldPosition() = default;
    constexpr explicit WorldPosition(const DVec3& v) noexcept : value(v) {}
    constexpr WorldPosition(double x, double y, double z) noexcept : value{x, y, z} {}

    friend constexpr bool operator==(const WorldPosition&, const WorldPosition&) = default;
};

/// A position in camera-relative render space. Always f32.
///
/// Small by construction, because the camera has been subtracted out, so f32 is
/// dense here even when the world position was hundreds of kilometres from the
/// origin.
struct RenderPosition {
    Vec3 value{};

    constexpr RenderPosition() = default;
    constexpr explicit RenderPosition(const Vec3& v) noexcept : value(v) {}

    friend constexpr bool operator==(const RenderPosition&, const RenderPosition&) = default;
};

[[nodiscard]] constexpr DVec3 operator-(const WorldPosition& a, const WorldPosition& b) noexcept {
    return a.value - b.value;
}

[[nodiscard]] constexpr WorldPosition operator+(const WorldPosition& p, const DVec3& d) noexcept {
    return WorldPosition{p.value + d};
}

/// Converts a world position into camera-relative render space.
///
/// The subtraction happens in f64 and only the (small) result is narrowed to
/// f32. Narrowing first and subtracting second would discard exactly the
/// precision this function exists to preserve.
[[nodiscard]] constexpr RenderPosition to_render_space(const WorldPosition& position,
                                                       const WorldPosition& camera) noexcept {
    return RenderPosition{(position - camera).cast<float>()};
}

/// The camera-relative frame for one rendered frame.
///
/// Holds the camera's f64 world position and produces render-space model
/// matrices. Because every object is expressed relative to the camera, the view
/// matrix carries rotation only and has no translation -  a property asserted by
/// test, since a stray translation here reintroduces the precision loss the
/// whole scheme exists to avoid.
class CameraRelativeFrame {
public:
    CameraRelativeFrame() = default;
    explicit CameraRelativeFrame(const WorldPosition& camera) noexcept : camera_(camera) {}

    [[nodiscard]] const WorldPosition& camera() const noexcept { return camera_; }

    [[nodiscard]] RenderPosition to_render_space(const WorldPosition& position) const noexcept {
        return pn::math::to_render_space(position, camera_);
    }

    /// Model matrix placing an object at its camera-relative offset.
    [[nodiscard]] Mat4 model_matrix(const WorldPosition& position) const noexcept {
        return translation(to_render_space(position).value);
    }

    /// View matrix from a rotation-only basis.
    ///
    /// Takes the camera's orientation as a rigid matrix whose translation is
    /// ignored by construction: in camera-relative rendering the camera sits at
    /// the origin, so the view matrix is the inverse of a pure rotation.
    [[nodiscard]] static Mat4 view_matrix(const Mat4& camera_rotation) noexcept {
        Mat4 rotation_only = camera_rotation;
        rotation_only.at(0, 3) = 0.0f;
        rotation_only.at(1, 3) = 0.0f;
        rotation_only.at(2, 3) = 0.0f;
        return inverse_rigid(rotation_only);
    }

private:
    WorldPosition camera_{};
};

}  // namespace pn::math

#endif  // PN_MATH_WORLD_HPP
