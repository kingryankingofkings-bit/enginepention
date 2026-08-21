# Pention Engine

A clean-room, custom-built, next-generation 3D sandbox / free-roam game engine
and integrated editor, written in C++23.

**Status: Phase 0 complete — environment and repository truth established.**
No engine capability exists yet. This README will not describe a feature as
existing until that feature is `VERIFIED` with executed evidence.

## What this project is

An independently engineered engine. It studies the documented capabilities and
published techniques of leading engines, and implements comparable capability
through original designs. It contains no engine forks, no middleware, and no
copied implementation code — see [DEPENDENCY_BOUNDARY.md](DEPENDENCY_BOUNDARY.md)
and [PROVENANCE_LEDGER.md](PROVENANCE_LEDGER.md) for the enforced boundary.

## Current state

| Phase | State |
|---|---|
| 0 — Environment and repository truth | Complete |
| 1 — Research and normalized requirements | Not started |
| 2 — Product specification and architecture | Not started |
| 3 — Toolchain bootstrap and Tier A core | Not started |
| 4–9 | Not started |

There is no buildable engine, no renderer, no editor, and no runtime yet.
Anything stating otherwise would violate §4 of the governing execution prompt.

## Governing documents

| Document | Purpose |
|---|---|
| [docs/ENVIRONMENT_BASELINE.md](docs/ENVIRONMENT_BASELINE.md) | Measured facts about the build environment, and the external blockers |
| [docs/INSTRUCTION_CONFLICTS.md](docs/INSTRUCTION_CONFLICTS.md) | Recorded conflicts between instruction sources and how each was resolved |
| [PROVENANCE_LEDGER.md](PROVENANCE_LEDGER.md) | Clean-room controls; every external source consulted |
| [DEPENDENCY_BOUNDARY.md](DEPENDENCY_BOUNDARY.md) | Every non-engine artefact, and why it is infrastructure rather than a subsystem |
| [docs/adr/](docs/adr/) | Architecture decision records |

## Known blockers

The current development environment is a headless Linux container with **no
GPU**, no Windows SDK, no Direct3D 12 headers, and no shader compilers. The
graphics backend therefore cannot be compiled or executed here. These are
tracked as `BLOCK-001` through `BLOCK-005` in
[docs/ENVIRONMENT_BASELINE.md](docs/ENVIRONMENT_BASELINE.md) §3, each with the
smallest exact external action required to clear it.

Work that does not depend on them proceeds.
