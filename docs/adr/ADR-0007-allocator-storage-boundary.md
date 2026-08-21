# ADR-0007 - Allocators take storage; they do not acquire it

- **Status:** Accepted - **Date:** 2026-08-21 - **Requirements:** PN-PLT-005, PN-PLT-006, PN-PLT-007

## Context

The capability inventory places virtual memory, tagged allocators, arenas, and
pools together in the platform and runtime core. But ADR-0004 puts `core`
*below* `platform` in the dependency graph, and reserving address space is a
platform operation. Written naively, `core`'s allocators would need to call into
`platform`, which is an upward edge and a build failure.

The obvious fixes are both bad. Moving the allocators up into `platform` puts
general-purpose data structures in the OS-abstraction module. Moving virtual
memory down into `core` puts an OS dependency in the module that is supposed to
have none.

## Alternatives

**A. Allocators live in `platform`.** Rejected: an arena is not an OS concept,
and `core`'s containers will need arenas, which would then force `core` to
depend on `platform` anyway - the same cycle, one level up.

**B. Virtual memory lives in `core`, guarded by `#ifdef`.** Rejected: it puts
platform conditionals in the module every other module compiles, and it makes
`core` untestable without an OS-backed allocation path.

**C. Introduce a `memory` module between `core` and `platform`.** Reasonable,
and rejected only as premature - it buys one boundary and costs a module that
would contain two types.

**D. Allocators operate on caller-supplied storage.** Chosen.

## Decision

`core`'s allocators - `Arena`, `PoolAllocator` - are constructed over a
`std::span<std::byte>` that someone else owns. They never call the OS, never
grow, and never free their backing storage.

`platform`'s `VirtualMemory` acquires address space and hands out spans.
Composition happens at the point of use:

```
platform::VirtualMemory  ->  span<byte>  ->  core::Arena
```

## Reasons

1. It resolves the cycle by making the dependency run the correct way: the thing
   that knows about the OS hands memory to the thing that does not.
2. **Allocators become trivially testable.** A test supplies a stack array and
   exercises every path including exhaustion, with no OS involvement, no
   mocking, and no flakiness. Exhaustion in particular is awkward to provoke
   against real virtual memory and trivial against a 64-byte array.
3. It forces the ownership question to be answered explicitly at every use site,
   rather than being hidden inside an allocator that quietly reserves more.

## Consequences

- An arena cannot grow. That is a deliberate constraint, not an oversight: a
  growing arena invalidates outstanding pointers, and the requirement for fixed
  budgets appears throughout this project - notably the streaming pool, which
  must never grow to accommodate demand.
- Callers must size their budgets up front. This is the intended pressure.
- Allocation failure is a normal, returnable outcome rather than a fatal
  condition, which is what the out-of-memory handling requirement needs.

## Risks

| Risk | Mitigation |
|---|---|
| Callers over-provision because growth is unavailable | Tagged tracking reports high-water marks, so budgets are set from measurement |
| A caller frees storage while an arena still points into it | Arena is non-owning by design and documented as such; lifetime is the caller's, checked by ASan in CI |
| The boundary is re-litigated later when a growable allocator is wanted | Recorded here: the answer is a new allocator type, not growth bolted onto `Arena` |

## Validation plan

Allocator tests construct over stack and heap arrays and cover alignment,
exhaustion, reset, marker rewind, and pool recycling without touching the OS.
The platform module separately tests that reserve/commit produces usable spans.

## Reversal cost

**Low.** The allocators do not change if storage acquisition later moves; only
the construction sites do.
