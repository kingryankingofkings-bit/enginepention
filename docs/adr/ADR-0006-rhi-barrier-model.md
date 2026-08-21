# ADR-0006 — RHI barrier and resource-state model

- **Status:** Accepted · **Date:** 2026-08-21 · **Requirements:** PN-RND-003, PN-RND-005 · **Design row:** ORIG-001 · **Source:** S-001, S-002

## Context

The engine must express GPU synchronization portably across Direct3D 12 and
Vulkan, and must derive barriers automatically because hand-written barriers stop
being maintainable at roughly ten passes and fail in vendor-specific ways
(RSK-M01, and the failure modes are stated outright in S-001).

The D3D12 Enhanced Barriers specification (S-001, retrieved in full) decomposes
the legacy monolithic barrier into three independently controlled concerns:
synchronization scope, memory access visibility, and texture layout. It defines
three barrier kinds — texture, buffer, and global — and states that buffers have
no transitionable layout, that access transitions perform no synchronization,
and that layout transitions must be compatible with the queue type performing
them. Vulkan's synchronization model separates the same three concerns.

Two independent explicit APIs converging on the same decomposition is strong
evidence that the decomposition is intrinsic to the problem rather than a
vendor artefact.

## Alternatives

**A. A single combined resource-state enum, legacy-D3D12 style.** Rejected on
the specification's own account of its failure modes: excessive synchronization
latency, excessive cache flushes, and implicit state promotion and decay that
S-001 describes as "a major source of developer confusion". Adopting a model
whose author documents why it was replaced would be a poor decision.

**B. Expose each backend's native barrier vocabulary and translate at the call
site.** Rejected: it puts API-specific reasoning into pass code, violating
ADR-0004 rule 1 in substance.

**C. Three independent axes — sync scope, access mask, layout — with barriers
derived by the render graph.** Chosen.

## Decision

The RHI barrier primitive carries three orthogonal axes:

| Axis | Meaning | Buffers |
|---|---|---|
| **Sync scope** | Which pipeline stages must complete before, and are blocked after | applies |
| **Access** | Which memory accesses must be made visible | applies |
| **Layout** | Texture memory arrangement | **not applicable** — buffers have no layout |

Three barrier kinds are represented: texture (all three axes), buffer (sync and
access), and global (sync and access across resources in a queue, and explicitly
incapable of expressing a layout change).

**Passes never emit barriers.** A pass declares its resource *intents*. The
render graph computes last-writer and next-reader relationships over the pass
DAG, derives the minimal barrier set, and lowers it to the active backend.

Where the two APIs' rules differ, the engine adopts the **stricter** rule, so
that code validated on one is valid on the other. Queue-type layout
compatibility is enforced by the graph at pass-assignment time, not discovered
at submission.

## Reasons

1. Both target APIs already separate these concerns; an abstraction that merges
   them would have to reconstruct information it discarded.
2. Derivation over the DAG is where the engineering value is, and it is
   independent of both APIs.
3. It gives the reference backend (ADR-0005) something meaningful to validate.

## Consequences

- The render graph becomes load-bearing from the first pass, not the tenth.
  Deliberate: retrofitting it is the expensive direction.
- The engine's state vocabulary is defined by what the graph must reason about.
  It is deliberately **not** a renaming of either API's enum — that would be the
  rebranding §3.1 prohibits and would inherit one vendor's quirks.
- Access masks must be narrow. S-001 notes that a blanket "common access"
  before-state implies all write accesses and causes unintended cache flushes;
  the engine therefore requires passes to declare narrow intents, and a blanket
  intent is a diagnostic.

## Risks

| Risk | Mitigation |
|---|---|
| The model is subtly D3D12-shaped | Cross-checked against Vulkan's model (S-002) before implementation; the reference backend is a third consumer of the interface |
| Derived barriers are over-conservative and cost performance | Timeline output is inspectable and asserted in tests; measurement deferred to hardware, never estimated |
| Derivation has a hazard bug that hardware would catch but the reference backend does not | Reference backend enforces the stricter rule set and *rejects* hazardous sequences; hardware debug layers remain required before `VERIFIED` |

## Validation plan

Hand-computed expected barrier timelines for a set of known graphs, asserted
exactly. A deliberately hazardous pass sequence must be rejected. On hardware,
the debug layer with synchronization validation must be silent — that is the
evidence required before any of this is `VERIFIED` rather than
`IMPLEMENTED_UNVERIFIED`.

## Reversal cost

**Moderate.** Confined to the RHI and render graph modules by ADR-0004, but
every pass declares intents in this vocabulary, so a vocabulary change touches
every pass declaration.
