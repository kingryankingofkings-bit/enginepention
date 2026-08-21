# ADR-0005 — A non-hardware reference RHI backend

- **Status:** Accepted · **Date:** 2026-08-21 · **Requirements:** PN-RND-003, PN-RND-005, PN-OPS-007 · **Addresses:** RSK-F02, `BLOCK-002`

## Context

The development environment has no GPU (`BLOCK-002`), no Windows SDK
(`BLOCK-001`), and no shader compiler (`BLOCK-003`). Under §4, code that cannot
be executed cannot be `VERIFIED`. Taken naively, that would leave the entire
renderer — including the render graph, resource lifetime analysis, and barrier
derivation, none of which are inherently hardware-dependent — permanently
unverifiable.

That would be a mischaracterisation. The render graph's correctness is a
question about a directed acyclic graph, not about silicon.

## Alternatives

**A. Write the renderer blind; verify nothing until hardware exists.** Rejected:
it maximises the amount of unverified code written before the first feedback,
which RSK-F02 identifies as a high-severity risk.

**B. Depend on a software Vulkan implementation.** Rejected on two grounds: no
Vulkan loader is present, and more decisively, §3.1 prohibits third-party
middleware for rendering. A software rasterizer supplied by another party is
exactly that.

**C. An in-project reference backend implementing the RHI interface without
hardware.** Chosen.

## Decision

The RHI defines a backend interface. Three implementations are planned:

| Backend | Purpose | Status here |
|---|---|---|
| D3D12 | First production backend (§2) | Cannot compile here — `BLOCK-001` |
| Vulkan | Second backend (§2) | Cannot compile here — `BLOCK-004` |
| **Reference** | Validation and CI on hosts without a GPU | Buildable and testable **now** |

The reference backend is written entirely in-project. It:

- Records every command, barrier, and resource transition into an inspectable
  timeline, so barrier derivation can be asserted against expected output.
- Validates resource state transitions and reports hazards — a use of a resource
  in a state it was not transitioned into is a test failure.
- Performs simple correctness-oriented rasterization sufficient for golden-image
  tests of geometry, transforms, depth ordering, and shading math.
- Makes **no** performance claim. It is a correctness oracle, not a renderer.

## Reasons

1. It converts PN-RND-003, PN-RND-005, and PN-RND-006 from `BLOCKED` into
   genuinely testable requirements, because their content is graph and state
   reasoning rather than hardware behaviour.
2. It gives §12's golden-image testing a home on GPU-less CI, which is a standing
   need well beyond this environment.
3. A second implementation of the RHI interface is the cheapest available proof
   that the interface is not accidentally D3D12-shaped — it exercises ADR-0004's
   rule 1 in substance rather than in text.

## Consequences

- Real engineering cost for a backend that ships nothing. Accepted: it is test
  infrastructure, and §13.13 requires test infrastructure anyway.
- **A pass that works on the reference backend is not thereby proven to work on
  hardware.** The reference backend cannot detect vendor-specific behaviour,
  driver bugs, or performance characteristics. This limit is stated in every
  report that cites it, and the requirements it serves are scoped to the
  correctness claims it can actually support.
- It must not become a hiding place. A rendering feature verified *only* on the
  reference backend is labelled as such and is not `VERIFIED` for hardware.

## Risks

| Risk | Mitigation |
|---|---|
| Mistaken for hardware verification | Every report citing it states the limit; the traceability matrix distinguishes reference-verified from hardware-verified |
| Diverges from real API semantics and validates a wrong abstraction | Semantics derived from the specifications (S-001, S-002); the backend enforces the *stricter* of the two APIs' rules so passing here implies passing there |
| Grows into an unmaintained third renderer | Deliberately minimal; correctness only, no performance work, no feature parity goal |

## Validation plan

Self-tests: a deliberately hazardous pass sequence must be *rejected*, proving
the validator detects rather than assumes. Golden-image tests cover transform,
depth ordering, and shading math. Barrier derivation is asserted against
hand-computed expected timelines for known graphs.

## Reversal cost

**Low.** It is additive. Deleting it costs only the tests that depend on it.
