// Pention Engine - pn-rhi-reference/raster.rs
// Requirement: PN-RND-003, PN-OPS-007
// Decision:    ADR-0003 (coordinates, units, precision), ADR-0005
//
// The second half of ADR-0005: correctness-oriented software rasterization,
// sufficient for golden-image tests of geometry, transforms, depth ordering, and
// shading math.
//
// It is a correctness oracle, not a renderer. There is no tiling, no SIMD, no
// threading, and **no performance claim of any kind**. Every choice here favours
// being obviously right over being fast, because the only thing this is for is
// deciding whether something else is wrong.
//
// Conventions it implements, from docs/conventions.md:
//
//   - Reversed-Z. Depth clears to 0.0, the projection maps near to 1.0 and far
//     towards 0.0, and the default comparison is greater-or-equal. Reversed-Z
//     bugs present as ordinary z-fighting, so getting this wrong here would make
//     the oracle agree with the bug.
//   - Infinite far plane, so there is no far clipping to implement - which is
//     one of the reasons reversed-Z was chosen.
//   - Linear colour throughout. sRGB encoding happens once, at output.

use std::fmt::Write as _;

/// Attributes carried per vertex and interpolated across a triangle.
///
/// Four is enough for a colour or a pair of texture coordinates plus two spare,
/// and a fixed count keeps the interpolation loop free of allocation.
pub const MAX_ATTRIBUTES: usize = 4;

/// A vertex in clip space, as a vertex shader would leave it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    /// Clip-space position, before the perspective divide.
    pub position: [f32; 4],
    pub attributes: [f32; MAX_ATTRIBUTES],
}

impl Vertex {
    pub fn new(position: [f32; 4], attributes: [f32; MAX_ATTRIBUTES]) -> Self {
        Self { position, attributes }
    }

    /// A vertex already in normalized device coordinates, with `w` of one.
    pub fn ndc(x: f32, y: f32, z: f32, attributes: [f32; MAX_ATTRIBUTES]) -> Self {
        Self { position: [x, y, z, 1.0], attributes }
    }
}

/// How a fragment's depth is compared against the buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthTest {
    Never,
    Always,
    /// Under reversed-Z this is "nearer than", because near is 1.0.
    Greater,
    /// The engine's default (docs/conventions.md).
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Equal,
}

impl DepthTest {
    fn passes(self, fragment: f32, stored: f32) -> bool {
        match self {
            DepthTest::Never => false,
            DepthTest::Always => true,
            DepthTest::Greater => fragment > stored,
            DepthTest::GreaterOrEqual => fragment >= stored,
            DepthTest::Less => fragment < stored,
            DepthTest::LessOrEqual => fragment <= stored,
            DepthTest::Equal => fragment == stored,
        }
    }
}

/// Which triangle facing is discarded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    None,
    Back,
    Front,
}

/// Fixed-function state for a draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RasterState {
    pub depth_test: DepthTest,
    pub depth_write: bool,
    pub cull: CullMode,
}

impl Default for RasterState {
    fn default() -> Self {
        Self {
            depth_test: DepthTest::GreaterOrEqual,
            depth_write: true,
            cull: CullMode::Back,
        }
    }
}

