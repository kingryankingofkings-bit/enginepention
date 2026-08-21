// Pention Engine - math/tests/matrix_test.cpp
// Requirement: PN-WLD-002
// Decision:    ADR-0003

#include "pn/math/matrix.hpp"
#include "pn/math/vector.hpp"
#include "pn/math/world.hpp"
#include "pn/testing/test.hpp"

namespace {

using pn::math::Mat4;
using pn::math::Vec3;
using pn::math::Vec4;

/// Rotation about +Y by the given angle, in the engine's right-handed basis.
Mat4 rotation_y(float radians) {
    const float c = std::cos(radians);
    const float s = std::sin(radians);
    Mat4 m = Mat4::identity();
    m.at(0, 0) = c;
    m.at(0, 2) = s;
    m.at(2, 0) = -s;
    m.at(2, 2) = c;
    return m;
}

}  // namespace

PN_TEST(matrix, identity_is_multiplicative_unit) {
    const Mat4 id = Mat4::identity();
    const Mat4 t = pn::math::translation(Vec3(3.0f, -4.0f, 5.0f));
    PN_CHECK_EQ(id * t, t);
    PN_CHECK_EQ(t * id, t);
}

PN_TEST(matrix, multiplication_composes_left_to_right_as_applied) {
    const Mat4 translate = pn::math::translation(Vec3(10.0f, 0.0f, 0.0f));
    const Mat4 scale = pn::math::scaling(Vec3(2.0f, 2.0f, 2.0f));

    // (translate * scale) must scale first, then translate.
    const Vec3 point{1.0f, 0.0f, 0.0f};
    const Vec3 result = pn::math::transform_point(translate * scale, point);
    PN_CHECK_NEAR(result.x, 12.0f, 1e-6f);

    // The other order scales the translation too.
    const Vec3 other = pn::math::transform_point(scale * translate, point);
    PN_CHECK_NEAR(other.x, 22.0f, 1e-6f);
}

PN_TEST(matrix, transform_direction_ignores_translation) {
    const Mat4 t = pn::math::translation(Vec3(100.0f, 200.0f, 300.0f));
    const Vec3 direction{0.0f, 0.0f, -1.0f};
    PN_CHECK_EQ(pn::math::transform_direction(t, direction), direction);
    // While a point does move.
    PN_CHECK_NE(pn::math::transform_point(t, direction), direction);
}

PN_TEST(matrix, transpose_is_an_involution) {
    Mat4 m = Mat4::zero();
    float value = 1.0f;
    for (std::size_t col = 0; col < 4; ++col) {
        for (std::size_t row = 0; row < 4; ++row) {
            m.at(row, col) = value;
            value += 1.0f;
        }
    }
    PN_CHECK_EQ(pn::math::transpose(pn::math::transpose(m)), m);
    PN_CHECK_EQ(pn::math::transpose(m).at(2, 1), m.at(1, 2));
}

PN_TEST(matrix, inverse_rigid_undoes_rotation_and_translation) {
    const Mat4 transform =
        pn::math::translation(Vec3(5.0f, -2.0f, 7.0f)) * rotation_y(0.7f);
    const Mat4 inverse = pn::math::inverse_rigid(transform);

    const Mat4 product = transform * inverse;
    const Mat4 id = Mat4::identity();
    for (std::size_t col = 0; col < 4; ++col) {
        for (std::size_t row = 0; row < 4; ++row) {
            PN_CHECK_NEAR(product.at(row, col), id.at(row, col), 1e-5f);
        }
    }
}

PN_TEST(matrix, matrix_vector_product_respects_w) {
    const Mat4 t = pn::math::translation(Vec3(1.0f, 2.0f, 3.0f));
    const Vec4 point = t * Vec4(0.0f, 0.0f, 0.0f, 1.0f);
    PN_CHECK_EQ(point, Vec4(1.0f, 2.0f, 3.0f, 1.0f));

    const Vec4 direction = t * Vec4(1.0f, 0.0f, 0.0f, 0.0f);
    PN_CHECK_EQ(direction, Vec4(1.0f, 0.0f, 0.0f, 0.0f));
}

PN_TEST(matrix, view_matrix_from_camera_relative_frame_has_no_translation) {
    // In camera-relative rendering the camera sits at the origin, so the view
    // matrix must be rotation only. A stray translation here reintroduces
    // exactly the large-magnitude coordinates ADR-0003 exists to avoid.
    Mat4 camera = rotation_y(1.1f);
    camera.at(0, 3) = 500000.0f;  // a world-scale translation that must be dropped
    camera.at(1, 3) = -12345.0f;
    camera.at(2, 3) = 999.0f;

    const Mat4 view = pn::math::CameraRelativeFrame::view_matrix(camera);

    PN_CHECK_NEAR(view.at(0, 3), 0.0f, 1e-6f);
    PN_CHECK_NEAR(view.at(1, 3), 0.0f, 1e-6f);
    PN_CHECK_NEAR(view.at(2, 3), 0.0f, 1e-6f);

    // And it still inverts the rotation correctly.
    const Mat4 rotation_only = rotation_y(1.1f);
    const Mat4 product = view * rotation_only;
    PN_CHECK_NEAR(product.at(0, 0), 1.0f, 1e-5f);
    PN_CHECK_NEAR(product.at(1, 1), 1.0f, 1e-5f);
    PN_CHECK_NEAR(product.at(2, 2), 1.0f, 1e-5f);
}
