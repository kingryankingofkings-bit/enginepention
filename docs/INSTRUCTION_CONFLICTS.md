# Instruction Conflict Register

Master Execution Prompt §1 requires that when two requirements conflict, the
conflict is **recorded** and the higher-ranked requirement applied — and that no
requirement is ever silently discarded. This file is that record.

The authority order from §1 is:

1. Law, licenses, patents, security boundaries, factual integrity
2. Clean-room / original-implementation rules
3. Verified buildability, correctness, data safety, reproducibility
4. Required engine capabilities and acceptance gates
5. Architecture and implementation preferences
6. Convenience, speed, aesthetics, optional enhancements

§1 further states that **no invoked skill** may weaken levels 1–4.

---

## CONF-001 — Implementation language and first graphics backend

**Status:** Re-resolved by user direction, 2026-08-21. See
[ADR-0009](adr/ADR-0009-hybrid-rust-cpp.md); the original resolution is
[ADR-0001](adr/ADR-0001-language-and-graphics-api.md).

| Source | Requirement |
|---|---|
| Master Execution Prompt §2 | C++23; Direct3D 12 as first production backend; RHI boundary designed for a later Vulkan backend; Windows 11 x64 first; CMake + Ninja as external orchestration |
| `engine-architect` skill | Rust; Vulkan via the `ash` crate; Cargo workspace; `ash` confined to one crate |

**Rank of each:** The prompt's clause is a level-5 architecture preference *in
isolation*, but it is also load-bearing for level-3 (buildability against the
declared target) and level-4 (the capability inventory in §7.3 is written in
terms of an explicit-API RHI with a D3D12-first fallback matrix). The skill's
clause is level 5 only, and §1 explicitly denies any skill the power to weaken
levels 1–4.

**Original resolution (2026-08-21, superseded in part).** The Master Execution
Prompt governs: C++23, Direct3D 12 first, Vulkan second, CMake + Ninja.

**Current resolution (2026-08-21, by explicit user direction).** A hybrid. Rust
for new subsystems, Vulkan as the graphics API, and the five existing C++
modules retained rather than rewritten. Three options were put to the user with
their costs - Rust-only, C++-only-with-Vulkan, and the hybrid - together with an
explicit recommendation against the hybrid on the grounds of two toolchains and
an FFI boundary through hot paths. The user chose the hybrid.

This is a level-5 architecture preference under §1, and the user is the highest
authority on it. The engineering objection is recorded in
[ADR-0009](adr/ADR-0009-hybrid-rust-cpp.md) rather than relitigated, and that
record is mostly about engineering the objection away: the FFI boundary is
contractually coarse, so it does not sit in a hot path.

**One thing the user's direction cannot change**, because it sits at level 2 and
not level 5: the `engine-architect` skill's "Vulkan via the `ash` crate" remains
prohibited. `ash` is a package-manager dependency, and
[DEPENDENCY_BOUNDARY.md](../DEPENDENCY_BOUNDARY.md) prohibits those on the same
terms whether the package manager is cargo or vcpkg. Vulkan bindings are
generated from the official Khronos registry instead, which §3.2 permits.

**What is NOT discarded.** The skill's *language-neutral* engineering content
does not conflict with the prompt and is adopted in full, because it constitutes
level-3 correctness guidance rather than level-5 preference:

| Skill guidance | Adopted? | Where |
|---|---|---|
| World space `f64`, render space camera-relative `f32` | Yes | Prompt §7.4 requires exactly this; [ADR-0003](adr/ADR-0003-coordinates-units-precision.md) |
| Bindless / descriptor-indexed resource addressing from the first textured draw | Yes | Prompt §7.3 requires descriptor management + GPU-driven visibility |
| Render graph before the second pass exists | Yes | Prompt §7.3 requires a data-driven render graph |
| Reversed-Z, 32-bit float depth, infinite far plane | Yes | [ADR-0003](adr/ADR-0003-coordinates-units-precision.md) |
| Linear color throughout, sRGB encode exactly once at present | Yes | Prompt §7.3 requires an HDR linear pipeline |
| Fixed-timestep simulation with interpolated render transforms | Yes | Prompt §7.1 and §7.5 require exactly this |
| Milestone ordering M0→M12, with the job system and render graph as non-cuttable | Yes, mapped onto the prompt's Phase 0–9 program | [ROADMAP.md](../ROADMAP.md) |
| Crate/module boundary discipline; the graphics API named in exactly one module | Yes, restated for CMake targets | [ADR-0004](adr/ADR-0004-module-boundaries.md) |