/// A colour and depth target.
#[derive(Debug, Clone, PartialEq)]
pub struct Framebuffer {
    width: u32,
    height: u32,
    /// Linear RGBA. Lighting math is always linear; sRGB happens at output.
    color: Vec<[f32; 4]>,
    /// Reversed-Z, so the cleared value is 0.0 and nearer is larger.
    depth: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = (width as usize) * (height as usize);
        Self {
            width,
            height,
            color: vec![[0.0, 0.0, 0.0, 0.0]; pixels],
            depth: vec![0.0; pixels],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Clears colour, and depth to 0.0 - the far value under reversed-Z.
    ///
    /// Not a parameter, because clearing depth to 1.0 is the reversed-Z mistake
    /// this whole convention exists to avoid, and offering it here would let the
    /// oracle be configured into agreeing with it.
    pub fn clear(&mut self, color: [f32; 4]) {
        self.color.fill(color);
        self.depth.fill(0.0);
    }

    pub fn color_at(&self, x: u32, y: u32) -> [f32; 4] {
        self.color[self.index(x, y)]
    }

    pub fn depth_at(&self, x: u32, y: u32) -> f32 {
        self.depth[self.index(x, y)]
    }

    fn index(&self, x: u32, y: u32) -> usize {
        assert!(x < self.width && y < self.height, "pixel out of bounds");
        (y as usize) * (self.width as usize) + (x as usize)
    }

    /// The number of pixels whose alpha is non-zero.
    ///
    /// Coverage is the property most geometry tests actually care about, and
    /// counting it is more robust than comparing colours that a shading change
    /// would move.
    pub fn covered_pixels(&self) -> usize {
        self.color.iter().filter(|pixel| pixel[3] > 0.0).count()
    }

    /// A character-per-pixel rendering, for golden-image comparison.
    ///
    /// Text rather than an image file, deliberately. A failing comparison shows
    /// which pixels differ, in the test output, without anyone opening a viewer
    /// or trusting an encoder - and a golden image nobody can read is a golden
    /// image nobody will update correctly.
    ///
    /// The ramp runs dark to light: space, then `.:-=+*#%@`.
    pub fn to_ascii(&self) -> String {
        const RAMP: &[u8] = b" .:-=+*#%@";
        let mut out = String::with_capacity(((self.width + 1) * self.height) as usize);
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.color_at(x, y);
                if pixel[3] <= 0.0 {
                    out.push(' ');
                    continue;
                }
                // Rec. 709 luma, on linear values. Not a display transform; just
                // a stable way to turn three channels into one character.
                let luma = 0.2126 * pixel[0] + 0.7152 * pixel[1] + 0.0722 * pixel[2];
                let clamped = luma.clamp(0.0, 1.0);
                let step = ((clamped * ((RAMP.len() - 1) as f32)).round() as usize)
                    .min(RAMP.len() - 1);
                out.push(RAMP[step] as char);
            }
            out.push('\n');
        }
        out
    }

    /// The same, for the depth buffer, so a depth-ordering failure is visible
    /// rather than inferred from a colour that happens to be wrong.
    pub fn depth_to_ascii(&self) -> String {
        const RAMP: &[u8] = b" .:-=+*#%@";
        let mut out = String::with_capacity(((self.width + 1) * self.height) as usize);
        for y in 0..self.height {
            for x in 0..self.width {
                let depth = self.depth_at(x, y).clamp(0.0, 1.0);
                let step =
                    ((depth * ((RAMP.len() - 1) as f32)).round() as usize).min(RAMP.len() - 1);
                out.push(RAMP[step] as char);
            }
            out.push('\n');
        }
        out
    }

    /// Binary PPM, so a human can actually look at a failure.
    ///
    /// PPM because its whole specification is a header and a byte per channel:
    /// no compression, no library, and nothing to get wrong that would make the
    /// picture lie about the buffer.
    ///
    /// This is the one place an sRGB transfer function is applied, matching the
    /// convention that encoding happens exactly once at output.
    pub fn to_ppm(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.color.len() * 3 + 32);
        let mut header = String::new();
        let _ = write!(header, "P6\n{} {}\n255\n", self.width, self.height);
        out.extend_from_slice(header.as_bytes());
        for pixel in &self.color {
            for channel in 0..3 {
                out.push(encode_srgb(pixel[channel]));
            }
        }
        out
    }
}

/// Linear to sRGB, from the published transfer function.
fn encode_srgb(linear: f32) -> u8 {
    let clamped = linear.clamp(0.0, 1.0);
    let encoded = if clamped <= 0.003_130_8 {
        12.92 * clamped
    } else {
        1.055 * clamped.powf(1.0 / 2.4) - 0.055
    };
    (encoded * 255.0 + 0.5) as u8
}

/// A vertex after clipping, still in clip space.
type ClipVertex = Vertex;

/// Draws indexed triangles.
///
/// `shade` receives the perspective-correctly interpolated attributes and
/// returns a linear RGBA colour. Passing it as a closure is what lets a
/// golden-image test exercise shading math without a shader compiler
/// (`BLOCK-003`).
pub fn draw_triangles<F>(
    target: &mut Framebuffer,
    state: RasterState,
    vertices: &[Vertex],
    indices: &[u32],
    shade: F,
) where
    F: Fn(&[f32; MAX_ATTRIBUTES]) -> [f32; 4],
{
    for triangle in indices.chunks_exact(3) {
        let corners = [
            vertices[triangle[0] as usize],
            vertices[triangle[1] as usize],
            vertices[triangle[2] as usize],
        ];
        for clipped in clip_to_frustum(&corners) {
            raster_one(target, state, &clipped, &shade);
        }
    }
}

