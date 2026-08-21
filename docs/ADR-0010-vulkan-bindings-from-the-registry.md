# ADR-0010 - Vulkan bindings generated from the Khronos registry

- **Status:** Accepted - **Date:** 2026-08-21
- **Builds on:** [ADR-0009](ADR-0009-hybrid-rust-cpp.md) (hybrid Rust and C++)
- **Requirements:** PN-RND-001, PN-RND-002, PN-OBJ-017, PN-OPS-001

## Context

[ADR-0009](ADR-0009-hybrid-rust-cpp.md) put the Vulkan RHI on the Rust side.
Vulkan is a C API: something has to declare roughly 560 types, 230 commands,
and several thousand constants in Rust before a single call can be made.

The usual answer is the `ash` crate. It is not available here. The Grandmaster
Prompt section 3.1 prohibits third-party dependencies, and
[DEPENDENCY_BOUNDARY.md](../DEPENDENCY_BOUNDARY.md) records that crates.io is
not an exception to that rule - a package manager is a package manager whether
it is vcpkg or cargo, and a permissive licence does not turn borrowed code into
custom-built code.

Three options were open:

1. **Hand-write the bindings.** Around 15,000 lines of declarations that must
   match a specification exactly. Every error is an ABI mismatch that either
   crashes or, worse, silently corrupts a structure a driver reads.
2. **Copy the C headers and translate them.** This is copying someone else's
   authored file, which is the thing the boundary exists to prevent.
3. **Generate from the machine-readable specification.**

## Decision

**Generate the bindings from `vk.xml`, the official Khronos registry, using a
generator written for this project: `rust/tools/vkgen`.**

The registry is a *specification*, not an implementation. Reading it to learn
what `VkImageLayout` means is the same act as reading the specification prose,
and it is the act section 3.3 of the Grandmaster Prompt explicitly permits.
Nothing Khronos wrote is copied into this repository: the generator reads the
registry's declarations and writes this project's own Rust from them.

This is *more* provenance-clean than a binding crate, not less. There is no
third-party code in the tree, and the one input is pinned by hash.

### The pin

`build_scripts/vulkan_registry_pin.txt` holds the URL, the SHA-256, and the
header version. It is read at runtime by the fetcher and compiled into `vkgen`
via `include_str!`, so the two cannot disagree about what is pinned.

`vk.xml` itself is **not committed**. It is another party's authored file, and
vendoring it would blur exactly the boundary this decision keeps. What is
committed is the pin, which is enough for any reviewer to re-fetch the same
bytes and confirm them.

`vkgen` verifies the hash itself rather than trusting the fetcher, using a
SHA-256 implemented from FIPS PUB 180-4 and checked against that standard's
published test vectors. Generating bindings from an unpinned registry requires
`--allow-unpinned-registry` and says so on stderr.

### The generated file is committed

`rust/crates/pn-vulkan-sys/src/generated.rs` is in the tree, so building the
engine does not require the registry or a network. `build_scripts/check_generated_bindings.py`
re-fetches, regenerates, and diffs it in CI. A hand edit to the generated file,
or a generator change nobody regenerated for, fails there rather than living in
the tree behind a provenance header that no longer describes it.

## Consequences for the emitted shape

### Enumerations are transparent newtypes, not Rust `enum`s

A Rust `enum` holding a value outside its declared set is undefined behaviour.
Drivers legitimately return values this registry does not list - a vendor
extension, or a result code added after these bindings were generated. Matching
on such a value in a real `enum` is UB, not a surprise.

```rust
#[repr(transparent)]
pub struct VkResult(pub i32);
impl VkResult {
    pub const VK_SUCCESS: Self = Self(0);
}
```

Call sites read the same; an unrecognised value is merely unrecognised.

### Callbacks are `Option`-wrapped

A bare `fn` is a non-null type in Rust, so zeroing a struct that holds one is
undefined behaviour - and `VkAllocationCallbacks` is routinely zeroed.
Registry-declared function pointers are therefore
`Option<unsafe extern "system" fn(..)>`, for which the zero bit pattern is
`None`.

### `extern "system"`, never `extern "C"`

`VKAPI_PTR` is `__stdcall` on 32-bit Windows and the platform C ABI everywhere
else. `extern "system"` reproduces exactly that. `extern "C"` would be a
stack-corrupting mismatch on precisely one supported target - the one least
likely to be tested first.

### Dispatch tables are split by loader

There are no link-time symbols worth using: the loader resolves everything
through `vkGetInstanceProcAddr` and `vkGetDeviceProcAddr`. Commands are sorted
into `EntryFns`, `InstanceFns`, and `DeviceFns` by the type of their first
parameter, and device-level commands are fetched from the device getter, which
returns a pointer into the driver rather than into the loader's dispatch
trampoline.

A missing entry is `None`, not a panic. Every one of these is legitimately
absent on some driver, and a table that refuses to load because an unused
command is missing is unusable.

### Scope

Core Vulkan 1.0 through 1.3, plus `VK_KHR_surface` and `VK_KHR_swapchain`. The
scope is a parameter of the generator and is printed in the generated file's
header. Widening it is a one-line change and a regenerated diff.

## Status of what this produces

Per section 4 of the Grandmaster Prompt, stated exactly:

| Claim | State |
|---|---|
| Registry is parsed and modelled | VERIFIED - 26 generator tests, plus 15 over the emitted bindings |
| Emitted bindings compile | VERIFIED - `cargo build -p pn-vulkan-sys`, no warnings |
| Constant values match the specification | VERIFIED - asserted against published values, not against the generator's own output |
| Struct layouts match a real driver's | **NOT VERIFIED** - no Vulkan implementation is present in this environment (BLOCK-002, BLOCK-004) |
| Any Vulkan call has been made | **NOT_STARTED** - nothing here calls Vulkan yet |

The layout tests check what C would produce for these declarations. They cannot
check what a driver actually expects, and this record does not pretend
otherwise.

## Alternatives rejected

| Option | Why not |
|---|---|
| `ash` or any binding crate | Prohibited by section 3.1; crates.io is not an exception |
| Hand-written bindings | 15,000 lines where each error is a silent ABI mismatch |
| Translating the C headers | Copying an authored file, which is the boundary violation itself |
| Generating at build time | Would make every build depend on the registry and a network |
