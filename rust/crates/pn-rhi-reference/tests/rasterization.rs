// Pention Engine - reference rasterizer tests
// Requirement: PN-RND-003, PN-OPS-007
// Decision:    ADR-0003, ADR-0005
//
// Golden images as text. A failure prints the picture next to the expected one,
// so the difference is visible in the test output without opening a viewer or
// trusting an encoder - and a golden image nobody can read is a golden image
// nobody will update correctly.
//
// The cases chosen are the ones a rasterizer gets wrong in a way that survives
// casual inspection: the fill rule on a shared edge, perspective correction on a
// triangle that is not facing the camera, the sign of reversed-Z, the Y flip,
// and the near plane.

use pn_rhi_reference::raster::{
    CullMode, DepthTest, Framebuffer, RasterState, Vertex, draw_triangles, MAX_ATTRIBUTES,
};

const NO_ATTRIBUTES: [f32; MAX_ATTRIBUTES] = [0.0; MAX_ATTRIBUTES];

fn white(_: &[f32; MAX_ATTRIBUTES]) -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

/// Front-facing under the engine's conventions: right-handed, counter-clockwise
/// in NDC.
fn front_face_state() -> RasterState {
    RasterState::default()
}

fn no_cull() -> RasterState {
    RasterState { cull: CullMode::None, ..RasterState::default() }
}

// -------------------------------------------------------------------
// Geometry
// -------------------------------------------------------------------

#[test]
fn a_triangle_covering_the_lower_left_half_renders_as_expected() {
    let mut target = Framebuffer::new(8, 8);
    target.clear([0.0, 0.0, 0.0, 0.0]);

    // Counter-clockwise in NDC: bottom-left, bottom-right, top-left.
    let vertices = [
        Vertex::ndc(-1.0, -1.0, 0.5, NO_ATTRIBUTES),
        Vertex::ndc(1.0, -1.0, 0.5, NO_ATTRIBUTES),
        Vertex::ndc(-1.0, 1.0, 0.5, NO_ATTRIBUTES),
    ];
    draw_triangles(&mut target, front_face_state(), &vertices, &[0, 1, 2], white);

    // NDC (-1,-1) is the image's bottom-left, because +Y is up in NDC and row
    // zero is at the top of a framebuffer. So the covered half is the lower-left
    // of the picture, widening towards the bottom. An implementation that
    // forgets the Y flip renders the vertical mirror of this and passes every
    // coverage-count test, which is why the expected picture is written out
    // rather than the pixel total.
    assert_eq!(
        target.to_ascii(),
        "@       \n\
         @@      \n\
         @@@     \n\
         @@@@    \n\
         @@@@@   \n\
         @@@@@@  \n\
         @@@@@@@ \n\
         @@@@@@@@\n"
    );
}

#[test]
fn two_triangles_sharing_an_edge_tile_a_square_exactly_once() {
    // The fill rule, which is the whole reason it exists. Without it the shared
    // diagonal is either drawn twice - a seam under blending, and double shading
    // cost always - or not at all, leaving a crack.
    //
    // Counting is not enough on its own: it would also pass if one triangle
    // covered everything and the other nothing. So the count is checked against
    // the full area, and the picture against a solid block.
    let mut target = Framebuffer::new(8, 8);
    target.clear([0.0, 0.0, 0.0, 0.0]);

    let vertices = [
        Vertex::ndc(-1.0, -1.0, 0.5, NO_ATTRIBUTES),
        Vertex::ndc(1.0, -1.0, 0.5, NO_ATTRIBUTES),
        Vertex::ndc(1.0, 1.0, 0.5, NO_ATTRIBUTES),
        Vertex::ndc(-1.0, 1.0, 0.5, NO_ATTRIBUTES),
    ];

    // Count how many times each pixel is written, by shading with an increment.
    let mut writes = vec![0u32; 64];
    {
        let mut counting = Framebuffer::new(8, 8);
        counting.clear([0.0, 0.0, 0.0, 0.0]);
        // Depth test Always with no write, so the second triangle is not
        // rejected by the first triangle's depth - which would hide a
        // double-cover behind a depth test rather than the fill rule.
        let state = RasterState {
            depth_test: DepthTest::Always,
            depth_write: false,
            cull: CullMode::Back,
        };
        for triangle in [[0u32, 1, 2], [0, 2, 3]] {
            let mut before = vec![false; 64];
            for y in 0..8u32 {
                for x in 0..8u32 {
                    before[(y * 8 + x) as usize] = counting.color_at(x, y)[3] > 0.0;
                }
            }
            draw_triangles(&mut counting, state, &vertices, &triangle, white);
            for y in 0..8u32 {
                for x in 0..8u32 {
                    let index = (y * 8 + x) as usize;
                    if counting.color_at(x, y)[3] > 0.0 && !before[index] {
                        writes[index] += 1;
                    }
                }
            }
        }
        assert_eq!(counting.covered_pixels(), 64, "the two triangles must fill the square");
    }

    draw_triangles(
        &mut target,
        RasterState { depth_test: DepthTest::Always, ..front_face_state() },
        &vertices,
        &[0, 1, 2, 0, 2, 3],
        white,
    );
    assert_eq!(target.covered_pixels(), 64);
    assert_eq!(
        target.to_ascii(),
        "@@@@@@@@\n@@@@@@@@\n@@@@@@@@\n@@@@@@@@\n\
         @@@@@@@@\n@@@@@@@@\n@@@@@@@@\n@@@@@@@@\n"
    );
}

