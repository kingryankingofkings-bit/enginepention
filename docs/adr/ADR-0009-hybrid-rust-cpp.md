# ADR-0009 - Hybrid Rust and C++, and the FFI boundary contract

- **Status:** Accepted - **Date:** 2026-08-21
- **Amends:** [ADR-0001](ADR-0001-language-and-graphics-api.md) (language and first backend)
- **Requirements:** PN-PLT-018, PN-RND-001, PN-OBJ-017
- **Directed by:** the user, 2026-08-21, choosing a hybrid over both single-language options

## Context

[ADR-0001](ADR-0001-language-and-graphics-api.md) chose C++23 with Direct3D 12
first, resolving [CONF-001](../INSTRUCTION_CONFLICTS.md) in favour of the
execution prompt over the `engine-architect` skill's Rust and Vulkan. Five
modules were built on that basis and are green: `testing`, `math`, `core`,
`platform`, `jobs` - 152 tests, 12,442 assertions, ten build configurations.

The user has since directed a hybrid: **Rust for new work, Vulkan as the
graphics API, and the existing C++ retained.** Three options were put to them
with costs attached, including the two single-language ones and an explicit
recommendation against the hybrid; they chose the hybrid. That is their call to
make, and this record exists so the decision and its known costs are visible
rather than implicit.

## The objection, recorded rather than relitigated

The stated concern was: two toolchains, two build systems, and an FFI boundary
running through the engine's hottest paths, for no capability either language
lacks alone.

The first two are real and are simply paid. The third is the one that can
actually be engineered away, and most of this record is about doing that. A
hybrid fails when the boundary is *fine-grained* - a call per job, per
allocation, per vector operation. It succeeds when the boundary is **coarse**.

## Decision

### Ownership

| Side | Owns |
|---|---|
| **C++23** | `testing`, `math`, `core`, `platform`, `jobs` - already built, tested, and green. Not rewritten |
| **Rust** | New subsystems, starting with the Vulkan RHI and the render graph |

Neither side reimplements what the other owns. Two schedulers competing for the
same cores, or two diverging math libraries, would be worse than either
single-language option.

### The boundary contract

Binding, and enforced in CI:

1. **Coarse crossings only.** The boundary is crossed at most once per
   subsystem operation, never once per element. `parallel_for` crosses once per
   *chunk*, not once per index - which is the whole point of a grain size.
2. **Plain data only.** `#[repr(C)]` structs, primitives, and raw pointers to
   contiguous buffers with explicit lengths. No C++ templates, no `std::` types,
   no Rust generics, no trait objects cross.
3. **Explicit ownership.** Whichever side allocates, frees. No memory is
   allocated on one side and released on the other.
4. **No unwinding across the boundary, in either direction.** C++ compiles with
   exceptions disabled in the shipping configuration already; Rust callbacks
   invoked from C++ wrap their body in `catch_unwind` and convert a panic into
   a reported failure rather than letting it unwind into C++ frames, which is
   undefined behaviour.
5. **One header is the contract.** Every FFI entry point is declared in a C
   header under `engine/*/include/pn/*/ *_c_api.h`, and the Rust `-sys` crate
   mirrors it. Nothing crosses that is not in a header.

### Which direction calls which

**C++ does not call Rust except through callbacks it was handed.** Rust drives;
C++ provides services. This keeps the dependency direction of ADR-0004 intact
with Rust sitting above the existing C++ modules rather than beside them.

The one crossing that matters for performance is parallel work. Rust hands the
C++ scheduler a work count, a grain size, one function pointer, and one opaque
context; the scheduler fans it out across its existing workers. **One FFI call
per parallel operation, one callback per chunk** - so a 50,000-element loop at
grain 512 crosses about 98 times, not 50,000.

### Vulkan bindings are generated, not depended upon

The `engine-architect` skill specifies "Vulkan via the `ash` crate". **`ash`
cannot be used.** It is a package-manager dependency, which
[DEPENDENCY_BOUNDARY.md](../../DEPENDENCY_BOUNDARY.md) prohibits on the same
terms as any other third-party middleware - a permissive licence does not
satisfy the custom-build requirement, and that rule does not change because the
package manager is `cargo` rather than `vcpkg`.

What *is* permitted is the official specification. Khronos publishes the Vulkan
registry (`vk.xml`) and the official headers in its own repository, both
retrieved successfully from this environment (3.3 MB and 1.36 MB, HTTP 200).
Bindings are generated from that registry by a tool in this repository.

This is a real constraint discovered by applying the boundary rule to the
skill's advice, not a preference. It is recorded because the temptation to add
"just one crate" will recur.

**No crates.io dependencies, at all.** The Rust standard library is permitted as
the language runtime, exactly as libstdc++ is on the C++ side.

## Consequences

- Two build systems. CMake remains authoritative and drives cargo, so there is
  one entry point rather than two.
- Two test harnesses. Rust uses its own built-in one; C++ uses the in-project
  framework. Both report into CTest so CI has a single gate.
- The FFI surface is a maintenance burden proportional to its size, which is the
  argument for keeping it small. Every addition needs a header entry and a
  mirrored declaration, and that friction is deliberate.
- Rust cannot use the C++ `Arena`, `Expected`, or handle types - they are
  templates and C++ vocabulary types. Rust uses its own idioms natively. This is
  duplication of *concept*, not of *implementation*, and is accepted.
- Debugging spans two toolchains. A stack trace crossing the boundary is worse
  than one that does not.

## Risks

| Risk | Mitigation |
|---|---|
| The boundary creeps fine-grained and performance collapses | Rule 1 is a review gate; every new entry point states its crossing frequency in the header |
| A Rust panic unwinds into C++ frames | `catch_unwind` in every callback, tested with a deliberately panicking body |
| Someone adds a crates.io dependency | CI check fails the build on any non-empty `[dependencies]` in any crate |
| The two languages' math or scheduling diverge | Neither side reimplements what the other owns; Rust calls the C++ scheduler rather than starting its own |
| Generated Vulkan bindings drift from the registry | The registry version is pinned and recorded; regeneration is reproducible and diffable |

## Validation plan

- A vertical slice crossing the boundary in both directions, with tests on both
  sides: Rust closures executed on C++ worker threads, computing a result
  asserted against a serial reference.
- A test that a panicking Rust callback is contained rather than unwinding.
- A CI check that no crate declares an external dependency.
- The existing 152 C++ tests continue to pass unchanged - the hybrid must not
  disturb what already works.

## Reversal cost

**Asymmetric.** Adding more Rust is cheap. Removing the C++ later means
rewriting five tested modules; removing the Rust later means rewriting whatever
has been built above the boundary. The boundary contract is what keeps either
direction merely expensive rather than impossible.