/// Clips a triangle against the two planes that matter, returning zero or more
/// triangles.
///
/// Only two. With an infinite far plane there is no far clipping to do - one of
/// the reasons reversed-Z was chosen - and the side planes need none because the
/// bounding box is clamped to the viewport anyway.
///
/// The two are genuinely different conditions, and collapsing them into one
/// hides a case:
///
///   - `w > 0`, because the perspective divide needs it. A vertex behind the eye
///     has a negative w, and dividing by it flips the geometry inside out so it
///     reappears mirrored in front.
///   - `w - z >= 0`, the near plane. Under reversed-Z near maps to `z == w`, so
///     anything closer has `z > w`.
///
/// Note the `>=`: geometry lying exactly *on* the near plane is inside it. An
/// earlier version required a positive epsilon here and clipped it away, which
/// presents as surfaces vanishing at the moment you walk right up to them.
fn clip_to_frustum(triangle: &[Vertex; 3]) -> Vec<[ClipVertex; 3]> {
    // The divide needs a w that is not merely positive but usable.
    const MINIMUM_W: f32 = 1e-6;

    let polygon = vec![triangle[0], triangle[1], triangle[2]];
    let polygon = clip_against(polygon, |vertex| vertex.position[3] - MINIMUM_W);
    let polygon = clip_against(polygon, |vertex| vertex.position[3] - vertex.position[2]);

    let mut out = Vec::with_capacity(2);
    for index in 1..polygon.len().saturating_sub(1) {
        out.push([polygon[0], polygon[index], polygon[index + 1]]);
    }
    out
}

/// Sutherland-Hodgman against one plane, given a signed distance function.
///
/// A vertex is inside when its distance is zero or positive, so a polygon lying
/// exactly on the plane survives rather than disappearing.
fn clip_against<F>(polygon: Vec<Vertex>, distance: F) -> Vec<Vertex>
where
    F: Fn(&Vertex) -> f32,
{
    if polygon.is_empty() {
        return polygon;
    }
    let distances: Vec<f32> = polygon.iter().map(&distance).collect();
    if distances.iter().all(|value| *value >= 0.0) {
        return polygon;
    }
    if distances.iter().all(|value| *value < 0.0) {
        return Vec::new();
    }

    let mut out = Vec::with_capacity(polygon.len() + 1);
    for index in 0..polygon.len() {
        let next_index = (index + 1) % polygon.len();
        let current_distance = distances[index];
        let next_distance = distances[next_index];

        if current_distance >= 0.0 {
            out.push(polygon[index]);
        }
        if (current_distance >= 0.0) != (next_distance >= 0.0) {
            let t = current_distance / (current_distance - next_distance);
            out.push(lerp_vertex(&polygon[index], &polygon[next_index], t));
        }
    }
    out
}

fn lerp_vertex(a: &Vertex, b: &Vertex, t: f32) -> Vertex {
    let mut position = [0.0f32; 4];
    for index in 0..4 {
        position[index] = a.position[index] + (b.position[index] - a.position[index]) * t;
    }
    let mut attributes = [0.0f32; MAX_ATTRIBUTES];
    for index in 0..MAX_ATTRIBUTES {
        attributes[index] =
            a.attributes[index] + (b.attributes[index] - a.attributes[index]) * t;
    }
    Vertex { position, attributes }
}

/// A vertex projected into screen space.
#[derive(Debug, Clone, Copy)]
struct Projected {
    x: f32,
    y: f32,
    /// Normalized device depth, which is linear in screen space and so can be
    /// interpolated with plain barycentrics.
    z: f32,
    /// Reciprocal of clip w, for perspective-correct attribute interpolation.
    inverse_w: f32,
    attributes: [f32; MAX_ATTRIBUTES],
}

fn project(vertex: &Vertex, width: f32, height: f32) -> Projected {
    let w = vertex.position[3];
    let inverse_w = 1.0 / w;
    let ndc_x = vertex.position[0] * inverse_w;
    let ndc_y = vertex.position[1] * inverse_w;
    let ndc_z = vertex.position[2] * inverse_w;
    Projected {
        // The viewport transform. Y is flipped because NDC has +Y up and a
        // framebuffer has row zero at the top; forgetting it is the bug that
        // renders everything upside down and passes every coverage-count test.
        x: (ndc_x * 0.5 + 0.5) * width,
        y: (1.0 - (ndc_y * 0.5 + 0.5)) * height,
        z: ndc_z,
        inverse_w,
        attributes: vertex.attributes,
    }
}

/// Twice the signed area of a screen-space triangle.
fn edge(ax: f32, ay: f32, bx: f32, by: f32, px: f32, py: f32) -> f32 {
    (bx - ax) * (py - ay) - (by - ay) * (px - ax)
}