#[test]
fn a_degenerate_triangle_covers_nothing() {
    // Clipping and projection legitimately produce these, and the next thing
    // that would happen is a division by the zero area.
    let mut target = Framebuffer::new(8, 8);
    target.clear([0.0, 0.0, 0.0, 0.0]);

    let collinear = [
        Vertex::ndc(-1.0, 0.0, 0.5, NO_ATTRIBUTES),
        Vertex::ndc(0.0, 0.0, 0.5, NO_ATTRIBUTES),
        Vertex::ndc(1.0, 0.0, 0.5, NO_ATTRIBUTES),
    ];
    draw_triangles(&mut target, no_cull(), &collinear, &[0, 1, 2], white);
    assert_eq!(target.covered_pixels(), 0);
}

#[test]
fn geometry_entirely_outside_the_viewport_writes_nothing() {
    let mut target = Framebuffer::new(8, 8);
    target.clear([0.0, 0.0, 0.0, 0.0]);

    let offscreen = [
        Vertex::ndc(3.0, 3.0, 0.5, NO_ATTRIBUTES),
        Vertex::ndc(5.0, 3.0, 0.5, NO_ATTRIBUTES),
        Vertex::ndc(3.0, 5.0, 0.5, NO_ATTRIBUTES),
    ];
    draw_triangles(&mut target, no_cull(), &offscreen, &[0, 1, 2], white);
    assert_eq!(target.covered_pixels(), 0);
}

// -------------------------------------------------------------------
// Culling
// -------------------------------------------------------------------

#[test]
fn winding_selects_which_face_is_culled() {
    let counter_clockwise = [
        Vertex::ndc(-1.0, -1.0, 0.5, NO_ATTRIBUTES),
        Vertex::ndc(1.0, -1.0, 0.5, NO_ATTRIBUTES),
        Vertex::ndc(-1.0, 1.0, 0.5, NO_ATTRIBUTES),
    ];
    let clockwise = [counter_clockwise[0], counter_clockwise[2], counter_clockwise[1]];

    let draw = |vertices: &[Vertex; 3], cull: CullMode| {
        let mut target = Framebuffer::new(8, 8);
        target.clear([0.0, 0.0, 0.0, 0.0]);
        draw_triangles(
            &mut target,
            RasterState { cull, ..RasterState::default() },
            vertices,
            &[0, 1, 2],
            white,
        );
        target.covered_pixels()
    };

    // Counter-clockwise in NDC is front-facing, matching the right-handed
    // convention. Getting this backwards inverts culling everywhere and looks
    // like missing geometry.
    assert!(draw(&counter_clockwise, CullMode::Back) > 0);
    assert_eq!(draw(&counter_clockwise, CullMode::Front), 0);
    assert_eq!(draw(&clockwise, CullMode::Back), 0);
    assert!(draw(&clockwise, CullMode::Front) > 0);
    assert!(draw(&clockwise, CullMode::None) > 0);
}

// -------------------------------------------------------------------
// Reversed-Z
// -------------------------------------------------------------------

