# Research Gaps

Unresolved or unverifiable claims, per Master Execution Prompt §5.3. §16 forbids
inventing research; §4 requires that where live research is unavailable,
time-sensitive comparisons are marked `NOT_VERIFIED_CURRENT` and work continues
only from stable specifications and clearly labelled assumptions. This file is
the honest accounting of what could not be established.

## GAP-001 — Vendor documentation domains are blocked by egress policy

**Severity: high. This is the dominant limitation on Phase 1.**

The execution environment routes outbound HTTPS through a policy-enforcing
proxy. Measured on 2026-08-21, the following required §5.1 domains are **denied**:

| Domain | Needed for | Result |
|---|---|---|
| `dev.epicgames.com` | Unreal Engine 5 documentation and release notes | Blocked |
| `docs.unity3d.com` | Unity current release manual | Blocked |
| `godotengine.org` | Godot stable release documentation | Blocked |
| `o3de.org` / `docs.o3de.org` | O3DE and Atom renderer documentation | Blocked |
| `cryengine.com` | CryEngine release documentation | Blocked |
| `learn.microsoft.com` | Direct3D 12 / DXR reference documentation | Blocked |
| `khronos.org`, `registry.khronos.org`, `docs.vulkan.org` | Vulkan, SPIR-V, OpenXR registries | Blocked |
| `developer.nvidia.com`, `gpuopen.com` | Vendor technical material | Blocked |
| `arxiv.org`, `en.wikipedia.org` | Papers, background | Blocked |

Only `github.com` and `raw.githubusercontent.com` are reachable.

**What this recovers.** Microsoft publishes the authoritative Direct3D
specifications in `microsoft/DirectX-Specs`, and Khronos publishes the Vulkan
and glTF specification sources in `KhronosGroup/*`. Both are on GitHub and were
retrieved in full — see S-001, S-002, S-003. **The technical standards that
actually drive implementation are therefore available at tier 1–2.**

**What remains lost.** Vendor *product* documentation — the feature lists,
maturity labels, platform requirements, and workflow descriptions that §5.2
requires for the competitive comparison. Those live only on blocked domains.
`WebSearch` returns titles and snippets but cannot substitute for reading the
primary document, and §5.1 explicitly ranks secondary analysis as usable *only
to find or clarify a primary source*.

**Consequence.** Every cell in [ENGINE_COMPARISON_MATRIX.md](ENGINE_COMPARISON_MATRIX.md)
derived from search snippets is labelled `NOT_VERIFIED_CURRENT`. The comparison
matrix is therefore **provisional** and must be re-derived before it is used to
justify any parity claim in Phase 9.

**Smallest external action required.** Either (a) an egress policy permitting the
vendor documentation domains above, or (b) the user supplying the relevant
documentation directly. Tracked as `BLOCK-006`.

**What is NOT affected.** Phases 2 through 8 depend far more on the standards
(reachable) than on the competitive matrix (not reachable). Requirements derived
from §7's mandatory capability inventory are given by the prompt itself and do
not require competitive research to be valid. Implementation continues.

## GAP-002 — Version-specific graphics capability claims

`NOT_VERIFIED_CURRENT`: the Shader Model 6.9 / Agility SDK 1.619 / DXR 1.2
claims in S-005 come from corroborating secondary outlets, not from Microsoft's
own release note, which is on a blocked domain.

**Mitigation, and why it is adequate:** the engine is required by §7.3 to
perform capability detection and provide graceful fallback regardless. A
runtime-queried capability is strictly more reliable than a documented one. No
architectural decision is permitted to *depend* on an unverified version claim;
where one would, the fallback path is mandatory rather than optional.

## GAP-003 — CryEngine's public release is materially behind its internal one

Search results indicate the latest public CryEngine release is 5.7 LTS (2022),
while Crytek internally runs a newer line that has not been released to third
parties. If accurate, a comparison against *public* CryEngine measures a
four-year-old artefact and would overstate this project's relative position.

Status: `NOT_VERIFIED_CURRENT`. **Recorded rather than averaged away**, per
§5.3's instruction to record disagreements instead of resolving them into a
false conclusion. Any parity claim against CryEngine must state which version.

## GAP-004 — No performance data of any kind

No GPU exists in this environment (`BLOCK-002`). Consequently:

- No frame-time, bandwidth, occupancy, or memory-residency measurement is
  possible.
- No competitor performance claim can be independently reproduced.
- Every figure in [PERFORMANCE_BUDGETS.md](PERFORMANCE_BUDGETS.md) when written
  will be a **target**, explicitly labelled as such, never an achievement.

§11's rule applies without exception: a performance target may remain a target;
it is never rewritten as an achievement.

## GAP-005 — Patent landscape not assessed

§5.2 and §5.3 require a patent risk assessment. Real-time rendering carries
known patent activity across several of the techniques this engine will need.
Assessing that requires searchable patent databases, all unreachable here, and
competent legal review, which is out of scope for an engineering agent.

Recorded in [TECHNIQUE_RISK_REGISTER.md](TECHNIQUE_RISK_REGISTER.md) as
requiring qualified legal review, per §16's instruction to flag rather than
invent legal conclusions. **No patent clearance is claimed or implied anywhere
in this repository.**