The skill's Rust/Cargo/`ash` specifics are the only discarded portion, and they
are discarded **explicitly here**, not silently.

---

## CONF-002 — Declared target platform vs. available execution environment

**Status:** Open — permanent for this environment. Managed, not resolved.

| Source | Requirement |
|---|---|
| Master Execution Prompt §2 | Windows 11 x64 is the first fully supported development and runtime platform |
| Observed environment | Headless Ubuntu 24.04 Linux container, no GPU, no Windows SDK, no D3D12 headers, no shader compilers |

**Rank:** Level 3 (verified buildability) collides with a physical fact. §4
governs: *"If hardware, credentials, signing certificates, proprietary console
SDKs, or user action are required, isolate the exact blocked test and continue
all independent work."*

**Resolution:** Windows 11 x64 remains the declared first supported target and
is **not** downgraded. The Linux host is treated as a development and validation
environment. Blocked items are enumerated as `BLOCK-001`..`BLOCK-005` in
[ENVIRONMENT_BASELINE.md](ENVIRONMENT_BASELINE.md) §3, each with the smallest
exact external action required.

**Explicitly forbidden responses to this conflict**, per §4 and §16 — none of
these will be used:

- Declaring the renderer complete because it compiles as non-GPU code
- Substituting a browser, framework demo, or command-line simulation for the engine
- Lowering an acceptance gate because the hardware to test it is absent
- Presenting any estimate as a measurement

Every requirement blocked by this conflict is labelled `BLOCKED` in
[FEATURE_TRACEABILITY_MATRIX.md](../FEATURE_TRACEABILITY_MATRIX.md), never
`DEFERRED_BY_SCOPE` and never omitted.

---

## CONF-003 — `std::expected` availability across the two supported compilers

**Status:** Resolved. See [ADR-0002](adr/ADR-0002-error-handling-and-expected.md).

Measured, not assumed: `__cpp_lib_expected` is `202211` under `g++ -std=c++23`
and **absent** under `clang++ -std=c++23` against the same libstdc++. Level-3
(buildability across the declared toolchains) outranks the convenience of
using the standard type. Resolution: implement `pn::Expected` in `engine/core`.

---

## CONF-004 — Two different bars for `VERIFIED` on barrier derivation

**Status:** Resolved toward the stricter bar. PN-RND-003 stays
`IMPLEMENTED_UNVERIFIED`.

| Source | Bar for `VERIFIED` |
|---|---|
| [FEATURE_REQUIREMENTS_CATALOG.md](../FEATURE_REQUIREMENTS_CATALOG.md), PN-RND-003 | "Barrier derivation unit-tested on the reference backend without a GPU" |
| [ADR-0006](adr/ADR-0006-rhi-barrier-model.md), validation plan | "On hardware, the debug layer with synchronization validation must be silent - that is the evidence required before any of this is `VERIFIED`" |

The catalog's criterion is now met. `pn-rhi-reference` reconstructs each
resource's state from the derived barriers alone and checks the pass intents
against it, a compiled render graph is validated end to end through it, and the
backend is demonstrated to reject seven specific ways of getting a timeline
wrong - so a clean report is not the vacuous kind.

The ADR's criterion is not met and cannot be met here (`BLOCK-002`,
`BLOCK-004`).

**Rank.** Both documents are this project's own, so the authority order does not
separate them. What separates them is section 4: a state label must not
overstate the evidence. The two criteria are not alternatives - they check
different things. The reference backend establishes that the derivation is
self-consistent against an independent model of the same specification. It
cannot establish that the model matches a driver, and the model was written by
the same author as the thing it checks.

**Resolution.** Take the stricter bar. Marking PN-RND-003 `VERIFIED` on the
catalog's wording would be reading the weaker of two criteria this project set
for itself, which is the shape of exactly the failure section 4 prohibits.

The catalog wording is left as it stands rather than edited to match the ADR.
Rewriting an acceptance criterion after building against it is a habit worth not
starting, even when the rewrite would be in the stricter direction.

**What would resolve it.** A Vulkan or D3D12 backend running this frame on
hardware with the synchronization validation layer enabled and silent.