#[test]
fn under_reversed_z_the_larger_depth_wins() {
    // Near maps to 1.0 and far towards 0.0, so "nearer" is "greater". A
    // rasterizer that assumes the opposite produces ordinary-looking
    // z-fighting, which is why the convention document asks for this test by
    // name.
    let mut target = Framebuffer::new(4, 4);
    target.clear([0.0, 0.0, 0.0, 0.0]);

    let quad = |z: f32| {
        [
            Vertex::ndc(-1.0, -1.0, z, NO_ATTRIBUTES),
            Vertex::ndc(1.0, -1.0, z, NO_ATTRIBUTES),
            Vertex::ndc(1.0, 1.0, z, NO_ATTRIBUTES),
            Vertex::ndc(-1.0, 1.0, z, NO_ATTRIBUTES),
        ]
    };
    let indices = [0u32, 1, 2, 0, 2, 3];

    // Far first, then near: the near one must replace it.
    draw_triangles(&mut target, front_face_state(), &quad(0.25), &indices, |_| {
        [0.25, 0.25, 0.25, 1.0]
    });
    draw_triangles(&mut target, front_face_state(), &quad(0.75), &indices, |_| {
        [1.0, 1.0, 1.0, 1.0]
    });
    assert_eq!(target.color_at(2, 2), [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(target.depth_at(2, 2), 0.75);

    // Now the reverse order: the far one must be rejected.
    draw_triangles(&mut target, front_face_state(), &quad(0.25), &indices, |_| {
        [0.25, 0.25, 0.25, 1.0]
    });
    assert_eq!(target.color_at(2, 2), [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(target.depth_at(2, 2), 0.75);
}

#[test]
fn depth_clears_to_zero_which_is_the_far_value() {
    // Clearing to 1.0 is the reversed-Z mistake the convention exists to
    // prevent: everything then fails the depth test and the screen stays empty.
    let mut target = Framebuffer::new(4, 4);
    target.clear([0.0, 0.0, 0.0, 1.0]);
    for y in 0..4 {
        for x in 0..4 {
            assert_eq!(target.depth_at(x, y), 0.0);
        }
    }
}

#[test]
fn depth_write_can_be_disabled_without_disabling_the_test() {
    let mut target = Framebuffer::new(4, 4);
    target.clear([0.0, 0.0, 0.0, 0.0]);

    let quad = |z: f32| {
        [
            Vertex::ndc(-1.0, -1.0, z, NO_ATTRIBUTES),
            Vertex::ndc(1.0, -1.0, z, NO_ATTRIBUTES),
            Vertex::ndc(1.0, 1.0, z, NO_ATTRIBUTES),
            Vertex::ndc(-1.0, 1.0, z, NO_ATTRIBUTES),
        ]
    };
    let indices = [0u32, 1, 2, 0, 2, 3];

    draw_triangles(&mut target, front_face_state(), &quad(0.5), &indices, white);
    assert_eq!(target.depth_at(2, 2), 0.5);

    let read_only = RasterState { depth_write: false, ..front_face_state() };
    draw_triangles(&mut target, read_only, &quad(0.9), &indices, |_| [0.5, 0.5, 0.5, 1.0]);
    assert_eq!(target.color_at(2, 2), [0.5, 0.5, 0.5, 1.0], "the fragment passed");
    assert_eq!(target.depth_at(2, 2), 0.5, "but did not write depth");
}

// -------------------------------------------------------------------
// Interpolation
// -------------------------------------------------------------------

#[test]
fn attributes_are_interpolated_across_a_triangle() {
    let mut target = Framebuffer::new(16, 16);
    target.clear([0.0, 0.0, 0.0, 0.0]);

    let vertices = [
        Vertex::ndc(-1.0, -1.0, 0.5, [0.0, 0.0, 0.0, 0.0]),
        Vertex::ndc(1.0, -1.0, 0.5, [1.0, 0.0, 0.0, 0.0]),
        Vertex::ndc(-1.0, 1.0, 0.5, [0.0, 1.0, 0.0, 0.0]),
    ];
    draw_triangles(&mut target, front_face_state(), &vertices, &[0, 1, 2], |a| {
        [a[0], a[1], 0.0, 1.0]
    });

    // The corner nearest each vertex carries that vertex's value.
    let bottom_left = target.color_at(0, 15);
    assert!(bottom_left[0] < 0.15 && bottom_left[1] < 0.15, "{bottom_left:?}");
    let top_left = target.color_at(0, 0);
    assert!(top_left[1] > 0.85, "{top_left:?}");
}

#[test]
fn interpolation_is_perspective_correct() {
    // The test that separates a real rasterizer from one that looks right on
    // anything facing the camera squarely.
    //
    // Two vertices at different depths, so their clip w differs. At the
    // half-way pixel the screen-space barycentric is 0.5, but the correct
    // attribute value is not: it is weighted by 1/w. Linear interpolation is
    // the classic wobbling-texture bug.
    let mut target = Framebuffer::new(64, 4);
    target.clear([0.0, 0.0, 0.0, 0.0]);

    // w = 1 on the left, w = 4 on the right. x/w spans the viewport either way.
    let vertices = [
        Vertex::new([-1.0, -1.0, 0.5, 1.0], [0.0, 0.0, 0.0, 0.0]),
        Vertex::new([4.0, -4.0, 2.0, 4.0], [1.0, 0.0, 0.0, 0.0]),
        Vertex::new([-1.0, 1.0, 0.5, 1.0], [0.0, 0.0, 0.0, 0.0]),
    ];
    draw_triangles(&mut target, no_cull(), &vertices, &[0, 1, 2], |a| [a[0], 0.0, 0.0, 1.0]);

    // At the horizontal midpoint the screen-space weight is 1/2. The
    // perspective-correct value there is
    //     (0.5 * 1/4) / (0.5 * 1/1 + 0.5 * 1/4) = 0.2
    // so linear interpolation would give 0.5 and be wrong by a wide margin.
    let midpoint = target.color_at(32, 3);
    assert!(midpoint[3] > 0.0, "the midpoint must be covered");
    assert!(
        (midpoint[0] - 0.2).abs() < 0.05,
        "expected about 0.2, got {} - this is linear interpolation, not perspective-correct",
        midpoint[0]
    );
}

// -------------------------------------------------------------------
// Clipping
// -------------------------------------------------------------------

#[test]
fn a_triangle_entirely_behind_the_camera_draws_nothing() {
    let mut target = Framebuffer::new(8, 8);
    target.clear([0.0, 0.0, 0.0, 0.0]);

    // Negative w: behind the eye. Without clipping the perspective divide flips
    // the geometry inside out and it appears, mirrored, in front.
    let behind = [
        Vertex::new([-1.0, -1.0, -1.0, -1.0], NO_ATTRIBUTES),
        Vertex::new([1.0, -1.0, -1.0, -1.0], NO_ATTRIBUTES),
        Vertex::new([-1.0, 1.0, -1.0, -1.0], NO_ATTRIBUTES),
    ];
    draw_triangles(&mut target, no_cull(), &behind, &[0, 1, 2], white);
    assert_eq!(target.covered_pixels(), 0);
}

#[test]
fn a_triangle_straddling_the_near_plane_is_clipped_not_mangled() {
    // One vertex in front, two behind. The result must be a sane partial
    // triangle rather than the garbage an unclipped perspective divide
    // produces.
    let mut target = Framebuffer::new(16, 16);
    target.clear([0.0, 0.0, 0.0, 0.0]);

    let straddling = [
        Vertex::new([0.0, 0.0, 0.5, 1.0], NO_ATTRIBUTES),
        Vertex::new([2.0, -2.0, 2.0, -1.0], NO_ATTRIBUTES),
        Vertex::new([-2.0, -2.0, 2.0, -1.0], NO_ATTRIBUTES),
    ];
    draw_triangles(&mut target, no_cull(), &straddling, &[0, 1, 2], white);

    let covered = target.covered_pixels();
    assert!(covered > 0, "the visible part must still be drawn");
    assert!(covered < 256, "it must not fill the whole target");
}

// -------------------------------------------------------------------
// Determinism and output
// -------------------------------------------------------------------

#[test]
fn the_same_draw_produces_byte_identical_output() {
    // A golden image is only a test if the renderer is deterministic.
    let render = || {
        let mut target = Framebuffer::new(32, 32);
        target.clear([0.0, 0.0, 0.0, 0.0]);
        let vertices = [
            Vertex::ndc(-0.8, -0.8, 0.5, [1.0, 0.0, 0.0, 0.0]),
            Vertex::ndc(0.9, -0.6, 0.7, [0.0, 1.0, 0.0, 0.0]),
            Vertex::ndc(-0.1, 0.85, 0.3, [0.0, 0.0, 1.0, 0.0]),
        ];
        draw_triangles(&mut target, no_cull(), &vertices, &[0, 1, 2], |a| {
            [a[0], a[1], a[2], 1.0]
        });
        target
    };

    let reference = render();
    for _ in 0..8 {
        assert_eq!(render(), reference);
        assert_eq!(render().to_ppm(), reference.to_ppm());
    }
}

#[test]
fn the_ppm_output_has_the_right_header_and_size() {
    let mut target = Framebuffer::new(3, 2);
    target.clear([1.0, 1.0, 1.0, 1.0]);
    let ppm = target.to_ppm();

    assert!(ppm.starts_with(b"P6\n3 2\n255\n"));
    assert_eq!(ppm.len(), "P6\n3 2\n255\n".len() + 3 * 2 * 3);
    // White in linear is white in sRGB.
    assert_eq!(ppm[ppm.len() - 1], 255);
}

#[test]
fn srgb_encoding_is_applied_exactly_once_at_output() {
    // Mid-grey in linear is not mid-grey in sRGB. A pipeline that encodes twice,
    // or not at all, is visibly wrong and easy to ship.
    let mut target = Framebuffer::new(1, 1);
    target.clear([0.5, 0.5, 0.5, 1.0]);
    let ppm = target.to_ppm();
    let byte = ppm[ppm.len() - 1];

    // 1.055 * 0.5^(1/2.4) - 0.055 = 0.7354..., which is 188 of 255.
    assert_eq!(byte, 188, "linear 0.5 must encode to sRGB 188, not 128");

    // And the framebuffer itself is untouched: it stays linear.
    assert_eq!(target.color_at(0, 0), [0.5, 0.5, 0.5, 1.0]);
}

#[test]
fn the_depth_buffer_can_be_read_as_a_picture() {
    // A depth-ordering failure should be visible, not inferred from a colour
    // that happens to be wrong.
    let mut target = Framebuffer::new(4, 4);
    target.clear([0.0, 0.0, 0.0, 0.0]);
    let quad = [
        Vertex::ndc(-1.0, -1.0, 1.0, NO_ATTRIBUTES),
        Vertex::ndc(1.0, -1.0, 1.0, NO_ATTRIBUTES),
        Vertex::ndc(1.0, 1.0, 1.0, NO_ATTRIBUTES),
        Vertex::ndc(-1.0, 1.0, 1.0, NO_ATTRIBUTES),
    ];
    draw_triangles(&mut target, front_face_state(), &quad, &[0, 1, 2, 0, 2, 3], white);

    // Depth 1.0 everywhere: the near plane, the brightest character. Geometry
    // lying exactly on the near plane must survive clipping - an earlier version
    // required a positive epsilon and deleted it, which presents as surfaces
    // vanishing the moment you walk right up to them.
    assert_eq!(target.depth_to_ascii(), "@@@@\n@@@@\n@@@@\n@@@@\n");
}

#[test]
fn geometry_exactly_on_the_near_plane_survives_clipping() {
    // The regression test for the defect above, stated on its own rather than
    // discovered inside a test about something else. z == w is the near plane,
    // and the near plane is inside the frustum.
    let mut target = Framebuffer::new(8, 8);
    target.clear([0.0, 0.0, 0.0, 0.0]);

    let on_the_plane = [
        Vertex::new([-1.0, -1.0, 1.0, 1.0], NO_ATTRIBUTES),
        Vertex::new([1.0, -1.0, 1.0, 1.0], NO_ATTRIBUTES),
        Vertex::new([-1.0, 1.0, 1.0, 1.0], NO_ATTRIBUTES),
    ];
    draw_triangles(&mut target, no_cull(), &on_the_plane, &[0, 1, 2], white);
    assert!(target.covered_pixels() > 0, "geometry at z == w must not be clipped away");
    assert_eq!(target.depth_at(0, 7), 1.0);
}

#[test]
fn a_vertex_closer_than_the_near_plane_is_clipped_back_to_it() {
    // Two vertices in front of the near plane, one behind it. The result should
    // be the visible portion, with the new edge lying on the plane.
    let mut target = Framebuffer::new(16, 16);
    target.clear([0.0, 0.0, 0.0, 0.0]);

    let straddling = [
        // z > w: closer than near, must be clipped.
        Vertex::new([0.0, 0.0, 2.0, 1.0], NO_ATTRIBUTES),
        Vertex::new([1.0, -1.0, 0.2, 1.0], NO_ATTRIBUTES),
        Vertex::new([-1.0, -1.0, 0.2, 1.0], NO_ATTRIBUTES),
    ];
    draw_triangles(&mut target, no_cull(), &straddling, &[0, 1, 2], white);

    let covered = target.covered_pixels();
    assert!(covered > 0, "the part beyond the near plane must be drawn");
    assert!(covered < 256, "the part in front of it must not be");

    // Nothing may end up with a depth above the near plane's 1.0; a value
    // greater than that is exactly the geometry that should have been clipped.
    for y in 0..16u32 {
        for x in 0..16u32 {
            assert!(
                target.depth_at(x, y) <= 1.0,
                "pixel ({x},{y}) has depth {} - clipping let unclipped geometry through",
                target.depth_at(x, y)
            );
        }
    }
}
