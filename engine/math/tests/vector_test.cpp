// Pention Engine - math/tests/vector_test.cpp
// Requirement: PN-WLD-001
// Decision:    ADR-0003

#include "pn/math/vector.hpp"
#include "pn/testing/test.hpp"

namespace {
using pn::math::DVec3;
using pn::math::Vec3;
}  // namespace

PN_TEST(vector, arithmetic_is_componentwise) {
    const Vec3 a{1.0f, 2.0f, 3.0f};
    const Vec3 b{4.0f, 5.0f, 6.0f};
    PN_CHECK_EQ(a + b, Vec3(5.0f, 7.0f, 9.0f));
    PN_CHECK_EQ(b - a, Vec3(3.0f, 3.0f, 3.0f));
    PN_CHECK_EQ(a * 2.0f, Vec3(2.0f, 4.0f, 6.0f));
    PN_CHECK_EQ(2.0f * a, Vec3(2.0f, 4.0f, 6.0f));
    PN_CHECK_EQ(-a, Vec3(-1.0f, -2.0f, -3.0f));
}

PN_TEST(vector, dot_and_length_agree) {
    const Vec3 v{3.0f, 4.0f, 0.0f};
    PN_CHECK_NEAR(pn::math::length_squared(v), 25.0f, 1e-6f);
    PN_CHECK_NEAR(pn::math::length(v), 5.0f, 1e-6f);
    PN_CHECK_NEAR(dot(v, v), 25.0f, 1e-6f);
}

PN_TEST(vector, cross_product_is_antisymmetric) {
    const Vec3 a{1.0f, 2.0f, 3.0f};
    const Vec3 b{4.0f, 5.0f, 6.0f};
    PN_CHECK_EQ(cross(a, b), -cross(b, a));
    // And orthogonal to both inputs.
    PN_CHECK_NEAR(dot(cross(a, b), a), 0.0f, 1e-5f);
    PN_CHECK_NEAR(dot(cross(a, b), b), 0.0f, 1e-5f);
}

PN_TEST(vector, normalize_returns_unit_length) {
    const Vec3 v{0.0f, 0.0f, -8.0f};
    PN_CHECK_NEAR(pn::math::length(pn::math::normalize(v)), 1.0f, 1e-6f);
}

PN_TEST(vector, normalize_of_zero_returns_zero_not_nan) {
    // Returning zero rather than producing NaN keeps a degenerate input from
    // silently poisoning every downstream computation.
    const Vec3 zero{};
    const Vec3 result = pn::math::normalize(zero);
    PN_CHECK_EQ(result, Vec3(0.0f, 0.0f, 0.0f));
    PN_CHECK(result.x == result.x);  // not NaN
}

PN_TEST(vector, cast_between_precisions_is_explicit) {
    const DVec3 world{1.5, -2.25, 3.125};
    const Vec3 render = world.cast<float>();
    PN_CHECK_EQ(render, Vec3(1.5f, -2.25f, 3.125f));
}

PN_TEST(vector, lerp_hits_both_endpoints) {
    const Vec3 a{0.0f, 0.0f, 0.0f};
    const Vec3 b{10.0f, 20.0f, 30.0f};
    PN_CHECK_EQ(pn::math::lerp(a, b, 0.0f), a);
    PN_CHECK_EQ(pn::math::lerp(a, b, 1.0f), b);
    PN_CHECK_EQ(pn::math::lerp(a, b, 0.5f), Vec3(5.0f, 10.0f, 15.0f));
}
