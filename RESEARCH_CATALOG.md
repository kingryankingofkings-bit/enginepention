# Research Catalog

Annotated primary sources with retrieval dates and claim-level notes, per Master
Execution Prompt §5.3. Retrieval date for every entry below: **2026-08-21**.

**Read [RESEARCH_GAPS.md](RESEARCH_GAPS.md) alongside this file.** The execution
environment's egress policy blocks most vendor documentation domains, which
materially limits what could be verified. That limitation is characterised
precisely in the gaps file and is not papered over here.

## Source tiers used

Per §5.1 the priority order is (1) official vendor product documentation,
(2) official standards bodies, (3) primary peer-reviewed and conference
material, (4) credible secondary analysis *only to locate or clarify a primary
source*. Every claim below is tagged with the tier it actually came from.

---

## S-001 — D3D12 Enhanced Barriers specification

- **Title:** Enhanced Barriers
- **Publisher:** Microsoft, `microsoft/DirectX-Specs`
- **URL:** https://raw.githubusercontent.com/microsoft/DirectX-Specs/master/d3d/D3D12EnhancedBarriers.md
- **Retrieved:** 2026-08-21 — 201,304 bytes, HTTP 200
- **Tier:** 1 (official vendor specification, retrieved in full)
- **Local evidence:** `docs/evidence/` (see collection note at end of file)

**Information used.** The barrier model only. Specifically:

1. Enhanced Barriers decompose the legacy monolithic `ResourceBarrier` into
   three independently controlled concerns: **synchronization scope**
   (`SyncBefore`/`SyncAfter`), **memory access/cache visibility**
   (`AccessBefore`/`AccessAfter`), and **texture layout**
   (`LayoutBefore`/`LayoutAfter`).
2. Three barrier kinds exist: **texture** (sync + access + layout), **buffer**
   (sync + access; buffers have no transitionable layout), and **global**
   (sync + access across all resources in a queue; explicitly cannot express a
   layout change).
3. Layout and access transitions must be compatible with the queue type
   performing the barrier — e.g. a compute queue cannot transition into or out
   of `D3D12_BARRIER_LAYOUT_RENDER_TARGET`.
4. Access transitions perform **no** synchronization; synchronization must be
   expressed separately through the sync scopes. The spec is explicit that these
   are orthogonal.
5. `ACCESS_COMMON` as `AccessBefore` implies the set of all write accesses and
   is "strongly discouraged" — it triggers a debug-layer warning because it
   causes unintended cache flushes.
6. Resource state **promotion and decay** — described in the spec as "a major
   source of developer confusion" — do not exist under enhanced barriers.
7. After a sequence of barriers, a subresource's layout is the final
   `LayoutAfter` in the sequence, giving well-defined barrier ordering.
8. Buffers and simultaneous-access textures have immutable layout and may be
   accessed by any number of readers plus at most one writer concurrently,
   provided read and write regions do not intersect.

**How it is used in this project.** As a *behavioural reference for a
requirement*, per §3.3.5. The neutral requirement derived is: *the RHI's barrier
primitive must express synchronization scope, memory access visibility, and
texture layout as three independent axes, and must be able to represent
buffer-only and global barriers where layout is absent.* That requirement is
satisfied by an original design (see
[ADR-0006](docs/adr/ADR-0006-rhi-barrier-model.md)), not by mirroring
Microsoft's enum values.

**Implementation code copied:** **No.** The document is a specification written
in prose plus API declarations. No implementation source was consulted; the
`DirectX-Graphics-Samples` repository was deliberately **not** opened, since
§3.3.4 forbids browsing a competing implementation's source tree during
implementation.

**Claim-level confidence:** High. Retrieved in full from the vendor's own
specification repository.

---

## S-002 — Vulkan core revisions appendix

- **Title:** Core Revisions (Informative), Vulkan specification appendices
- **Publisher:** Khronos Group, `KhronosGroup/Vulkan-Docs`
- **URL:** https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/main/appendices/versions.adoc
- **Retrieved:** 2026-08-21 — 41,810 bytes, HTTP 200
- **Tier:** 2 (official standards body, retrieved in full)

**Information used.** Which extensions have been promoted into each core Vulkan
version. This determines what the second RHI backend may rely on without
querying extensions individually.

**Implementation code copied:** No. Specification prose only.

**Claim-level confidence:** High for promotion lists.

---

## S-003 — glTF 2.0 specification index

- **Title:** glTF 2.0 Specification
- **Publisher:** Khronos Group, `KhronosGroup/glTF`
- **URL:** https://raw.githubusercontent.com/KhronosGroup/glTF/main/specification/2.0/README.md
- **Retrieved:** 2026-08-21 — 3,548 bytes, HTTP 200
- **Tier:** 2 (official standards body)

