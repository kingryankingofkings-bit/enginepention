# Technique Risk Register

Patent, license, provenance, feasibility, hardware, performance, and maintenance
risks, per Master Execution Prompt §5.3.

Severity: **H** high · **M** medium · **L** low.

## Legal and patent risk

> **No patent clearance is claimed, performed, or implied anywhere in this
> repository.** §16 forbids inventing legal conclusions. The entries below flag
> matters requiring qualified legal review; they are not legal advice and they
> are not a clearance.

| ID | Technique | Risk | Sev | Handling |
|---|---|---|---|---|
| RSK-L01 | Real-time rendering techniques generally | Patent activity is known to exist across this field. This environment cannot reach patent databases ([GAP-005](RESEARCH_GAPS.md#gap-005--patent-landscape-not-assessed)) | H | **Requires qualified legal review before commercial release.** Every technique implemented is recorded with its conceptual source in [ORIGINALITY_DESIGN_MAP.md](ORIGINALITY_DESIGN_MAP.md) so a reviewer has a concrete list to assess |
| RSK-L02 | Perceptual/psychoacoustic audio coding | Historically patent-dense | M | Prefer uncompressed and simple lossless internal formats; defer any perceptual codec until reviewed |
| RSK-L03 | HRTF datasets for spatial audio | Many public datasets carry research-only or non-commercial terms | M | PN-AUD-007 requires the dataset's provenance and licence be recorded **before** use; an original measurement or synthesis strategy is the fallback |
| RSK-L04 | Texture block-compression formats | Format specifications are public; some hardware formats have had licensing history | M | Implement compressors from public format specifications; record in the provenance ledger; flag for review |
| RSK-L05 | Trademark exposure from feature naming | Reusing a competitor's feature brand would infringe and would breach §3.3.7 | L | Original names only, tracked in [ORIGINALITY_DESIGN_MAP.md](ORIGINALITY_DESIGN_MAP.md). Forbidden list explicitly includes Nanite, Lumen, Blueprint, Niagara, MetaSounds, Chaos |

## Provenance risk

| ID | Risk | Sev | Handling |
|---|---|---|---|
| RSK-P01 | **AI-generated code recognisably derived from a known engine or library.** §3.1 prohibits this exactly as it prohibits copy-paste. This is the single most likely provenance failure mode for this project | H | Neutral requirement written before design; algorithms implemented from prose or mathematics, never from a reference implementation or a paper's code appendix (§16); similarity audit before every release candidate; distinctive constants, comment phrasings, and structural quirks treated as smells |
| RSK-P02 | A permissively licensed library imported because its licence permits it | H | §3.1 and §16 are explicit that licence permission does not satisfy the custom-build requirement. [DEPENDENCY_BOUNDARY.md](DEPENDENCY_BOUNDARY.md) has no "small utility" exemption, deliberately |
| RSK-P03 | Accidental exposure to a competitor's source through a search result or a documentation page that embeds implementation | M | §3.3.4 prohibits browsing competing source trees during implementation. `microsoft/DirectX-Graphics-Samples` was deliberately not opened when the Enhanced Barriers spec was consulted |
| RSK-P04 | A specification document that embeds a reference implementation | M | Take the prose and the interface; do not transcribe sample code. Record which sections were used, as S-001 does |

## Feasibility and scope risk

| ID | Risk | Sev | Handling |
|---|---|---|---|
| RSK-F01 | **Total scope.** 214 catalogued requirements spanning renderer, physics, animation, audio, AI, networking, UI, editor, and asset pipeline is, at commercial staffing, many engineering-years | H | Tiers A–D are cumulative and gated. Progress is reported per requirement with an explicit state, so partial completion is always visible as partial. §16's prohibition on implying completeness is the controlling rule |
| RSK-F02 | Writing a graphics backend that cannot be compiled or run in the development environment | H | The RHI is designed against two specifications rather than one implementation. A non-hardware reference backend (ADR-0005) keeps render-graph and resource logic executable and testable without a GPU. Backend code stays `IMPLEMENTED_UNVERIFIED` until real hardware runs it |
| RSK-F03 | Original high-density geometry and GI systems are research-grade problems | H | Both are Tier D. Neither is started before its Tier C dependencies are `VERIFIED`. A documented fallback path is a requirement, not a contingency |
| RSK-F04 | In-project scripting language and VM is a large sub-project | M | Constrained deterministic language, sandboxed by construction, with a conformance suite written before the implementation |
| RSK-F05 | Determinism claims across compilers and platforms are easy to assert and hard to hold | M | Determinism is scoped explicitly: fixed-step, defined float behaviour, no fast-math in simulation. Limits documented per PN-PHY-017 rather than claimed globally |

## Hardware and performance risk

| ID | Risk | Sev | Handling |
|---|---|---|---|
| RSK-H01 | **No GPU in the development environment.** No rendering performance figure can be produced | H | `BLOCK-002`. Every rendering budget is a *target*, labelled as such. §11's rule that a target is never rewritten as an achievement is absolute here |
| RSK-H02 | 4 CPU cores is a weak signal for scheduler correctness | M | Randomized scheduling stress and TSan rather than reliance on natural contention |
| RSK-H03 | Capability variance across GPU vendors and driver versions | M | Runtime capability query with mandatory fallback (PN-RND-001, §7.3). No architectural decision may depend on an unverified version claim ([GAP-002](RESEARCH_GAPS.md#gap-002--version-specific-graphics-capability-claims)) |
| RSK-H04 | Memory budget is the binding open-world constraint, more than frame time | M | Streaming pool sized as fixed, never elastic (PN-WLD-006). Budget overrun is a test failure, not a warning |

## Maintenance and architecture risk

| ID | Risk | Sev | Handling |
|---|---|---|---|
| RSK-M01 | Graphics API details leaking out of the RHI module, turning the second backend into a rewrite | H | ADR-0004's single-module rule, enforced by a CI check on includes rather than by convention |
| RSK-M02 | Retrofitting threading into single-threaded subsystems | H | Job system is Tier A and precedes the subsystems that depend on it. This ordering is treated as non-negotiable |
| RSK-M03 | Retrofitting `f64` world coordinates after `f32` has spread | H | PN-WLD-001 is Tier A. World-space types are `f64` from the first commit that has a transform |
| RSK-M04 | Retrofitting motion vectors after multiple vertex paths exist | H | Motion-vector output is a *requirement of every vertex path* from the first one, not a post-process added at PN-RND-023 |
| RSK-M05 | C++ memory-safety defects | H | Sanitizer configurations in CI, bounds-checked debug containers, mandatory parser fuzzing (PN-OPS-008) |
| RSK-M06 | GCC/Clang divergence discovered late | M | Both compilers build every commit from the first commit. Already paid off once — [CONF-003](docs/INSTRUCTION_CONFLICTS.md#conf-003--stdexpected-availability-across-the-two-supported-compilers) |
| RSK-M07 | Documentation drifting into describing planned behaviour as existing | H | §4 and §13 require planned behaviour to be labelled planned. Feature state is tracked per requirement, and the README states plainly that nothing is built yet |
