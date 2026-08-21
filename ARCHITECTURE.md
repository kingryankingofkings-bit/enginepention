# Architecture

Resolves the decisions §8 requires before large-scale implementation. Each
section states the decision and links the ADR carrying its alternatives,
consequences, risks, validation plan, and reversal cost.

**Status note.** This document describes the architecture the engine is being
built to. Except where a section says otherwise, none of it is implemented yet.
Per §13, planned behaviour is labelled planned and does not read as existing.

## 1. Process model

| Process | Role |
|---|---|
| Editor | Authoring. Hosts the runtime for play-in-editor. Excluded from shipping builds. |
| Runtime (game) | The standalone player. No editor code linked. |
| Dedicated server | Headless. No renderer, no window, no audio device. |
| Asset workers | Separate processes for cooking, so a crash in a third-party-format parser cannot take down the editor. |

Asset cooking is deliberately out-of-process. Parsers consume untrusted input,
and §13.13 requires an untrusted-asset policy; process isolation is the boundary
that makes a parser crash recoverable rather than fatal.

## 1a. Two languages, one boundary

The engine is a hybrid: C++23 owns `testing`, `math`, `core`, `platform`, and
`jobs` - all built and tested - and Rust owns new subsystems, starting with the
Vulkan RHI. See [ADR-0009](docs/adr/ADR-0009-hybrid-rust-cpp.md).

The design question in a hybrid is not *which* languages but *where the boundary
sits*. A boundary crossed per element is fatal to performance; one crossed per
subsystem operation costs nothing measurable. The contract is therefore:

- **Coarse crossings only.** Once per operation, plus once per chunk for
  parallel work - never once per element. Asserted by tests on both sides.
- **Plain data only.** `#[repr(C)]` structs, primitives, and pointer+length
  pairs. No templates, no generics, no `std::` or Rust vocabulary types.
- **The allocating side frees.**
- **Nothing unwinds across**, in either direction.
- **One C header is the contract**, mirrored by a Rust `-sys` crate.

Rust drives; C++ provides services. Crucially, **Rust does not start its own
scheduler** - two schedulers would oversubscribe the cores and each would make
the other's decisions wrong. Rust hands the C++ scheduler a count, a grain, one
function pointer, and one context, and it fans the work out across the workers
it already owns.

`cargo` never runs on its own: CMake builds the native libraries first and then
drives it, so there is one build entry point and one test gate.

## 2. Module boundaries and dependency direction

See [ADR-0004](docs/adr/ADR-0004-module-boundaries.md). One CMake target per
boundary; dependencies point strictly downward; two rules enforced in CI — the
graphics API appears in exactly one module, and no include crosses a forbidden
edge.

## 3. Coordinates, units, precision

See [ADR-0003](docs/adr/ADR-0003-coordinates-units-precision.md) and
[docs/conventions.md](docs/conventions.md). World space `f64`; render space
camera-relative `f32`; metres, kilograms, seconds, radians, photometric light
units; right-handed Y-up −Z-forward; reversed-Z with an infinite far plane;
column-major matrices; linear colour with a single sRGB encode at present.

## 4. Data-oriented runtime model and editor-to-runtime conversion

Authoring data, cooked data, runtime state, and replicated state are **distinct
representations**, not one type reused (PN-OBJ-008). Authoring data optimises
for editability and diffing; cooked data optimises for load speed and locality
and is deterministic (PN-AST-003); runtime state optimises for cache-friendly
iteration; replicated state optimises for bandwidth.

Conversion happens in one direction, at cook time. The editor never consumes
runtime state as authoring truth, because that is how an editor silently becomes
unable to reproduce its own output.

## 5. Frame pipeline

Simulation runs at a fixed 1/60 s with an accumulator; rendering runs at a
variable clamped rate and interpolates between the two most recent simulation
states (PN-PHY-016).

```
input sample ─► fixed-step simulation (0..n steps) ─► interpolate render state
                       │                                        │
                       ├─ gameplay ─ animation ─ physics        │
                       └─ audio update, network send            │
                                                                ▼
                                     visibility ─► render graph ─► submit ─► present
                                                        │
                                     streaming decisions run concurrently
```

Rendering reads an interpolated snapshot, never live simulation state. That is
what allows simulation and rendering to run at different rates without tearing
the data they share.

