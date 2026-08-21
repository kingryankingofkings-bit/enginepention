# Performance Budgets

**Every figure in this document is a target. Not one is a measurement.**

No GPU exists in this environment (`BLOCK-002`), so no frame-time, bandwidth,
occupancy, or residency figure can be produced here at all - see
[GAP-004](RESEARCH_GAPS.md). Section 11 of the governing prompt is explicit that
a performance target may remain a target and is never rewritten as an
achievement, and this file is written so that the two can never be confused: the
Measured column stays empty until something is measured, and what fills it will
be a row in the results log below, not an edited target.

The budgets are also *chosen*, not derived from any authority. They are an
engineering allocation, and the reasoning for each is stated so that a future
measurement can argue with it.

## Frame budget

Two refresh targets, because a sandbox that only meets one is a sandbox with one
supported configuration:

| Target | Frame period | Basis |
|---|---|---|
| 60 Hz | 16.67 ms | The desktop default this engine is designed around |
| 30 Hz | 33.33 ms | The quality-tier floor; below this, camera motion stops feeling continuous |

The periods are arithmetic, not claims. Everything below allocates the 60 Hz
period; the 30 Hz tier is not simply double, because fixed per-frame costs do not
halve.

### CPU allocation at 60 Hz

| Budget | Target | Measured | Reasoning |
|---|---|---|---|
| Main/game thread | 6.0 ms | - | Simulation, gameplay, scripting. Must leave room for a full render-thread frame in parallel |
| Render thread | 4.0 ms | - | Graph compilation, barrier derivation, command recording submission |
| Worker pool aggregate | 8.0 ms/core | - | Culling, animation, physics, streaming decode |
| Worker utilization | >= 70% during a busy frame | - | Below this the job system is not paying for its complexity |
| Fixed-timestep simulation | 4 steps maximum per frame | - | Implemented and tested (`FixedTimestep`); the cap is what stops a hitch becoming a spiral |

The main and render thread targets sum to more than one frame period
deliberately: they are meant to overlap. If they cannot overlap, that is a
finding, not a budget violation.

### GPU allocation at 60 Hz

| Budget | Target | Measured | Reasoning |
|---|---|---|---|
| Total GPU frame | 14.0 ms | - | Leaves ~2.5 ms of headroom against the 16.67 ms period for driver and present variance |
| Depth prepass | 1.0 ms | - | |
| Opaque geometry | 4.0 ms | - | |
| Shadow rendering | 2.5 ms | - | All cascades and local lights combined |
| Lighting and GI | 3.0 ms | - | |
| Post-processing | 2.0 ms | - | Tone mapping, bloom, temporal reconstruction |
| Transparency and UI | 1.5 ms | - | |

### Counts

| Budget | Target | Measured | Reasoning |
|---|---|---|---|
| CPU-submitted draws/dispatches | <= 2,000 per frame | - | Above this the CPU cost of submission dominates, which is what GPU-driven rendering exists to remove |
| GPU-driven draws | Unbounded by CPU cost | - | The acceptance criterion for PN-RND-014 is that CPU frame time is flat from 1k to 100k objects |
| Visible primitives | <= 20M per frame | - | |
| Render graph passes | <= 200 per frame | - | Graph compilation is per-frame work; this bounds it |

### Memory and residency

| Budget | Target | Measured | Reasoning |
|---|---|---|---|
| Peak GPU memory, 8 GB tier | 6.5 GB | - | Leaves room for the compositor and other processes |
| Texture residency | Fixed pool, never grows | - | PN-RND-018's acceptance criterion is that residency stays inside a fixed budget during traversal |
| Geometry residency | Fixed pool, never grows | - | |
| Transient per-frame allocation | 0 bytes from the general heap | - | Frame-scoped work uses arenas; a general-heap allocation in the frame loop is a defect |
| Streaming pool | Fixed, eviction observable | - | PN-WLD-006 |

### Latency

| Budget | Target | Measured | Reasoning |
|---|---|---|---|
| Cell streaming latency | <= 500 ms from request to resident | - | At traversal speed this is the distance the prefetch horizon has to cover |
| Frame spike during traversal | No frame above 2x the period | - | PN-WLD-005's acceptance criterion is "no frame spike above budget"; this defines it |
| Level load | <= 5 s to interactive | - | |
| Save | <= 500 ms, non-blocking | - | |

### Tooling and pipeline

| Budget | Target | Measured | Reasoning |
|---|---|---|---|
| Editor input-to-response | <= 100 ms | - | Beyond this an editor feels like it is thinking rather than responding |
| Editor frame rate | >= 30 Hz while editing | - | |
| Asset cook, single asset | <= 2 s median | - | |
| Full cook, sample project | <= 10 min | - | |
| Server tick | 30 Hz sustained | - | |
| Bandwidth per client | <= 64 KB/s at target player count | - | PN-NET-006 |

## What must be recorded with every result

Section 11 requires all of the following, and a result missing any of them is
not a result:

- Commit or build identifier.
- Date and build configuration.
- CPU, GPU, RAM, storage, operating system, driver version, display resolution,
  and power mode.
- Scene, camera path, seed, duration, warm-up period, settings, and quality
  tier.
- Median, relevant percentiles, worst observed spike, and sample count.
- Capture method and where the raw artifact lives.
- Comparison baseline, and whether the change is statistically **and**
  practically meaningful. A 1% median improvement inside run-to-run variance is
  neither.

A single number with no configuration behind it is not evidence. Neither is a
best-case run.

## Results log

| Date | Commit | Budget | Target | Measured | Configuration | Artifact |
|---|---|---|---|---|---|---|
| _(none)_ | | | | | | |

Empty is the honest state. No performance measurement of any kind exists in this
repository, and none can be taken in this environment.

## Method

Optimization follows profiling, never precedes it. Correctness tests are
retained when an algorithm is replaced - a faster wrong answer is not an
improvement. Advanced GPU paths are capability-gated at runtime and their
fallbacks are validated, because a path that is only ever exercised on the
development machine is a path that is only ever correct there.
