#!/usr/bin/env python3
"""Fail the build if any Rust crate declares an external dependency.

Requirement: PN-OPS-009
Decision:    ADR-0009

The custom-build boundary prohibits package-manager dependencies without
qualifying the package manager, and cargo is one. The rule does not soften
because reaching for a crate is the Rust ecosystem's convention - which is
exactly why it needs a mechanical check rather than a reviewer's memory.

Path dependencies within this workspace are permitted: they are this project's
own code, not a third party's.
"""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path

DEPENDENCY_TABLES = ("dependencies", "dev-dependencies", "build-dependencies")


def offending_dependencies(manifest: Path) -> list[str]:
    """Returns a description of each non-path dependency in one manifest."""
    try:
        data = tomllib.loads(manifest.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as exc:
        return [f"could not be parsed: {exc}"]

    problems: list[str] = []
    scopes = [("", data)]
    scopes += [(f"target.{name}.", table) for name, table in data.get("target", {}).items()]

    for prefix, table in scopes:
        for kind in DEPENDENCY_TABLES:
            for name, spec in (table.get(kind) or {}).items():
                # A bare string is always a registry version requirement.
                if isinstance(spec, str):
                    problems.append(f"[{prefix}{kind}] {name} = \"{spec}\" (crates.io)")
                    continue
                if isinstance(spec, dict):
                    if "path" in spec:
                        continue  # this project's own code
                    origin = (
                        "git" if "git" in spec
                        else "registry" if "registry" in spec
                        else "crates.io"
                    )
                    problems.append(f"[{prefix}{kind}] {name} ({origin})")
    return problems


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    rust_root = repo_root / "rust"

    if not rust_root.is_dir():
        print("No rust/ directory; nothing to check")
        return 0

    failures: list[tuple[Path, str]] = []
    manifests = sorted(
        p for p in rust_root.rglob("Cargo.toml")
        if "target" not in p.parts and "rust-target" not in p.parts
    )

    if not manifests:
        print("No Cargo manifests found under rust/", file=sys.stderr)
        return 1

    for manifest in manifests:
        for problem in offending_dependencies(manifest):
            failures.append((manifest.relative_to(repo_root), problem))

    # Belt and braces: a committed lockfile naming a registry source would mean a
    # dependency slipped in through a path this parser did not model.
    lockfile = rust_root / "Cargo.lock"
    if lockfile.is_file():
        text = lockfile.read_text(encoding="utf-8")
        for match in re.finditer(r'^source = "(.+)"$', text, re.MULTILINE):
            failures.append(
                (lockfile.relative_to(repo_root), f"lockfile names an external source: {match.group(1)}")
            )

    if failures:
        print("Crate dependency check FAILED", file=sys.stderr)
        for path, problem in failures:
            print(f"  {path}: {problem}", file=sys.stderr)
        print(
            "\nNo crates.io, git, or registry dependencies are permitted - see\n"
            "DEPENDENCY_BOUNDARY.md. A permissive licence does not satisfy the\n"
            "custom-build requirement, and cargo is a package manager like any\n"
            "other. Path dependencies inside this workspace are fine.",
            file=sys.stderr,
        )
        return 1

    print(f"Crate dependency check passed ({len(manifests)} manifests, no external dependencies)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
