# Originality Design Map

Per Master Execution Prompt §5.3: each desired capability, the observed user
need it answers, the original proposed approach, and how that approach avoids
implementation copying.

This file is the bridge required by §3.3.5 — observed capability becomes a
**neutral requirement** before any design exists. Rows are added as designs are
made, not in advance. An empty row would be a fiction.

## Naming policy

Original subsystems get original names (§3.3.7). Protected branding is never
used as a feature name; the explicitly forbidden set includes **Nanite, Lumen,
Blueprint, Niagara, MetaSounds, Chaos**, and this list is not exhaustive — the
rule is that no competitor's feature brand becomes a name here.

Equally forbidden by §3.1: rebranding standard technology under a misleading
proprietary name, or claiming independent invention without evidence. A cascaded
shadow map is called a cascaded shadow map. Original names are for genuinely
original subsystems, not for dressing up a standard technique.

### Names coined so far

| Name | What it is | Original? |
|---|---|---|
| **Pention** | The engine | Yes — coined for this project |
| `pn::` | C++ namespace | Yes |
| `pn::Expected` | Fallible-return type | Named descriptively; deliberately **not** named `expected`, to avoid implying `std::expected` semantics ([ADR-0002](docs/adr/ADR-0002-error-handling-and-expected.md)) |

---

## Design rows

### ORIG-001 — Portable barrier and resource-state model

| Field | Content |
|---|---|
| **Desired capability** | Correct, minimal GPU synchronization across an explicit graphics API, portable between at least two such APIs |
| **Observed user need** | Engines expose developers to hazard bugs that reproduce on one vendor's driver and not another's. Hand-written barriers stop being maintainable at roughly ten passes. The need is for barriers to be *derived* rather than written |
| **Source of the neutral requirement** | S-001 (D3D12 Enhanced Barriers specification, read in full) and the Vulkan synchronization model. The specification states the failure modes of the older monolithic model directly: excessive sync latency, excessive cache flushes, and resource state promotion/decay described as "a major source of developer confusion" |
| **Neutral requirement** | `PN-RND-003`: the barrier primitive shall express **synchronization scope**, **memory access visibility**, and **texture layout** as three independent axes, and shall represent buffer-only and global barriers where layout does not apply |
| **Original approach** | The render graph owns all barriers; no pass may emit one. Each pass declares resource *intents*. The graph computes last-writer/next-reader relationships over the pass DAG and derives the minimal barrier set, then lowers that to whichever backend is active. The engine's internal state vocabulary is defined by what the *graph* needs to reason about, and is deliberately not a renaming of either API's enum |
| **How copying is avoided** | The specification was consulted as prose and interface, which §3.2 permits as an open technical standard used as a behavioural reference. No implementation source was read — `DirectX-Graphics-Samples` was deliberately not opened. The derivation algorithm operates on the engine's own pass-DAG representation, which has no counterpart in either vendor's API |
| **Why this is not a rebrand** | The lowering step is thin and honest: engine intent → API barrier. The claimed original work is the *derivation over the DAG*, not the barrier concept, which is standard and is described as standard |

### ORIG-002 — Fallible-return convention

| Field | Content |
|---|---|
| **Desired capability** | One error convention across engine, tools, and sandbox, without exceptions, without allocation |
| **Observed user need** | Ignoring a failure must require a deliberate act; error propagation must not cost a branch at every call site |
| **Neutral requirement** | `PN-PLT-027` |
| **Original approach** | `pn::Expected<T, E>` with `[[nodiscard]]`, monadic composition, `PN_TRY` propagation, and an engine-specific `Error` carrying category, code, message, and `std::source_location` |
| **How copying is avoided** | Implemented from the interface described in the published C++ standard — permitted under §3.2 as an open technical specification. No standard-library implementation (libstdc++, libc++, MSVC STL) was consulted. The type carries engine-specific context the standard type has no place for, so it is not a reimplementation of the same thing |
| **Why this is not a rebrand** | It is not claimed as an invention. [ADR-0002](docs/adr/ADR-0002-error-handling-and-expected.md) states plainly that it exists because a measured compiler divergence made the standard type unusable across the supported toolchains |

---

## Rows still to be written

Every Tier C and Tier D capability in
[FEATURE_REQUIREMENTS_CATALOG.md](FEATURE_REQUIREMENTS_CATALOG.md) that requires
an original design needs a row here **before** its implementation begins —
notably `PN-RND-017` (cluster-based high-density geometry), `PN-RND-020`
(virtual texturing), `PN-RND-024` (upscaling), `PN-RND-027` (global illumination
and reflections), `PN-RND-029` (denoising), `PN-ANM-010` (data-driven pose
selection), and `PN-AUD-007` (spatial audio).

These are the requirements where the pull toward reproducing a known
implementation is strongest, which is exactly why §3.3.5's ordering — neutral
requirement first, design second — is enforced for them rather than assumed.
