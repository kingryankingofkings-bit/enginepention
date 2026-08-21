// Pention Engine - math/tests/precision_test.cpp
// Requirement: PN-WLD-001, PN-WLD-002
// Decision:    ADR-0003
//
// These tests exist to make the precision claims in docs/conventions.md
// checkable. A table of ULP figures in a document drifts away from reality; a
// test that computes them does not.

#include "pn/math/scalar.hpp"
#include "pn/math/vector.hpp"
#include "pn/math/world.hpp"
#include "pn/testing/test.hpp"

namespace {

using pn::math::DVec3;
using pn::math::Vec3;
using pn::math::WorldPosition;

}  // namespace

PN_TEST(precision, f32_ulp_table_matches_conventions_document) {
    // docs/conventions.md publishes these figures to justify the f64 world
    // space decision. Asserting them keeps document and reality together.
    PN_CHECK_NEAR(pn::math::ulp_at(1'000.0f), 6.1e-5f, 1e-6f);        // ~0.06 mm at 1 km
    PN_CHECK_NEAR(pn::math::ulp_at(10'000.0f), 9.77e-4f, 1e-5f);      // ~1 mm at 10 km
    PN_CHECK_NEAR(pn::math::ulp_at(100'000.0f), 7.81e-3f, 1e-4f);     // ~8 mm at 100 km
    PN_CHECK_NEAR(pn::math::ulp_at(1'000'000.0f), 6.25e-2f, 1e-3f);   // ~6 cm at 1000 km
}

PN_TEST(precision, f64_ulp_is_negligible_at_world_scale) {
    // The reason world space is f64: at 1000 km its ULP is sub-micrometre, so
    // world-space arithmetic is not the precision bottleneck at any plausible
    // world size.
    PN_CHECK_LT(pn::math::ulp_at(1'000'000.0), 1e-9);
}

PN_TEST(precision, camera_relative_conversion_preserves_detail_at_500km) {
    // PN-WLD-001's acceptance criterion is stated at 500 km from the origin.
    const WorldPosition camera{500'000.0, 0.0, 0.0};
    const WorldPosition object{500'000.0 + 1.234567, 2.5, -3.75};

    const Vec3 relative = pn::math::to_render_space(object, camera).value;

    // Subtraction happens in f64, so only the small result is narrowed. The
    // residual error is bounded by the f32 ULP at the *offset* magnitude,
    // roughly 1e-7 m, not by the ULP at 500 km.
    PN_CHECK_NEAR(relative.x, 1.234567f, 1e-6f);
    PN_CHECK_NEAR(relative.y, 2.5f, 1e-6f);
    PN_CHECK_NEAR(relative.z, -3.75f, 1e-6f);
}

PN_TEST(precision, naive_f32_world_space_visibly_fails_at_500km) {
    // The negative control. This is what the engine would do if world positions
    // were f32: narrow first, subtract second. It is here so that a future
    // change which quietly reintroduces f32 world storage fails a test that
    // explains why, rather than producing a rendering artefact nobody can trace.
    const double camera_x = 500'000.0;
    const double object_x = 500'000.0 + 1.234567;

    const float naive = static_cast<float>(object_x) - static_cast<float>(camera_x);
    const float correct = static_cast<float>(object_x - camera_x);

    const float naive_error = naive > correct ? naive - correct : correct - naive;

    // f32 ULP at 500 km is 2^-5 m = 31.25 mm, so the naive result is wrong by
    // an amount that is plainly visible in a rendered frame.
    PN_CHECK_GT(naive_error, 1.0e-3f);
    PN_CHECK_NEAR(pn::math::ulp_at(500'000.0f), 0.03125f, 1e-6f);
}

PN_TEST(precision, world_positions_subtract_in_double_precision) {
    constexpr double magnitude = 1'000'000.0;
    constexpr double separation = 0.001;  // one millimetre

    const WorldPosition a{magnitude, 0.0, 0.0};
    const WorldPosition b{magnitude + separation, 0.0, 0.0};
    const DVec3 delta = b - a;

    // The meaningful invariant is not that the result is exact - it cannot be,
    // because magnitude + separation is not representable in f64. It is that
    // the error is bounded by the representation error at the operand
    // magnitude, and that this error is orders of magnitude smaller than the
    // separation being resolved.
    //
    // Deriving the tolerance from ulp_at() rather than hard-coding it means
    // this test states the actual guarantee and cannot drift into asserting a
    // precision the type does not have. An earlier version of this test used a
    // hard-coded 1e-12, which is tighter than f64 itself at this magnitude.
    const double representation_ulp = pn::math::ulp_at(magnitude);
    PN_CHECK_NEAR(delta.x, separation, 4.0 * representation_ulp);

    // One millimetre at 1000 km is resolvable with vast margin: the ULP here is
    // sub-nanometre, roughly seven orders of magnitude below the separation.
    PN_CHECK_LT(representation_ulp, separation / 1'000'000.0);
}

PN_TEST(precision, camera_relative_frame_places_camera_at_origin) {
    const WorldPosition camera{123'456.0, -7'890.0, 42.0};
    const pn::math::CameraRelativeFrame frame{camera};

    const Vec3 at_camera = frame.to_render_space(camera).value;
    PN_CHECK_EQ(at_camera, Vec3(0.0f, 0.0f, 0.0f));
}
