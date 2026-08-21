# ADR-0001 — Implementation language, first graphics backend, build orchestration

- **Status:** Accepted
- **Date:** 2026-08-21
- **Resolves:** [CONF-001](../INSTRUCTION_CONFLICTS.md#conf-001--implementation-language-and-first-graphics-backend)

## Context

Two instruction sources specify incompatible foundations. The Master Execution
Prompt §2 specifies C++23 with Direct3D 12 first and a Vulkan backend later,
orchestrated by CMake and Ninja. The invoked `engine-architect` skill specifies
Rust with Vulkan via `ash` in a Cargo workspace.

These cannot both hold. The choice propagates into every source file, the build
system, the test framework, the asset pipeline's output formats, and the shader
toolchain. It is the single most expensive decision in the project to revisit.

Master Execution Prompt §1 establishes an authority order and states that no
invoked skill may weaken its first four levels. The user's instruction
accompanying the prompt designated it as authoritative in explicit terms.

## Alternatives considered

**A. Rust + Vulkan + Cargo (the skill's position).** Rust's ownership model
eliminates the use-after-free and data-race classes that consume real time in a
C++ engine, and `cargo` enforces module boundaries the compiler actually checks.
Vulkan is cross-platform, so the Linux development host could in principle run
validation layers under a software implementation.

Rejected. It contradicts a level-3/4 requirement on the authority of a level-5
source, which §1 forbids. It also contradicts the prompt's own mandatory
repository layout in §13, which is rooted at `/CMakeLists.txt` with a `/cmake/`
directory — a layout that is meaningless in a Cargo workspace. The prompt is
internally consistent about C++; adopting Rust would require overriding §2 and
§13 together.

**B. C++23 + Vulkan first, D3D12 second.** Would let the Linux host compile the
graphics backend, converting `BLOCK-001` from a compile blocker into a runtime
blocker. Genuinely attractive for short-term verifiability.

Rejected, but narrowly. §2 names D3D12 as "the first production graphics
backend" and asks for the RHI boundary to be designed for "a later Vulkan
backend". Reversing that order to make the development container more convenient
is precisely the level-6 (convenience) reasoning that §1 ranks last. The
verifiability gain is also smaller than it appears: with no GPU and no Vulkan
loader present, a Vulkan backend would compile but could not execute, so it
would reach `IMPLEMENTED_UNVERIFIED` and stop — the same state D3D12 reaches.

**C. C++23 + D3D12 first, Vulkan second, per the prompt.** Chosen.

## Decision

- **Language:** C++23. Both GCC 13.3 and Clang 18.1.3 must compile the tree; see
  ADR-0002 for the consequence.
- **First production graphics backend:** Direct3D 12, targeting Windows 11 x64.
- **Second backend:** Vulkan. The RHI boundary is designed for it from the start
  rather than retrofitted.
- **Build orchestration:** CMake + Ninja, used strictly as external build-time
  tools. They are not engine components and no engine code depends on them at
  runtime.
- **Reference backend:** a third, non-hardware RHI backend is introduced so that
  render-graph correctness, resource lifetime analysis, and golden-image tests
  are executable on hosts without a GPU. See ADR-0005.

## Reasons

1. §1's authority order is explicit and the skill is explicitly denied the power
   to override levels 1–4.
2. The prompt's mandatory artifact layout (§13) is a CMake/C++ layout. Choosing
   Rust would put the project in violation of a second requirement to satisfy a
   preference.
3. D3D12 and Vulkan are close enough in model — explicit queues, command lists,
   descriptor heaps/sets, explicit barriers, manual memory residency — that an
   RHI designed honestly against one ports to the other. The ordering therefore
   costs little in architecture even though it costs verifiability here.

## Consequences

- The graphics backend cannot be compiled in the current environment
  (`BLOCK-001`, `BLOCK-003`). Work on it produces code that will be labelled
  `IMPLEMENTED_UNVERIFIED` until a Windows host is available. It will not be
  labelled `VERIFIED` on any other basis.
- The engine cannot rely on `std::expected`, `std::print`, or any other C++23
  library feature not present in *both* supported compilers. Feature use is
  gated on measured availability, not on the standard's promises.
- Memory safety is an engineering obligation rather than a language guarantee.
  This raises the required investment in sanitizers, bounds-checked containers
  in debug builds, and fuzzing of every parser — all of which §13.13 mandates
  regardless.
- The single-graphics-API-module rule from the skill is retained and enforced in
  CI against CMake targets instead of Cargo crates (ADR-0004).

## Risks

| Risk | Mitigation |
|---|---|
| The renderer is written blind and is wrong in ways only hardware reveals | Reference backend (ADR-0005) executes the graph and resource logic without a GPU; D3D12-specific code kept as thin as the abstraction permits |
| C++ memory-safety defects | ASan/UBSan/TSan in CI, in-project bounds-checked debug containers, mandatory fuzzing of parsers per §13.13 |
| GCC/Clang divergence discovered late | Both compilers build every commit in CI from the first commit, not added later |
| D3D12-shaped abstraction leaks make the Vulkan port a rewrite | RHI reviewed against both APIs' models before the first backend call is written; descriptor model designed bindless-first, which both support |

## Validation plan

- CI builds the full tree under GCC and Clang, both Debug and Release, on every
  commit. A compiler-specific failure is a build break, not a warning.
- A CI check asserts that D3D12 and Vulkan headers are named in exactly one
  module each (ADR-0004).
- When a Windows host becomes available, the first acceptance evidence required
  is a clean debug-layer run, matching the prompt's §12 standard for `VERIFIED`.

## Reversal cost

**Very high, and rising.** Reversing the language after Phase 3 means rewriting
every file. Reversing the backend order after the RHI has a D3D12 implementation
costs the backend module only — days, not months — provided ADR-0004's
single-module rule has actually held. That rule is what keeps this decision
reversible at all, which is why it is enforced mechanically rather than by
convention.
