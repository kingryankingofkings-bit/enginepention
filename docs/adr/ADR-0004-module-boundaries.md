# ADR-0004 — Module boundaries and dependency direction

- **Status:** Accepted · **Date:** 2026-08-21 · **Requirements:** PN-PLT-018, RSK-M01

## Context

§8 requires resolved module boundaries and permitted dependency directions.
Module boundaries inside one large target are advisory — the compiler does not
enforce them, so they erode. The graphics API is the specific case that matters
most: if D3D12 types spread beyond one module, the Vulkan backend required by §2
becomes a rewrite rather than a port.

## Alternatives

**A. One monolithic engine target with directory conventions.** Fastest to
build, and the convention is unenforceable. Rejected — this is precisely how
boundaries erode.

**B. One CMake target per boundary, with dependency direction enforced by the
build graph and by a CI check.** Chosen.

**C. Dynamic modules with a plugin ABI from the start.** Rejected as premature;
the ABI-safe extension boundary is PN-OBJ-017, a Tier C concern.

## Decision

One CMake library target per boundary. Dependencies point strictly downward; a
cycle means a boundary is wrong and the boundary gets fixed, never papered over
with an interface.

```
                      app
                       │
    ┌────────┬─────────┼─────────┬────────┬────────┐
  editor   render    world    physics   anim    audio
    │        │         │         │        │        │
    │   rendergraph    │         │        │        │
    │        │         │         │        │        │
    │       rhi      assets      │        │        │
    │        │         │         │        │        │
    └────────┴───scene─┴─────────┴────────┴────────┘
                       │
                     jobs
                       │
                   platform
                       │
                     core
                       │
                     math
```

**Two rules, enforced mechanically rather than by convention:**

1. **The graphics API appears in exactly one module.** D3D12 and DXGI headers may
   be included only under `engine/render/rhi/`; Vulkan headers likewise when that
   backend exists. Any other module including them is a build failure.
2. **Dependencies point one way.** A CI check parses includes and fails on any
   edge that runs against the graph above.

`editor` may depend on anything; nothing depends on `editor`, and it is excluded
from shipping configurations.

## Consequences

- More CMake targets, so more build configuration. Accepted.
- The RHI must expose handle-based resources, never raw API types, or rule 1 is
  satisfied textually while being violated in substance.
- A future backend is a port confined to one module — which is the entire point,
  and the thing that keeps ADR-0001's backend ordering reversible.

## Risks

| Risk | Mitigation |
|---|---|
| Rule 1 satisfied textually but violated in spirit via leaked type aliases | RHI public headers reviewed for API-shaped types; handles are opaque |
| `core` accretes until every module recompiles on any change | `core` contents reviewed per addition; ADR-0002 accepted `Expected` there deliberately and on the record |
| CI check drifts out of date with new modules | The check reads the dependency graph from one declarative file, not a hard-coded list |

## Validation plan

`build_scripts/check_layering.py` runs in CI: it fails on an include that
crosses a forbidden edge, and on a graphics-API header included outside the RHI
module. A deliberately violating file is used to prove the check actually fails.

## Reversal cost

**Low.** Boundaries can be merged cheaply. Splitting them later is the expensive
direction, which is the argument for drawing them now.
