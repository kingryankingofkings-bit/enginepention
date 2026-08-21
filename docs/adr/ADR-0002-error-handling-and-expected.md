# ADR-0002 — Error handling and the in-project `Expected` type

- **Status:** Accepted
- **Date:** 2026-08-21
- **Resolves:** [CONF-003](../INSTRUCTION_CONFLICTS.md#conf-003--stdexpected-availability-across-the-two-supported-compilers)

## Context

The engine needs one fallible-return convention. It must work identically under
both supported compilers, must not allocate, must not depend on exceptions being
enabled, and must be usable in the asset pipeline, the parsers, the RHI, and the
platform layer alike.

Measured on this host (evidence: `docs/evidence/environment-probe.txt`):

| Probe | `g++ -std=c++23` | `clang++ -std=c++23` |
|---|---|---|
| `__cpp_lib_expected` | `202211` | absent |

The standard type is therefore unavailable on one of the two compilers the
project has committed to supporting.

## Alternatives considered

**A. Use `std::expected` and drop Clang support.** Rejected: reduces toolchain
coverage to satisfy convenience, and Clang carries sanitizers and static
analysis the project depends on for a C++ codebase (ADR-0001's risk table).

**B. Use `std::expected` where available, fall back behind a macro.** Rejected:
two code paths with different conversion, comparison, and monadic-operation
semantics is a correctness hazard that surfaces as a portability bug months
later. The point of one convention is that it is one convention.

**C. Exceptions.** Rejected: the engine has hot paths, real-time budgets, and a
requirement (§13.13) to run with sanitizers and in constrained configurations.
Exceptions also propagate poorly across the C ABI boundary the platform layer
and any future scripting host use. They remain available for genuinely
exceptional, non-recoverable conditions only.

**D. Error codes with out-parameters.** Rejected: does not compose, and makes
ignoring a failure the default rather than a deliberate act.

**E. In-project `pn::Expected<T, E>`.** Chosen.

## Decision

Implement `pn::Expected<T, E>` in `engine/core`, with `pn::Error` as the default
error type carrying a category, a code, a static message, and a
`std::source_location` capture point.

Properties, all enforced by tests:

- Never allocates. Storage is a union of `T` and `E` plus a discriminant.
- Trivially destructible when both `T` and `E` are.
- `[[nodiscard]]` — discarding a fallible result is a compile-time diagnostic.
- Monadic composition (`map`, `and_then`, `or_else`) so error propagation does
  not require a branch at every call site.
- `PN_TRY(expr)` propagates a failure to the caller in one line.
- Independent of exceptions; the tree compiles with `-fno-exceptions`.

This is not a reimplementation of `std::expected` copied from a standard library
source. It is implemented from the interface described in the published C++
standard, which §3.2 permits as an open technical specification used as a
behavioural reference, and §3.1's prohibition on copied implementation code is
respected — no libstdc++, libc++, or Microsoft STL source was consulted.

## Reasons

1. One semantics across both compilers, verified by the same test suite compiled
   twice.
2. The engine controls the type, so it can carry engine-specific context —
   source location, error category, an asset GUID where relevant — that
   `std::expected` has no place for.
3. `-fno-exceptions` compatibility is a hard requirement for the scripting
   sandbox and the shipping configuration; the standard type does not remove
   the need for this work anyway.

## Consequences

- One more type in `core`, which every module compiles. It is header-only,
  templated, and small; ADR-0004's warning about `core` bloat applies and is
  accepted here because the alternative is per-module error conventions.
- Interop with any standard-library API returning `std::expected` requires an
  explicit adapter. Acceptable: no such API is used at the moment.
- The `PN_TRY` macro is a macro, with the usual hygiene caveats. It is the only
  control-flow macro the engine defines, and its expansion is tested.

## Risks

| Risk | Mitigation |
|---|---|
| Subtle divergence from `std::expected` semantics confuses future contributors | The type is deliberately *not* named `expected`; documented as an engine type with its own contract |
| Template bloat in compile time | Header kept minimal; no dependency on `<string>` or `<vector>` in the error path |
| `[[nodiscard]]` fatigue leads to reflexive `(void)` casts | Discarding requires the explicit `PN_IGNORE_RESULT` spelling, which is greppable in review |

## Validation plan

- Unit tests covering: value and error construction, move-only payloads,
  non-trivial destructor sequencing, monadic chaining, `PN_TRY` propagation
  across several frames, and correct `source_location` capture.
- The same test binary is compiled by GCC and Clang in CI; both must pass.
- A dedicated CI configuration compiles the tree with `-fno-exceptions` to prove
  the independence claim rather than assert it.

## Reversal cost

**Low to moderate.** If a future toolchain baseline makes `std::expected`
universally available, `pn::Expected` can become an alias only if its extra
context is dropped — which is the part worth keeping. More likely it stays.
Call sites use `PN_TRY` and the monadic operations, so they are insulated from
the underlying representation.
