# ADR-0003 — Coordinates, units, and precision policy

- **Status:** Accepted · **Date:** 2026-08-21 · **Requirements:** PN-WLD-001, PN-WLD-002, PN-PHY-016

## Context

§7.4 requires double-precision or partition-relative world coordinates, stable
physics/render conversion, and multiplayer-safe spatial addressing. The choice
determines the type of every transform in the engine, so it must precede the
first transform that exists.

`f32` carries a 24-bit mantissa. Its ULP reaches ~8 mm at 100 km and ~6 cm at
1000 km, and error compounds through matrix concatenation, so visible artefacts
typically begin around 20–30 km — well inside the range an open world needs.

## Alternatives

**A. `f32` throughout with periodic origin rebasing.** Simpler, and keeps one
float type. Rejected: the shift is a stop-the-world event invalidating every
cached spatial structure, it interacts badly with networked play, and it splits
physics islands that straddle the shift. It also converts a continuous precision
problem into a discrete correctness problem, which is harder to test.

**B. `f64` world space, camera-relative `f32` for rendering.** Chosen.

**C. Fixed-point world coordinates.** Genuinely attractive for determinism and
uniform precision. Rejected for now: it complicates every interaction with
floating-point physics and graphics APIs, and the determinism benefit is
achievable within the fixed-step policy without it. Revisit if networked
determinism proves insufficient — recorded so the option is not forgotten.

## Decision

World-space positions and transforms are `f64`. Rendering subtracts the camera's
`f64` position on the CPU and uploads `f32` offsets. The view matrix carries no
translation. Physics runs in `f64` world space or per-cell local space with an
`f64` origin, never in render space. Units are metres, kilograms, seconds,
radians, and photometric light units.

World-space and render-space position types are **distinct types**, not aliases,
so mixing them is a compile error rather than a precision bug.

## Consequences

- Transform storage roughly doubles for world-space data. Accepted: it is CPU-side
  and small relative to geometry and textures.
- Anything caching a world-space value on the GPU across frames must rebase or
  carry its own origin. This obligation is inherited by GI probes, temporal
  history buffers, and acceleration structures, and is written into each of
  those requirements rather than left as folklore.
- Shadow cascades are built in camera-relative space.

## Risks

| Risk | Mitigation |
|---|---|
| `f64` leaks into a hot GPU-facing path | Distinct types; conversion only at the documented boundary |
| A subsystem forgets to rebase a cached world value | Each affected requirement names rebasing in its acceptance criterion |
| Precision assumed rather than measured | Test asserts bounded round-trip error at 500 km |

## Validation plan

A precision test flying a synthetic camera to 500 km asserts bounded transform
round-trip error; a handedness test asserts `cross(+X, +Y) == +Z`; a reversed-Z
test asserts near → 1.0 and far → 0.0.

## Reversal cost

**Very high after Phase 3.** This is why it is decided before the first
transform exists rather than after.