/// Whether a zero edge function counts as inside, under the top-left rule.
///
/// Without a fill rule, two triangles sharing an edge either both draw the
/// pixels on it - visible as a seam with blending, and as double shading cost
/// always - or neither does, leaving a crack. The rule breaks the tie
/// consistently: a shared edge belongs to exactly one of the two triangles.
fn is_top_left(ax: f32, ay: f32, bx: f32, by: f32) -> bool {
    let dx = bx - ax;
    let dy = by - ay;
    // In screen space, with y increasing downwards and vertices wound so the
    // signed area is positive: a top edge runs right-to-left with no vertical
    // extent, and a left edge runs downwards.
    (dy == 0.0 && dx < 0.0) || dy > 0.0
}

fn raster_one<F>(
    target: &mut Framebuffer,
    state: RasterState,
    triangle: &[Vertex; 3],
    shade: &F,
) where
    F: Fn(&[f32; MAX_ATTRIBUTES]) -> [f32; 4],
{
    let width = target.width as f32;
    let height = target.height as f32;

    let mut points = [
        project(&triangle[0], width, height),
        project(&triangle[1], width, height),
        project(&triangle[2], width, height),
    ];

    let area = edge(points[0].x, points[0].y, points[1].x, points[1].y, points[2].x, points[2].y);

    // A triangle with no area covers nothing. Not an error: clipping and
    // projection legitimately produce them, and dividing by this area is the
    // next thing that would happen.
    if area == 0.0 {
        return;
    }

    // Right-handed, counter-clockwise-front geometry becomes clockwise in screen
    // space once Y is flipped, so a front face has negative signed area here.
    let front_facing = area < 0.0;
    match state.cull {
        CullMode::Back if !front_facing => return,
        CullMode::Front if front_facing => return,
        _ => {}
    }

    // Normalise the winding so the inside test and the fill rule have one
    // orientation to reason about. Swapping two vertices flips the sign while
    // preserving which edges are which, where negating the edge functions would
    // silently invert the top-left rule.
    let area = if area < 0.0 {
        points.swap(1, 2);
        -area
    } else {
        area
    };

    let min_x = points.iter().fold(f32::MAX, |acc, p| acc.min(p.x)).floor().max(0.0) as u32;
    let max_x =
        (points.iter().fold(f32::MIN, |acc, p| acc.max(p.x)).ceil()).min(width) as u32;
    let min_y = points.iter().fold(f32::MAX, |acc, p| acc.min(p.y)).floor().max(0.0) as u32;
    let max_y =
        (points.iter().fold(f32::MIN, |acc, p| acc.max(p.y)).ceil()).min(height) as u32;

    let inverse_area = 1.0 / area;

    for y in min_y..max_y {
        for x in min_x..max_x {
            // Pixel centres, not corners. Sampling at the corner shifts the
            // whole image half a pixel, which every coverage count survives.
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;

            let w0 = edge(points[1].x, points[1].y, points[2].x, points[2].y, px, py);
            let w1 = edge(points[2].x, points[2].y, points[0].x, points[0].y, px, py);
            let w2 = edge(points[0].x, points[0].y, points[1].x, points[1].y, px, py);

            let accept = |value: f32, ax: f32, ay: f32, bx: f32, by: f32| {
                value > 0.0 || (value == 0.0 && is_top_left(ax, ay, bx, by))
            };

            if !accept(w0, points[1].x, points[1].y, points[2].x, points[2].y)
                || !accept(w1, points[2].x, points[2].y, points[0].x, points[0].y)
                || !accept(w2, points[0].x, points[0].y, points[1].x, points[1].y)
            {
                continue;
            }

            let b0 = w0 * inverse_area;
            let b1 = w1 * inverse_area;
            let b2 = w2 * inverse_area;

            let depth = b0 * points[0].z + b1 * points[1].z + b2 * points[2].z;
            let index = (y as usize) * (target.width as usize) + (x as usize);
            if !state.depth_test.passes(depth, target.depth[index]) {
                continue;
            }

            // Perspective correction. Barycentrics are linear in screen space,
            // but an attribute is linear in *world* space, so it has to be
            // interpolated divided by w and multiplied back. Skipping this is
            // the classic wobbling-texture bug, and it is invisible on any
            // triangle that happens to face the camera squarely.
            let inverse_w = b0 * points[0].inverse_w
                + b1 * points[1].inverse_w
                + b2 * points[2].inverse_w;
            let w = 1.0 / inverse_w;

            let mut attributes = [0.0f32; MAX_ATTRIBUTES];
            for slot in 0..MAX_ATTRIBUTES {
                let numerator = b0 * points[0].attributes[slot] * points[0].inverse_w
                    + b1 * points[1].attributes[slot] * points[1].inverse_w
                    + b2 * points[2].attributes[slot] * points[2].inverse_w;
                attributes[slot] = numerator * w;
            }

            target.color[index] = shade(&attributes);
            if state.depth_write {
                target.depth[index] = depth;
            }
        }
    }
}
