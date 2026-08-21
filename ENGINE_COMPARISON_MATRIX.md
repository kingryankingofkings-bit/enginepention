# Engine Comparison Matrix

Engines as columns, neutral capabilities as rows, per Master Execution Prompt
§5.3.

> ## Provisional — read this before using any cell
>
> Every vendor documentation domain required by §5.1 is blocked by this
> environment's egress policy (see [GAP-001](RESEARCH_GAPS.md#gap-001--vendor-documentation-domains-are-blocked-by-egress-policy)).
> The cells below are derived from search-result snippets, which §5.1 ranks as
> tier-4 evidence usable only to locate a primary source.
>
> **Every cell is `NOT_VERIFIED_CURRENT`.** This matrix must be re-derived from
> primary sources before it supports any parity claim in Phase 9. It is
> published in this state because §4 requires visible gaps rather than a
> confident-looking table that was not actually verified.

## Evidence legend

| Tag | Meaning |
|---|---|
| `T1` | Official vendor documentation, retrieved and read |
| `T2` | Official standards body, retrieved and read |
| `T4?` | Secondary source snippet only — **not verified against a primary source** |
| `—` | No evidence gathered |

## Versions under comparison

| Engine | Version believed current on 2026-08-21 | Evidence |
|---|---|---|
| Unreal Engine | 5.7 | `T4?` |
| Unity | 6.3 LTS | `T4?` |
| Godot | 4.5.x | `T4?` |
| Open 3D Engine | Atom-based; release cadence unclear | `T4?` |
| CryEngine | 5.7 LTS public (2022); newer internal line not publicly released | `T4?` — and disputed, see [GAP-003](RESEARCH_GAPS.md#gap-003--cryengines-public-release-is-materially-behind-its-internal-one) |

## Capability rows

Capability names here are **neutral**, per §3.3.5 and §3.3.7 — vendor feature
brands are deliberately not used as row labels.

| Neutral capability | UE | Unity | Godot | O3DE | CryEngine | Evidence |
|---|---|---|---|---|---|---|
| Explicit-API backends (D3D12 / Vulkan / Metal) | yes | yes | Vulkan, D3D12, Metal | D3D12, Vulkan, Metal | D3D12, Vulkan | `T4?` |
| Data-driven render graph | yes | yes — URP and HDRP share one render-graph compiler as of 6.3 | partial | yes — data-driven pipeline | — | `T4?` |
| Virtualized high-density geometry | yes, production; foliage support added 5.7 | — | — | — | — | `T4?` |
| Many-light culling at scale | yes, beta in 5.7 | via pipeline | clustered; light culling tuned in 4.5 | yes | — | `T4?` |
| Layered/stack material system | yes, production in 5.7 | — | — | AZSL shader language | — | `T4?` |
| Dynamic global illumination | yes | real-time GI arriving in URP per 2026 strategy | SDFGI / VoxelGI | per-mesh and per-material GI | — | `T4?` |
| Hardware ray tracing | yes | yes | yes | yes, hardware-accelerated | yes | `T4?` |
| Procedural content generation graph | yes, production in 5.7 | — | — | — | — | `T4?` |
| Motion matching | yes, improved 5.7 | — | — | — | — | `T4?` |
| Heterogeneous volume rendering | beta in 5.7 | — | — | — | — | `T4?` |
| Stencil buffer exposed to user shaders | — | — | yes, added 4.5, Forward+ and Compatibility | — | — | `T4?` |
| Offline shader precompilation to cut runtime stutter | yes | yes | yes — shader baker added 4.5 | — | — | `T4?` |
| Anti-aliasing suite | TAA + upscalers | TAA + upscalers | SMAA added 4.5 | MSAA, SMAA, TAA | — | `T4?` |
| Editor screen-reader accessibility | — | — | experimental, added 4.5 | — | — | `T4?` |
| Large-world partitioning and streaming | yes | yes | — | — | — | `T4?` |
| Open licence / source availability | source-available | proprietary | MIT | Apache-2.0 | source-available | `T4?` |

Blank cells mean **no evidence was gathered**, not "absent". Recording absence of
evidence as absence of capability would be exactly the fabrication §16 forbids.

## How this matrix is used

Per §3.3.5, an observed capability becomes a **neutral requirement** before any
design work. The requirement carries forward into
[FEATURE_REQUIREMENTS_CATALOG.md](FEATURE_REQUIREMENTS_CATALOG.md); the original
approach chosen to satisfy it is recorded in
[ORIGINALITY_DESIGN_MAP.md](ORIGINALITY_DESIGN_MAP.md).

A capability may enter this matrix on documented behaviour alone even when its
internal method is unknown (§5.1, final paragraph). This project never infers an
undocumented proprietary implementation, and never treats a competitor's
observed behaviour as a design to copy.