**Information used.** Index only at this stage; retrieved to confirm the
specification is reachable from this environment before the asset pipeline
commits to glTF as its import path (§7.12 directs prioritising "a carefully
scoped glTF path"). Detailed extraction is deferred to the asset-pipeline work
and will be logged as a separate source entry when it happens.

**Implementation code copied:** No.

---

## S-004 — Vulkan 1.4 promoted extension set

- **Publisher:** Khronos Group (press) via search summary; corroborated by S-002
- **Tier:** 4 corroborated by tier 2
- **Retrieved:** 2026-08-21

**Information used.** Vulkan 1.4 promotes into core, among others:
`VK_KHR_dynamic_rendering_local_read`, `VK_KHR_maintenance5`,
`VK_KHR_maintenance6`, `VK_KHR_map_memory2`, `VK_KHR_push_descriptor`,
`VK_KHR_line_rasterization`, `VK_KHR_index_type_uint8`,
`VK_KHR_shader_float_controls2`, `VK_KHR_shader_subgroup_rotate`,
`VK_KHR_vertex_attribute_divisor`, `VK_EXT_host_image_copy`,
`VK_EXT_pipeline_protected_access`, `VK_EXT_pipeline_robustness`. Guarantees
8K render targets and up to eight simultaneous render targets.

**Claim-level confidence:** Medium-high. The promotion list is corroborated by
S-002, which was retrieved in full. Treat the "8K / eight render targets" limit
claim as **`NOT_VERIFIED_CURRENT`** until read directly from the limits table.

---

## S-005 — Direct3D 12 Agility SDK / Shader Model 6.9

- **Tier:** 4 (secondary technical press; the primary vendor blog is
  egress-blocked from this environment)
- **Retrieved:** 2026-08-21

**Information used.** Shader Model 6.9 has reached general release, shipped via
Agility SDK 1.619 with DXC 1.9.2602.16, alongside DXR 1.2. SM 6.9 adds "long
vector" support — loads, stores, and element-wise operations on HLSL vectors of
more than 4 and up to 1024 elements — and extends `IsNan`/`IsInf` and related
intrinsics to 16-bit floats.

**Claim-level confidence:** **`NOT_VERIFIED_CURRENT`.** Multiple independent
secondary outlets agree, which raises confidence, but §5.1 requires a primary
source for a version-specific capability claim and the primary source is
unreachable here. The engine must therefore **query** shader model support at
runtime rather than assume it, which is required by §7.3's capability-detection
clause regardless.

---

## S-006 — Engine capability landscape (all engines)

- **Tier:** 4 throughout. **Every vendor documentation domain required by §5.1
  is blocked by this environment's egress policy.**
- **Retrieved:** 2026-08-21

Findings are recorded in [ENGINE_COMPARISON_MATRIX.md](ENGINE_COMPARISON_MATRIX.md)
with per-cell evidence tiers. Every cell sourced this way is labelled
`NOT_VERIFIED_CURRENT`. See [RESEARCH_GAPS.md](RESEARCH_GAPS.md) for the exact
blocked-domain list and what it costs.

---

## S-007 — Vulkan API Registry (`vk.xml`)

- **Tier:** 1. The registry is the machine-readable form of the Vulkan
  specification, published by Khronos.
- **Source:** `KhronosGroup/Vulkan-Headers`, `registry/vk.xml`
- **Retrieved:** 2026-08-21 — 3,303,216 bytes, HTTP 200
- **Pinned:** `VK_HEADER_VERSION 360`, sha256
  `65d829561fa4b9e01a15e1327d9e6744f66b025b08c5c7ad13636bf0a8b15c62`
  (`build_scripts/vulkan_registry_pin.txt`)

### What was taken

The declarations themselves: 63 handles, 357 enumerant groups, 1,759 structures
and unions, 865 commands, 11 callback types, 20 API versions, and 706
extensions. Scoped to core 1.0–1.3 plus `VK_KHR_surface` and
`VK_KHR_swapchain`, that is 563 types and 229 commands.

Two rules were read out of the registry schema rather than hard-coded from
observed results:

- **Extension enumerant numbering:** `1000000000 + (extnumber - 1) * 1000 +
  offset`, negated when the entry carries `dir="-"`. Checked against
  `VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR` (extension 2, offset 0) =
  1000001000 and `VK_ERROR_OUT_OF_DATE_KHR` = -1000001004.
- **Core-version layering:** `VK_VERSION_1_0` depends on
  `VK_GRAPHICS_VERSION_1_0`, which depends on `VK_COMPUTE_VERSION_1_0`, which
  depends on `VK_BASE_VERSION_1_0`. These `apitype="internal"` features carry
  most of the actual requirements: selecting `VK_VERSION_1_0` without following
  the chain yields 22 types and 14 commands instead of 563 and 229.

### What was not taken

No code. The registry is read as a specification, exactly as the prose
specification is read. The file is not committed, and nothing Khronos authored
appears in this repository.

---

## S-008 — FIPS PUB 180-4, Secure Hash Standard

- **Tier:** 1. A published NIST standard.
- **Retrieved:** 2026-08-21
- **Sections used:** 4.1.2 and 4.2.2 (functions and round constants), 5.1.1
  (padding), 5.3.3 (initial hash value), 6.2 (SHA-256 computation), Appendix B
  (test vectors).

### Why this is here at all

The generated bindings carry the SHA-256 of the registry they came from.
Without a hash, "regenerated from `vk.xml`" is an unverifiable claim - the file
could have come from any registry, or an edited one. `vkgen` therefore verifies
the pin itself rather than trusting the Python fetcher to have done so.

Implemented from the algorithm description in `rust/tools/vkgen/src/sha256.rs`
and checked against the standard's own published digests for the empty message,
`abc`, the 448-bit two-block vector, and one million `a`. Those vectors are what
make the implementation checkable; a hash that is merely self-consistent
verifies nothing.

---

## Evidence collection note

Retrieved specification documents are **not** committed to this repository.
Committing third-party specification text would put material of another party's
authorship into the tree, which cuts against the clean separation
[DEPENDENCY_BOUNDARY.md](DEPENDENCY_BOUNDARY.md) maintains. What is committed is
this catalog: the URL, the retrieval date, the byte count, the HTTP status, and
the specific claims taken. Any reviewer can re-fetch and check.
