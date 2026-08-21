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
| _(none yet — populated during Phase 1)_ | | | | | | |

### Engine capability documentation

| ID | Title | Publisher | URL | Retrieved | Information used | Implementation code copied? |
|---|---|---|---|---|---|---|
| _(none yet — populated during Phase 1)_ | | | | | | |

### Technical papers and presentations

| ID | Title | Publisher | URL | Retrieved | Information used | Implementation code copied? |
|---|---|---|---|---|---|---|
| _(none yet — populated during Phase 1)_ | | | | | | |

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
