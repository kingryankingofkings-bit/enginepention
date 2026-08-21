#!/usr/bin/env python3
"""Verify the committed Vulkan bindings match what vkgen produces today.

Requirement: PN-RND-001, PN-OPS-001
Decision:    ADR-0009, ADR-0010

rust/crates/pn-vulkan-sys/src/generated.rs is committed so the build does not
require the registry. That only works if the committed file is genuinely what
the generator produces from the pinned vk.xml. Without this check, a hand edit
to the generated file - or a generator change nobody regenerated for - would
sit in the tree indefinitely, and the provenance header would be a claim rather
than a fact.

Fetches the pinned registry, regenerates into a temporary file, and diffs.
"""

from __future__ import annotations

import difflib
import subprocess
import sys
import tempfile
from pathlib import Path

REGISTRY = Path("third_party/vk.xml")
COMMITTED = Path("rust/crates/pn-vulkan-sys/src/generated.rs")
MANIFEST = Path("rust/Cargo.toml")
FETCHER = Path("build_scripts/fetch_vulkan_registry.py")


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    committed = repo_root / COMMITTED

    if not committed.is_file():
        print(f"{COMMITTED} is missing", file=sys.stderr)
        return 1

    fetch = subprocess.run(
        [sys.executable, str(repo_root / FETCHER)],
        cwd=repo_root,
        capture_output=True,
        text=True,
    )
    sys.stdout.write(fetch.stdout)
    if fetch.returncode != 0:
        sys.stderr.write(fetch.stderr)
        print(
            "\nCould not obtain the pinned registry, so the generated bindings "
            "cannot be verified. This is a blocked check, not a passing one.",
            file=sys.stderr,
        )
        return 1

    with tempfile.TemporaryDirectory() as scratch:
        regenerated = Path(scratch) / "generated.rs"
        generate = subprocess.run(
            [
                "cargo",
                "run",
                "--offline",
                "--quiet",
                "--manifest-path",
                str(repo_root / MANIFEST),
                "-p",
                "vkgen",
                "--",
                str(repo_root / REGISTRY),
                "--emit",
                str(regenerated),
            ],
            cwd=repo_root,
            capture_output=True,
            text=True,
        )
        if generate.returncode != 0:
            sys.stdout.write(generate.stdout)
            sys.stderr.write(generate.stderr)
            print("\nvkgen failed", file=sys.stderr)
            return 1

        expected = regenerated.read_text()

    actual = committed.read_text()
    if actual == expected:
        print(f"{COMMITTED} matches what vkgen produces from the pinned registry")
        return 0

    diff = difflib.unified_diff(
        actual.splitlines(keepends=True),
        expected.splitlines(keepends=True),
        fromfile=f"{COMMITTED} (committed)",
        tofile=f"{COMMITTED} (regenerated)",
        n=3,
    )
    # Bounded: a registry bump rewrites thousands of lines, and a wall of diff
    # in a CI log helps nobody decide what to do about it.
    shown = 0
    for line in diff:
        sys.stdout.write(line)
        shown += 1
        if shown >= 200:
            print("... diff truncated ...")
            break

    print(
        "\nThe committed bindings are out of date. Regenerate them:\n"
        "  python3 build_scripts/fetch_vulkan_registry.py\n"
        "  cargo run -p vkgen --manifest-path rust/Cargo.toml -- \\\n"
        "      third_party/vk.xml --emit rust/crates/pn-vulkan-sys/src/generated.rs\n"
        "and review the diff before committing it.",
        file=sys.stderr,
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
