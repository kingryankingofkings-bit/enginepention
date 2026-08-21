#!/usr/bin/env python3
"""Verify every engine source file carries an authorship header.

Requirement: PN-OPS-009
Clean-room rule: the execution prompt requires an authorship record mapping every
implementation file to its originating task and design decision. A convention
that is not checked is not a record, so this runs in CI.

Exit status 0 if every file conforms, 1 otherwise.
"""

from __future__ import annotations

import sys
from pathlib import Path

SOURCE_ROOTS = ("engine", "editor", "tools", "server", "samples")
SOURCE_SUFFIXES = (".hpp", ".cpp", ".h", ".cc", ".inl")
HEADER_SCAN_LINES = 12



def _is_generated(path: Path) -> bool:
    """True for paths inside a build or generated tree.

    The checks walk the source tree directly rather than asking git, so they
    must skip generated output themselves. A stray nested build/ directory
    otherwise fails the authorship check on CMake's own compiler-probe file.
    """
    return any(part in {"build", "out", "CMakeFiles", ".git"} or part.startswith("build-")
               for part in path.parts)

def offending_files(repo_root: Path) -> list[tuple[Path, str]]:
    problems: list[tuple[Path, str]] = []
    for root_name in SOURCE_ROOTS:
        root = repo_root / root_name
        if not root.is_dir():
            continue
        for path in sorted(root.rglob("*")):
            if _is_generated(path):
                continue
            if path.suffix not in SOURCE_SUFFIXES or not path.is_file():
                continue
            try:
                head = path.read_text(encoding="utf-8").splitlines()[:HEADER_SCAN_LINES]
            except (OSError, UnicodeDecodeError) as exc:
                problems.append((path, f"unreadable: {exc}"))
                continue
            joined = "\n".join(head)
            if "Requirement:" not in joined:
                problems.append((path, "missing 'Requirement:' line in the header"))
            elif "Decision:" not in joined:
                problems.append((path, "missing 'Decision:' line in the header"))
    return problems


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    problems = offending_files(repo_root)

    if problems:
        print("Authorship header check FAILED", file=sys.stderr)
        for path, reason in problems:
            print(f"  {path.relative_to(repo_root)}: {reason}", file=sys.stderr)
        print(
            "\nEvery source file must name the requirement it implements and the "
            "ADR it follows, in the first "
            f"{HEADER_SCAN_LINES} lines. For example:\n"
            "  // Pention Engine - core/thing.hpp\n"
            "  // Requirement: PN-PLT-001 - what this file is for\n"
            "  // Decision:    ADR-0004",
            file=sys.stderr,
        )
        return 1

    print("Authorship header check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
