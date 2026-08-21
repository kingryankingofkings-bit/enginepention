# Environment Baseline — Phase 0 Record

Captured: 2026-08-21. Raw evidence: [`docs/evidence/environment-probe.txt`](evidence/environment-probe.txt).

This document records **confirmed facts** about the execution environment, the
**assumptions** derived from them, the **missing capabilities**, and the
**external blockers**. Per the Master Execution Prompt §4, nothing in this file
is estimated: every line corresponds to a command that was actually run and
whose output is captured in the evidence file.

## 1. Confirmed facts

| Property | Observed value |
|---|---|
| Operating system | Ubuntu 24.04.4 LTS, Linux 6.18.44 x86_64 |
| Logical CPUs | 4 |
| CPU | Intel Xeon @ 2.10 GHz |
| RAM | 15 GiB total, ~15 GiB available |
| Free disk | ~30 GiB on `/` |
| C++ compilers | GCC 13.3.0, Clang 18.1.3 |
| Build orchestration | CMake 3.28.3, Ninja 1.11.1, GNU Make 4.3 |
| VCS | git 2.43.0 |
| Scripting (tooling only) | Python 3.11.15 |

### C++23 support, measured

| Feature probe | `g++ -std=c++23` | `clang++ -std=c++23` |
|---|---|---|
| `__cplusplus` | `202100` | `202302` |
| `__cpp_lib_expected` | `202211` | **absent** |
| `__cpp_lib_span` | `202002` | `202002` |
| `__cpp_lib_source_location` | `201907` | `201907` |
| `__cpp_lib_bit_cast` | `201806` | `201806` |

Both compilers accept `-std=c++23`, but they do **not** agree on library
surface. `std::expected` is present under GCC and absent under Clang against
the same libstdc++. Any core error-handling type built on `std::expected`
would compile on one supported compiler and fail on the other.

**Consequence:** the engine's fallible-return type is implemented in-project
(`pn::Expected`) rather than aliased to `std::expected`. See
[ADR-0002](adr/ADR-0002-error-handling-and-expected.md). This is recorded as an
engineering constraint discovered by measurement, not a stylistic preference.

## 2. Missing capabilities

These were probed and are **absent**:

| Capability | Probe result | Impact |
|---|---|---|
| GPU device nodes (`/dev/dri`) | absent | No hardware graphics or compute of any kind |
| Direct3D 12 headers (`d3d12.h`) | absent | D3D12 backend cannot be compiled here |
| Windows SDK / MSVC (`cl.exe`, `fxc`) | absent | Windows target cannot be built here |
| Vulkan headers / loader / `vulkaninfo` | absent | Vulkan backend cannot be compiled here |
| Shader compilers (`dxc`, `glslc`, `spirv-val`) | absent | No HLSL→DXIL or GLSL→SPIR-V compilation |

The environment is a headless Linux container. It has **no GPU at all** — not a
weak GPU, not a software GPU with a driver, but no device node.

## 3. External blockers

Recorded per Master Execution Prompt §4 and §15. Each names the smallest exact
external action required. All independent work continues around them.

| ID | Blocked item | Smallest required external action |
|---|---|---|
| `BLOCK-001` | Compiling the Direct3D 12 RHI backend | A Windows 11 x64 machine with the Windows SDK and a supported MSVC or Clang toolchain |
| `BLOCK-002` | Executing any GPU workload; all GPU performance measurement | Any host exposing a GPU with a modern explicit graphics API driver |
| `BLOCK-003` | HLSL→DXIL shader compilation | `dxc` from the DirectX Shader Compiler release, on a host where it can run |
| `BLOCK-004` | Vulkan backend compilation and validation-layer runs | Vulkan SDK (headers, loader, validation layers) |
| `BLOCK-005` | Signed release artifacts | A code-signing certificate — `USER_ACTION_REQUIRED`, never simulated |

**What is *not* blocked.** The platform-agnostic majority of the engine —
memory, containers, math, handles, reflection, serialization, job system, ECS,
asset database, scene representation, physics, navigation, audio DSP, network
protocol, scripting VM — is ordinary portable C++23. It compiles, runs, and is
testable here today. The renderer's *hardware interface* is blocked; the
renderer's *structure* (render graph, resource lifetime analysis, barrier
derivation, pass scheduling) is testable against a non-hardware backend.

## 4. Assumptions

1. The Linux host is a **development and validation environment**, not the
   product's first supported platform. Windows 11 x64 remains the first
   supported runtime target per Master Execution Prompt §2.
2. Portability across GCC and Clang is a hard requirement from day one, because
   both are present and they already disagree (§1). This is cheaper to hold than
   to recover.
3. 4 cores is a *small* worker count. Job-system correctness tests that depend on
   contention must use randomized scheduling stress rather than relying on core
   count to expose races.
4. No measurement taken on this host may be presented as representative of the
   product's target hardware. Per §11, all performance records carry their
   hardware context; this host's context is "4-core Xeon, no GPU".

## 5. Repository state at Phase 0 entry

`git status` reported a repository with **zero commits** and an empty working
tree on branch `claude/wake-up-t2zl6x`; `git ls-remote origin` returned no refs.
There is no pre-existing user work in this repository, so the Phase 0 obligation
to preserve unrelated user work is satisfied vacuously — nothing was inspected
away, overwritten, or reset. The initial commit is therefore a clean provenance
origin.
