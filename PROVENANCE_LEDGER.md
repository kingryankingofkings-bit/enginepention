# Provenance Ledger

Established **before substantive implementation**, per Master Execution Prompt
§3.3.1. Every external document consulted during this project is recorded here
with its title, publisher, URL, retrieval date, the information taken from it,
and an explicit confirmation that no implementation code was copied.

## Standing rules in force

1. Study **product behaviour and public specifications**, never source
   repositories of competing engines or libraries (§3.3.3, §3.3.4).
2. Convert observed capability into a **neutral requirement** before designing a
   solution (§3.3.5).
3. Design each subsystem independently from first principles and published
   standards (§3.3.6).
4. Original names for original subsystems. Never reuse protected branding —
   including but not limited to Nanite, Lumen, Blueprint, Niagara, MetaSounds,
   Chaos — as a feature name (§3.3.7).
5. Where provenance is uncertain, quarantine the work, mark it
   `PROVENANCE_BLOCKED`, and reimplement independently (§3.3.10).

## Engine identity

The engine is named **Pention**. The C++ namespace is `pn`. The name is original
to this project and carries no relationship to any existing engine, product, or
trademark known to the authors. Subsystem names coined for this project are
recorded in [ORIGINALITY_DESIGN_MAP.md](ORIGINALITY_DESIGN_MAP.md) as they are
introduced.

## Authorship record

Every implementation file carries a header comment naming the originating task
and the design decision (ADR) it implements, satisfying §3.3.8. The mapping is
mechanically checkable: `build_scripts/check_authorship.py` fails the build for
any source file in `engine/`, `editor/`, `tools/`, or `server/` lacking one.

## Source consultation log

Format: one row per document actually consulted. A source is added **when it is
read**, not when it is planned. Empty sections are honest, not oversights.

### Standards and specifications

| ID | Title | Publisher | URL | Retrieved | Information used | Implementation code copied? |
|---|---|---|---|---|---|---|
| S-001 | Enhanced Barriers (D3D12) | Microsoft, `microsoft/DirectX-Specs` | https://raw.githubusercontent.com/microsoft/DirectX-Specs/master/d3d/D3D12EnhancedBarriers.md | 2026-08-21 | Barrier model: independent sync/access/layout axes; texture, buffer and global barrier kinds; queue-type layout compatibility; absence of state promotion/decay | **No** — prose and API declarations only; `DirectX-Graphics-Samples` deliberately not opened |
| S-002 | Vulkan Core Revisions appendix | Khronos Group, `KhronosGroup/Vulkan-Docs` | https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/main/appendices/versions.adoc | 2026-08-21 | Extension-to-core promotion lists per Vulkan version | **No** — specification prose only |
| S-003 | glTF 2.0 Specification (index) | Khronos Group, `KhronosGroup/glTF` | https://raw.githubusercontent.com/KhronosGroup/glTF/main/specification/2.0/README.md | 2026-08-21 | Index only; confirmed reachability before committing the asset pipeline to glTF | **No** |
| S-007 | Vulkan API Registry (`vk.xml`) | Khronos Group, `KhronosGroup/Vulkan-Headers` | https://raw.githubusercontent.com/KhronosGroup/Vulkan-Headers/main/registry/vk.xml | 2026-08-21 | The machine-readable Vulkan specification: type, command, enumerant and extension declarations, and the extension-enum numbering formula. Pinned at `VK_HEADER_VERSION 360`, sha256 `65d8295...b15c62` (`build_scripts/vulkan_registry_pin.txt`) | **No** — read as a specification, exactly as the prose specification is read. The file is **not committed**; `rust/tools/vkgen` reads its declarations and writes this project's own Rust from them. See [ADR-0010](docs/ADR-0010-vulkan-bindings-from-the-registry.md) |
| S-008 | FIPS PUB 180-4, Secure Hash Standard | NIST | Published standard, sections 4.1.2, 4.2.2, 5.3.3, 6.2 and Appendix B | 2026-08-21 | The SHA-256 algorithm: round and initial constants, padding rule, message schedule, compression function; and the published test vectors | **No** — implemented from the algorithm description. Vectors in `rust/tools/vkgen/tests/sha256.rs` are the standard's own published digests, which is what makes the implementation checkable rather than merely self-consistent |

### Engine capability documentation

| ID | Title | Publisher | URL | Retrieved | Information used | Implementation code copied? |
|---|---|---|---|---|---|---|
| S-006 | Engine capability landscape | Various (search snippets only) | see [ENGINE_COMPARISON_MATRIX.md](ENGINE_COMPARISON_MATRIX.md) | 2026-08-21 | Provisional capability rows for UE, Unity, Godot, O3DE, CryEngine. **All `NOT_VERIFIED_CURRENT`** — every vendor documentation domain is egress-blocked (`BLOCK-006`) | **No** — no source was reachable to copy from |

### Technical papers and presentations

| ID | Title | Publisher | URL | Retrieved | Information used | Implementation code copied? |
|---|---|---|---|---|---|---|
| S-005 | Shader Model 6.9 / Agility SDK 1.619 / DXR 1.2 | Secondary technical press | see [RESEARCH_CATALOG.md](RESEARCH_CATALOG.md#s-005--direct3d-12-agility-sdk--shader-model-69) | 2026-08-21 | SM 6.9 long-vector support; 16-bit float intrinsic coverage. Used only to justify **runtime capability detection**, never as a load-bearing assumption | **No** |

## Model-generated code declaration

This project's source is authored with AI assistance. That creates a specific
provenance hazard §3.1 names directly: AI-produced code that is *recognisably
derived* from an existing engine, library, tutorial, or sample is prohibited
exactly as a copy-paste would be.

Controls applied:

- Every subsystem is designed against a written neutral requirement in
  [FEATURE_REQUIREMENTS_CATALOG.md](FEATURE_REQUIREMENTS_CATALOG.md) before code
  is written, so the design has a traceable non-copied origin.
- Algorithms taken from published descriptions are implemented from the prose or
  mathematical statement, never from a reference implementation or a paper's
  code appendix (§16). The conceptual source is cited in the file header.
- Distinctive names, comment text, structural quirks, and magic constants that
  would fingerprint a known implementation are treated as provenance smells and
  investigated.
- Similarity auditing runs before every release candidate (§3.3.9); results are
  recorded in this ledger.

## Quarantine log

Work marked `PROVENANCE_BLOCKED` and its disposition.

| Date | Item | Reason | Disposition |
|---|---|---|---|
| _(none)_ | | | |

## Audit history

| Date | Audit type | Scope | Result | Evidence |
|---|---|---|---|---|
| 2026-08-21 | Initial provenance baseline | Empty repository, zero commits | Clean origin — no pre-existing code of any provenance | `docs/ENVIRONMENT_BASELINE.md` §5 |
| 2026-08-21 | Phase 1 source review | 6 sources consulted (S-001..S-006) | No implementation code copied from any source. Three retrieved in full at tier 1–2; three are tier-4 and labelled `NOT_VERIFIED_CURRENT` | [RESEARCH_CATALOG.md](RESEARCH_CATALOG.md) |
| 2026-08-21 | Vulkan binding generation review | `rust/tools/vkgen`, `rust/crates/pn-vulkan-sys` | Clean. The one input is a pinned specification file that is not committed; the emitted Rust is written by this project's generator, and CI re-derives it to prove the committed output is what the generator produces (`build_scripts/check_generated_bindings.py`) | [ADR-0010](docs/ADR-0010-vulkan-bindings-from-the-registry.md) |