## 6. Job system, thread ownership, determinism

A work-stealing job graph (PN-PLT-013) owns all worker threads. Subsystems
submit jobs; they do not create threads. Deterministic zones — simulation
stepping in particular — are scheduled so their results do not depend on
completion order, which is what makes replay and networked reconciliation
possible.

Determinism is scoped explicitly rather than claimed globally: fixed timestep,
defined floating-point behaviour, no fast-math in simulation code, and seeded
RNG streams (PN-PLT-004). Limits are documented per PN-PHY-017.

## 7. Memory ownership and lifetime

Tagged allocators with per-subsystem attribution (PN-PLT-006). Frame and scratch
arenas reset per frame rather than freeing individually. Long-lived objects are
addressed by **generational handles** (PN-PLT-011), so a stale reference is
detectably stale rather than silently aliasing a recycled slot — converting a
class of use-after-free into an assertion.

## 8. Graphics hardware interface and render graph

See [ADR-0006](docs/adr/ADR-0006-rhi-barrier-model.md) and
[ADR-0005](docs/adr/ADR-0005-reference-backend.md). Barriers carry three
independent axes (sync scope, access, layout); passes declare intents and never
emit barriers; the render graph derives the minimal barrier set over the pass
DAG and lowers it to the active backend. Three backends are planned: D3D12
(first production), Vulkan (second), and an in-project reference backend for
validation on hosts without a GPU.

## 9. Asset identity, database, and cooking

Stable 128-bit GUIDs generated at import and never regenerated (PN-AST-001),
stored in a sidecar committed to version control. A content-addressed cache with
a dependency graph gives incremental, parallel, deterministic cooking
(PN-AST-002, PN-AST-003). Container formats are original, versioned,
bounds-checked, and checksummed (PN-AST-004).

## 10. World partition, streaming, persistence

A hierarchical spatial index over `f64` world space (PN-WLD-004). Cells stream
asynchronously with prefetch, prioritization, and cancellation, inside a **fixed**
memory budget — the pool never grows to accommodate demand (PN-WLD-006), because
an elastic budget is not a budget. Sandbox mutations persist per cell with
version migration (PN-WLD-019, PN-WLD-020).

## 11. Script VM, reflection bridge, security

A constrained deterministic language compiled to bytecode for an in-project VM
(PN-OBJ-012). Sandboxed by construction: no ambient filesystem, network, or
process access; explicit resource limits on instructions, memory, and recursion.
The reflection bridge (PN-PLT-024) is the only route between script and engine,
which is what makes the boundary auditable.

## 12. Network authority, replication, prediction, replay

Authoritative server (PN-NET-004). Clients predict and reconcile; the server
never trusts client-reported state (PN-NET-007). Interest management bounds
bandwidth at scale (PN-NET-006). Replay is snapshot-based, with determinism used
where it is genuinely available rather than assumed (PN-NET-013).

## 13. Error propagation, diagnostics, crash containment

`pn::Expected` is the single fallible-return convention
([ADR-0002](docs/adr/ADR-0002-error-handling-and-expected.md)); discarding a
result requires an explicit, greppable spelling. Recovery boundaries are named:
an asset load failure isolates one asset; a cook failure isolates one worker
process; a script fault isolates one script. Each boundary has a negative test —
a boundary with no test proving containment is an assumption.

## 14. Testability seams and benchmark instrumentation

Every subsystem is constructed with its dependencies injected rather than
reaching for globals, so it can be tested in isolation. The RHI's backend
interface is the largest such seam and is what makes ADR-0005 possible.
Instrumentation zones are compiled in under a feature flag so profiling builds
measure the shipping code path rather than a different one.

## 15. What is not yet decided

Honest gaps, recorded rather than implied:

- **Forward+ vs deferred vs hybrid** (PN-RND-008) is deliberately open. §7.3
  requires it be selected by *measured* use cases, and no measurement is
  possible without a GPU (`BLOCK-002`). Deciding it now on preference would be
  the exact failure §11 warns against.
- **Fixed-point world coordinates** were considered and deferred, not rejected
  outright ([ADR-0003](docs/adr/ADR-0003-coordinates-units-precision.md)).
  Revisit if networked determinism proves insufficient.
- **Rollback networking suitability** (PN-NET-008) requires criteria written
  before implementation.
