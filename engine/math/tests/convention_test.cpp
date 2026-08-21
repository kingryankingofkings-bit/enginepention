// Pention Engine - math/tests/convention_test.cpp
// Requirement: docs/conventions.md
// Decision:    ADR-0003
//
// Each test here pins one binding convention. A convention with no test is a
// suggestion, and these are the conventions whose violation produces bugs that
// look like something else entirely.

#include "pn/math/matrix.hpp"
#include "pn/math/projection.hpp"
#include "pn/math/scalar.hpp"
#include "pn/math/vector.hpp"
#include "pn/math/world.hpp"
#include "pn/testing/test.hpp"

namespace {

using pn::math::Axis;
using pn::math::Mat4;
using pn::math::Vec3;
using pn::math::Vec4;

}  // namespace

PN_TEST(convention, basis_is_right_handed_y_up) {
    // The single cheapest test in the engine. Getting this backwards produces
    // mirrored normals and inverted culling, which are individually plausible
    // and collectively baffling.
    const Vec3 z = cross(Axis<float>::right, Axis<float>::up);
    PN_CHECK_EQ(z, Vec3(0.0f, 0.0f, 1.0f));

    // Forward is -Z, into the screen.
    PN_CHECK_EQ(Axis<float>::forward, Vec3(0.0f, 0.0f, -1.0f));

    // And the handedness holds in double precision too.
    const auto dz = cross(Axis<double>::right, Axis<double>::up);
    PN_CHECK_EQ(dz, pn::math::DVec3(0.0, 0.0, 1.0));
}

PN_TEST(convention, reversed_z_maps_near_to_one_and_far_toward_zero) {
    constexpr float near_plane = 0.1f;
    const Mat4 projection = pn::math::perspective_reversed_z_infinite(
        pn::math::radians_from_degrees(60.0f), 16.0f / 9.0f, near_plane);

    // A point exactly on the near plane, in view space (camera looks down -Z).
    const Vec4 at_near = projection * Vec4(0.0f, 0.0f, -near_plane, 1.0f);
    PN_REQUIRE(at_near.w > 0.0f);
    PN_CHECK_NEAR(at_near.z / at_near.w, 1.0f, 1e-6f);

    // A very distant point approaches zero rather than clipping.
    const Vec4 far_away = projection * Vec4(0.0f, 0.0f, -1.0e7f, 1.0f);
    PN_REQUIRE(far_away.w > 0.0f);
    const float far_depth = far_away.z / far_away.w;
    PN_CHECK_GT(far_depth, 0.0f);
    PN_CHECK_LT(far_depth, 1.0e-5f);

    // Depth decreases monotonically with distance - the defining property.
    const Vec4 mid = projection * Vec4(0.0f, 0.0f, -100.0f, 1.0f);
    const float mid_depth = mid.z / mid.w;
    PN_CHECK_LT(mid_depth, 1.0f);
    PN_CHECK_GT(mid_depth, far_depth);
}

PN_TEST(convention, reversed_z_helper_agrees_with_the_matrix) {
    constexpr float near_plane = 0.05f;
    const Mat4 projection = pn::math::perspective_reversed_z_infinite(
        pn::math::radians_from_degrees(90.0f), 1.0f, near_plane);

    for (const float distance : {0.05f, 1.0f, 10.0f, 1000.0f}) {
        const Vec4 clip = projection * Vec4(0.0f, 0.0f, -distance, 1.0f);
        PN_REQUIRE(clip.w > 0.0f);
        PN_CHECK_NEAR(clip.z / clip.w,
                      pn::math::reversed_z_depth(near_plane, distance), 1e-6f);
    }
}

PN_TEST(convention, projection_has_no_far_plane_term) {
    // An infinite far plane means no far distance appears anywhere in the
    // matrix. If a finite far plane is ever reintroduced, this fails.
    const Mat4 projection = pn::math::perspective_reversed_z_infinite(
        pn::math::radians_from_degrees(60.0f), 1.5f, 0.1f);
    PN_CHECK_EQ(projection.at(2, 2), 0.0f);
    PN_CHECK_EQ(projection.at(3, 2), -1.0f);
    PN_CHECK_EQ(projection.at(3, 3), 0.0f);
}

PN_TEST(convention, matrix_storage_is_column_major) {
    // Element (row, col) must live at data[col][row], so a matrix can be copied
    // into a uniform buffer with no transpose. A hidden transpose only
    // misbehaves under non-uniform scale or a non-symmetric projection, which
    // is why it survives months of testing.
    Mat4 m = Mat4::zero();
    m.at(1, 3) = 7.0f;  // row 1, column 3 - a translation component

    PN_CHECK_EQ(m.data[3][1], 7.0f);
    PN_CHECK_NE(m.data[1][3], 7.0f);

    // Translation lands in the fourth column, matching the shader convention.
    const Mat4 t = pn::math::translation(Vec3(1.0f, 2.0f, 3.0f));
    PN_CHECK_EQ(t.at(0, 3), 1.0f);
    PN_CHECK_EQ(t.at(1, 3), 2.0f);
    PN_CHECK_EQ(t.at(2, 3), 3.0f);
}

PN_TEST(convention, angles_round_trip_between_degrees_and_radians) {
    PN_CHECK_NEAR(pn::math::radians_from_degrees(180.0), pn::math::Constants<double>::pi, 1e-15);
    PN_CHECK_NEAR(pn::math::degrees_from_radians(pn::math::Constants<double>::half_pi), 90.0, 1e-13);
    PN_CHECK_NEAR(pn::math::degrees_from_radians(pn::math::radians_from_degrees(37.5)), 37.5, 1e-13);
}
