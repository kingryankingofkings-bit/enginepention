# Pention Engine

A clean-room, custom-built, next-generation 3D sandbox / free-roam game engine
and integrated editor. **Hybrid C++23 and Rust**, targeting Vulkan.

**Status: Phase 3 begun. A Tier A foundation builds and passes tests. There is
no renderer, no editor, and no runtime.**

This README will not describe a capability as existing until that capability is
`VERIFIED` with executed evidence.

## What this project is

An independently engineered engine. It studies the documented capabilities and
published techniques of leading engines and implements comparable capability
through original designs. It contains no engine forks, no middleware, and no
copied implementation code - see [DEPENDENCY_BOUNDARY.md](DEPENDENCY_BOUNDARY.md)
and [PROVENANCE_LEDGER.md](PROVENANCE_LEDGER.md) for the enforced boundary.

There are no third-party libraries of any kind, including for testing. A
permissive licence does not satisfy the custom-build requirement; the constraint
is on authorship, not on licensing.

## Current state

| Phase | State |
|---|---|
| 0 - Environment and repository truth | Complete |
| 1 - Research and normalized requirements | Complete, with a recorded shortfall ([GAP-001](RESEARCH_GAPS.md)) |
| 2 - Product specification and architecture | Complete |
| 3 - Toolchain bootstrap and Tier A core | In progress |
| 4-9 | Not started |

Of 214 catalogued requirements: **5 are `VERIFIED`**, 16 are
`IMPLEMENTED_UNVERIFIED`, 18 are `BLOCKED` by this environment, and the
remaining 175 are `NOT_STARTED`. Nothing is `DEFERRED_BY_SCOPE`. See
[FEATURE_TRACEABILITY_MATRIX.md](FEATURE_TRACEABILITY_MATRIX.md).

## What builds and passes today

162 C++ tests across 14 binaries and 127 Rust tests across 10 suites, green in
**10 build configurations** - GCC and Clang, Debug and Release, with exceptions
disabled, and under ASan+UBSan and TSan. Reproduced from a fresh checkout on
clean CI runners, not only on the machine it was written on.

| Module | Contents |
|---|---|
| `engine/testing` | The in-project test framework, which verifies its own failure reporting |
| `engine/math` | `f64` world space and camera-relative `f32` render space as distinct types, column-major matrices, reversed-Z infinite projection |
| `engine/core` | `pn::Expected` fallible-return type, allocation-free `Error`, generational handles, arena and pool allocators with tagged accounting |
| `engine/platform` | Virtual memory with reserve and commit separated, thread abstraction, fixed-timestep accumulator |
| `engine/jobs` | Bounded work-stealing deque and a job scheduler with dependencies, cancellation, and `parallel_for`, plus the C ABI that Rust uses |
| `rust/crates/pn-jobs` | Safe Rust over the C++ scheduler - Rust closures running on C++ worker threads, with panics contained at the boundary |
| `rust/tools/vkgen` | Reads the official Khronos Vulkan registry and writes this project's Rust bindings from it. Includes a zero-dependency XML pull parser and a SHA-256 implemented from FIPS 180-4 |
| `rust/crates/pn-vulkan-sys` | The generated Vulkan surface: 563 types and 229 commands for core 1.0-1.3 plus `VK_KHR_surface` and `VK_KHR_swapchain`. Nothing here is hand-written |
| `rust/crates/pn-rhi` | Sync, access, and arrangement as three independent axes, and the derivation that turns declared pass intents into a barrier timeline. Talks to no graphics API, which is why it is testable without a GPU |
| `rust/crates/pn-render-graph` | Pass DAG, culling, resource lifetimes, hazard validation, and a DOT visualization of passes and resources |
| `rust/crates/pn-rhi-reference` | A backend with no hardware behind it. It rebuilds resource state from the derived barriers and checks the passes against it, so a wrong barrier set is reported rather than drawn |

```sh
cmake -S . -B build -G Ninja && cmake --build build && ctest --test-dir build
```

CMake drives cargo; do not run `cargo` directly, as the Rust crates link against
libraries CMake produces. The Rust workspace declares **zero** external
dependencies - crates.io is prohibited on the same terms as any other package
manager, which is why the Vulkan bindings are generated from the official
Khronos registry rather than taken from `ash`. See
[ADR-0010](docs/adr/ADR-0010-vulkan-bindings-from-the-registry.md).

`vk.xml` is not committed; `build_scripts/vulkan_registry_pin.txt` pins its URL
and SHA-256 so any reviewer can re-fetch the same bytes, and CI re-derives the
committed bindings from it.

Full instructions: [BUILD.md](BUILD.md).

## Known blockers

The development environment is a headless Linux container with **no GPU**, no
Windows SDK, no Direct3D 12 headers, and no shader compilers, and its network
egress policy reaches only GitHub. The graphics backend cannot be compiled or
executed here, and vendor documentation could not be read for the competitive
research.

Tracked as `BLOCK-001` through `BLOCK-006` in
[docs/ENVIRONMENT_BASELINE.md](docs/ENVIRONMENT_BASELINE.md), each naming the
smallest external action that would clear it. Work that does not depend on them
proceeds.

## Governing documents

| Document | Purpose |
|---|---|
| [ARCHITECTURE.md](ARCHITECTURE.md) | Process model, module boundaries, frame pipeline, and what is deliberately still open |
| [ROADMAP.md](ROADMAP.md) | The ten-phase program mapped onto the M0-M12 rendering ladder, with the current position marked |
| [PERFORMANCE_BUDGETS.md](PERFORMANCE_BUDGETS.md) | Frame, memory, latency, and pipeline targets. Every figure is a target; the measured column is empty |
| [docs/conventions.md](docs/conventions.md) | Binding conventions, each naming the test that enforces it |
| [FEATURE_REQUIREMENTS_CATALOG.md](FEATURE_REQUIREMENTS_CATALOG.md) | 214 requirements with tier, dependencies, test method, and state |
| [FEATURE_TRACEABILITY_MATRIX.md](FEATURE_TRACEABILITY_MATRIX.md) | What is actually verified, and what is only implemented |
| [docs/INSTRUCTION_CONFLICTS.md](docs/INSTRUCTION_CONFLICTS.md) | Conflicts between instruction sources, and how each was resolved |
| [RESEARCH_GAPS.md](RESEARCH_GAPS.md) | What could not be established, and why |
| [TECHNIQUE_RISK_REGISTER.md](TECHNIQUE_RISK_REGISTER.md) | Legal, provenance, feasibility, hardware, and maintenance risk |
| [docs/adr/](docs/adr/) | Ten architecture decision records |

## A note on performance claims

There are none, and there will be none until there is hardware to measure on.
No figure in this repository is a measurement of rendering performance.
[PERFORMANCE_BUDGETS.md](PERFORMANCE_BUDGETS.md) states targets and keeps its
Measured column empty and its results log empty, which is the honest state. A
target is never rewritten as an achievement.
