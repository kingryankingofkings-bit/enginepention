# Roadmap

The Master Execution Prompt defines a ten-phase program (Phase 0-9). The
`engine-architect` skill defines a thirteen-milestone rendering ladder (M0-M12).
These are not competing plans: the phases say **when a body of work is
finished**, the milestones say **what order rendering capability has to arrive
in**. This document maps one onto the other, because
[docs/INSTRUCTION_CONFLICTS.md](docs/INSTRUCTION_CONFLICTS.md) records that
mapping as adopted and a claim of that kind should be checkable.

Where they disagree, the phases win: they are level 2 in the authority order and
the skill is level 5.

## Current position

| Phase | State |
|---|---|
| 0 - Environment and repository truth | Complete |
| 1 - Research and normalized requirements | Complete, with a recorded shortfall ([GAP-001](RESEARCH_GAPS.md)) |
| 2 - Product specification and architecture | Complete |
| 3 - Toolchain bootstrap and Tier A core | In progress |
| 4-9 | Not started |

Within Phase 3, the Tier A pieces that exist are the build system, the test
framework, math, core, platform, and the job scheduler; plus, on the Rust side,
the generated Vulkan type and command surface
([ADR-0010](docs/adr/ADR-0010-vulkan-bindings-from-the-registry.md)), barrier
derivation (`pn-rhi`), the render graph (`pn-render-graph`), and a reference
backend that validates the derived barriers without hardware
(`pn-rhi-reference`). No instance has been created and no frame has been drawn,
so **M0 has not been reached** - and M2's barrier work landing before M0 is not
progress out of order, it is the part of M2 that a machine with no GPU can
actually verify.

## The mapping

| Milestone | Capability | Skill's exit criterion | Phase | State |
|---|---|---|---|---|
| M0 | Instance, device, swapchain, clear | Validation layers silent; clean resize and shutdown | 3 | NOT_STARTED - blocked from verification by `BLOCK-002` and `BLOCK-004` |
| M1 | Bindless textured mesh | One global descriptor array; pipeline cache on disk | 3 | NOT_STARTED |
| M2 | Render graph and depth prepass | Barriers fully derived; reversed-Z verified | 3 | Barriers derived (`pn-rhi`, 19 tests) and the graph that orders the passes exists (`pn-render-graph`, 20 tests); reversed-Z projection is implemented and tested. What is missing is a pass that draws anything, which needs M0 |
| M3 | glTF import, PBR opaque, IBL | Matches a reference render of the same scene | 3-4 | NOT_STARTED |
| M4 | Job system, ECS, parallel recording | Frame scales with core count; no data races | 3 | Job system built and TSan-clean; ECS and parallel recording NOT_STARTED. Scaling with core count is **not** demonstrated and cannot be on a four-core host |
| M5 | Clustered lights, cascaded shadows | 1000+ lights; stable shadows | 4-5 | NOT_STARTED |
| M6 | GPU-driven culling, meshlets, indirect | Draw count independent of CPU cost | 5 | NOT_STARTED |
| M7 | HDR post stack and temporal AA | Exposure, bloom, tone mapping, temporally stable | 5 | NOT_STARTED |
| M8 | Streaming world, camera-relative | 100 km traversal with no hitch and no jitter | 4-5 | Camera-relative render space is implemented and tested; streaming NOT_STARTED |
| M9 | Sky, atmosphere, volumetrics | Full day/night cycle with aerial perspective | 5 | NOT_STARTED |
| M10 | Global illumination and reflections | Indirect light responds to time of day | 5 | NOT_STARTED |
| M11 | Terrain, sandbox edits, vegetation, physics | Runtime terrain edits collide correctly | 4-6 | NOT_STARTED |
| M12 | Editor, cooking, profiling, CI | Cooked build ships; golden-image tests green | 3, 8, 9 | CI exists from the first commit; the rest NOT_STARTED. Golden-image tests need the reference backend's rasterizer, which is the half of [ADR-0005](docs/adr/ADR-0005-reference-backend.md) that is not written |

Two milestones span phases because the prompt's phases are broader than the
skill's: M8's camera-relative arithmetic is Tier A groundwork that belongs in
Phase 3, while its streaming belongs in Phase 4-5. Splitting them is not a
softening of either plan - the parts land in different phases because they
genuinely are different work.

## What is not cuttable

The skill argues that M2 (render graph) and M4 (job system) must not be cut
under schedule pressure, because M6's culling passes and M10's GI passes become
a hand-maintained barrier problem without the graph, and the CPU becomes the
frame's bottleneck around M8 without the job system.

That reasoning is adopted. It is also why the job scheduler was built during
Phase 3 rather than deferred: retrofitting threading into code that assumed
single-threaded mutable access is the largest rewrite an engine can undergo.

If work has to be cut, fidelity features are cut - M10, M11 - never M2 or M4.

## What this roadmap deliberately does not contain

**Dates, durations, and effort estimates.** Nothing here has been measured
against a schedule, and a date invented to fill a column is a number that will
be quoted back later as though it meant something.

**A promise that every milestone is reachable in this environment.** M0 onward
require a GPU (`BLOCK-002`) and, for validation-layer verification, a Vulkan SDK
(`BLOCK-004`). Code for them can be written and unit-tested here; their exit
criteria cannot be met here. See
[docs/ENVIRONMENT_BASELINE.md](docs/ENVIRONMENT_BASELINE.md).
