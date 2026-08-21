// Pention Engine -  math/projection.hpp
// Requirement: docs/conventions.md (reversed-Z, infinite far plane)
// Decision:    ADR-0003

#ifndef PN_MATH_PROJECTION_HPP
#define PN_MATH_PROJECTION_HPP

#include "pn/math/matrix.hpp"
#include "pn/math/scalar.hpp"

#include <cmath>

namespace pn::math {

/// Right-handed perspective projection with reversed-Z and an infinite far
/// plane, targeting a [0, 1] clip-space depth range.
///
/// Reversed-Z maps the near plane to 1.0 and infinity to 0.0. Floating-point
/// density is highest near zero, and under this mapping that density lands where
/// depth precision is scarce -  at distance -  instead of being wasted near the
/// camera. It costs nothing to adopt on day one and is invasive later, because
/// every depth comparison, every projection matrix, and every depth-reconstruction
/// shader flips.
///
/// Derivation, so the constants are checkable rather than folklore. The camera
/// looks down -Z, so a point at distance d ahead has `z_view = -d`. Choosing
/// `clip.w = -z_view = d` and `clip.z = near` gives
/// `ndc.z = clip.z / clip.w = near / d`, which is exactly 1 at `d = near` and
/// tends to 0 as d grows without bound. No far plane appears in the result,
/// which is what removes far-plane clipping entirely.
///
/// The Vulkan Y-flip is deliberately NOT folded in here. It is applied once, via
/// a negative-height viewport (docs/conventions.md), so these matrices stay
/// identical to every published reference. Negating Y inside the projection
/// silently reverses winding order, after which every bug looks like a culling
/// bug.
///
/// @param vertical_fov_radians  Full vertical field of view, in radians.
/// @param aspect_ratio          Width divided by height.
/// @param near_plane            Distance to the near plane. Must be > 0.
template <Scalar T>
[[nodiscard]] Mat4T<T> perspective_reversed_z_infinite(T vertical_fov_radians,
                                                       T aspect_ratio,
                                                       T near_plane) noexcept {
    const T focal = T{1} / std::tan(vertical_fov_radians / T{2});

    Mat4T<T> m = Mat4T<T>::zero();
    m.at(0, 0) = focal / aspect_ratio;
    m.at(1, 1) = focal;
    m.at(2, 3) = near_plane;
    m.at(3, 2) = T{-1};
    return m;
}

/// Depth value a point at the given view-space distance projects to.
///
/// Exposed so tests can assert the reversed-Z mapping directly rather than
/// inferring it from a rendered image.
template <Scalar T>
[[nodiscard]] constexpr T reversed_z_depth(T near_plane, T view_distance) noexcept {
    if (view_distance <= T{0}) {
        return T{1};
    }
    return near_plane / view_distance;
}

}  // namespace pn::math

#endif  // PN_MATH_PROJECTION_HPP
