# Engine Conventions

**Binding.** Every item here is cheap to decide once and expensive to
renegotiate. Where a choice is arbitrary the point is consistency, not
correctness — but the arbitrary choice is written down, because the failure mode
is two subsystems each assuming a different one.

Each convention names the test that enforces it. A convention with no test is a
suggestion.

## Coordinate system and handedness

**Right-handed, Y-up, −Z forward.** Chosen to match glTF, the import format the
asset pipeline will actually use (PN-AST-005). Every conversion avoided at
import is a class of bug never debugged.

- +X right, +Y up, −Z forward (into the screen)
- Rotations counter-clockwise viewed from the positive axis toward the origin
- Camera Euler order: yaw (Y) → pitch (X) → roll (Z)

*Enforced by:* a test asserting `cross(+X, +Y) == +Z`. Catches a whole family of
mirrored-normal and inverted-culling bugs at the cheapest possible moment.

## Units and scale

- **1 unit = 1 metre.** Physics, attenuation falloff, and depth precision all
  carry implicit scale assumptions; metres is what they assume.
- Mass in kilograms, time in seconds, angles in **radians** internally. Degrees
  exist only in editor UI and human-facing serialized text.
- **Photometric light units**, not arbitrary multipliers: lux for directional,
  lumens for punctual, nits (cd/m²) for emissive. Physical units are what make
  auto-exposure and tonemapping behave without per-scene hand-tuning.

*Enforced by:* unit-tagged types in `pn::math` where the cost is zero, and
round-trip tests on the degree/radian boundary.

## Large-world coordinates

**World space is `f64`. GPU space is camera-relative `f32`.** (PN-WLD-001)

`f32` has a 24-bit mantissa, so its ULP at magnitude *v* is about
`2^(floor(log2 v) − 23)`:

| Distance from origin | f32 ULP | Practical result |
|---|---|---|
| 1 km | ~0.06 mm | fine |
| 10 km | ~1 mm | fine |
| 100 km | ~8 mm | visible vertex swim, crawling shadows |
| 1000 km | ~6 cm | unusable |

Precision loss compounds through matrix concatenation, so problems appear
earlier than the table suggests — often by 20–30 km.

**Camera-relative rendering.** Each frame, subtract the camera's `f64` world
position from each object's `f64` world position on the CPU, yielding a small
`f32` offset. Build the view matrix with the camera at the origin. The GPU then
only ever sees coordinates near zero, where `f32` is dense.

Consequences, each a silent bug if missed:

- The view matrix has **no translation** — rotation only.
- Anything caching a world-space GPU value across frames (GI probes, temporal
  history, acceleration structures) must be rebased when the camera moves, or
  must store its own origin alongside the data.
- Shadow cascade matrices are built in camera-relative space too.
- Physics runs in `f64` world space or in per-cell local space with an `f64` cell
  origin. **Never** in render space.

*Enforced by:* a precision test asserting bounded round-trip error at 500 km from
the origin (PN-WLD-001's acceptance criterion), plus a type-level distinction
between world and render position so the two cannot be mixed silently.

## Depth: reversed-Z

- 32-bit float depth format. **Never** a 24-bit UNORM depth format — reversed-Z
  works because float density near zero is high, and a UNORM format is uniformly
  spaced, discarding the entire benefit.
- Clear depth to **0.0**, not 1.0
- Compare with greater-or-equal
- Projection maps near → 1.0, far → 0.0
- **Infinite far plane** — removes far-plane clipping entirely and costs no
  precision under reversed-Z

*Enforced by:* a test asserting a point at the near plane maps to `z/w == 1.0`
and a very distant point approaches `0.0`. Reversed-Z bugs present as ordinary
z-fighting, so an explicit test saves real time.

## Colour and transfer functions

- **Lighting math is always linear.** No exceptions.
- Colour textures (albedo, emissive, some UI): sRGB format, hardware decodes on
  sample.
- Data textures (normal, roughness, metallic, AO, masks, heightmaps, flow):
  UNORM. Sampling these through an sRGB view is a bug that presents as
  "roughness looks wrong in the mid-tones" and is very hard to see by eye.
- Working space: linear sRGB / Rec.709 primaries. Wide gamut only when genuinely
  targeting HDR displays.
- HDR render targets: 16-bit float per channel.
- **sRGB encode happens exactly once**, in the final present pass.
- Normal maps: two-channel storage, Z reconstructed as `sqrt(1 − x² − y²)`.

*Enforced by:* a golden-image test that fails on a double-encode, and a
debug-build assertion that a data texture is never created with an sRGB view.

## Matrix storage

**Column-major**, element at `m[col * 4 + row]`, matching the shader-language
convention so a matrix can be copied into a uniform buffer with no transpose.
A transpose hidden in an upload path only manifests under non-uniform scale or
a non-symmetric projection, which means it survives months of testing.

Normals need the **inverse transpose** of the upper-left 3×3 under non-uniform
scale. Computed once per object on the CPU and uploaded — doing it per-vertex is
wasted ALU.

## Time

- **Fixed timestep for simulation** (physics, gameplay): 1/60 s with an
  accumulator, render transforms interpolated between the two most recent
  simulation states. Variable-timestep physics is non-deterministic and makes
  every replay, network, and bug-report system unreliable (PN-PHY-016).
- **Variable timestep for rendering**, clamped (100 ms) so a stall or a debugger
  breakpoint does not explode the simulation on resume.
- A monotonic `frame_index` of 64 bits. Temporal techniques index off it; a
  wrapping or resetting counter produces flicker that is maddening to trace.

## Naming and identity

- Assets are identified by a **stable 128-bit GUID**, generated at import and
  never regenerated (PN-AST-001). Path-based identity breaks the moment someone
  renames a folder, and breaks *silently* in cooked builds.
- The GUID lives in a sidecar next to the source asset, committed to version
  control.
- Runtime handles are **generational** (index + generation) so a stale handle is
  detectably stale rather than silently aliasing a recycled slot (PN-PLT-011).
  This converts a class of use-after-free into an assertion.

## Code conventions

- `pn::` namespace throughout; nested namespace per module.
- Types `PascalCase`; functions and variables `snake_case`; constants
  `kPascalCase`; macros `PN_SCREAMING_CASE`.
- Every source file carries an authorship header naming its originating task and
  the ADR it implements (§3.3.8), checked mechanically by
  `build_scripts/check_authorship.py`.
- Fallible operations return `pn::Expected`. Exceptions are reserved for
  genuinely non-recoverable conditions; the tree must compile with
  `-fno-exceptions`.
- Warnings are errors.
