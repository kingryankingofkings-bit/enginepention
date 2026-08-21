#!/usr/bin/env python3
"""Fetch and verify the official Khronos Vulkan registry.

Requirement: PN-RND-001
Decision:    ADR-0009, ADR-0010

The registry is NOT committed to this repository. It is another party's
authored material, and vendoring it would blur the boundary this project keeps
between its own code and the specifications it implements against. What is
committed is the pin in build_scripts/vulkan_registry_pin.txt: the exact URL,
the header version, and the SHA-256. Any reviewer can re-fetch and confirm they
are looking at the same bytes the bindings were generated from.

Downloads to third_party/vk.xml, which is git-ignored, and verifies the hash.
A mismatch is a hard failure: silently generating bindings from a different
registry than the one recorded would make the provenance record a fiction.
"""

from __future__ import annotations

import hashlib
import sys
import urllib.request
from pathlib import Path

PIN_FILE = Path("build_scripts/vulkan_registry_pin.txt")


def read_pin(repo_root: Path) -> dict[str, str]:
    """Reads the pin that rust/tools/vkgen also compiles in.

    Kept in a shared file rather than duplicated here so the fetcher and the
    generator cannot disagree about which registry is pinned.
    """
    values: dict[str, str] = {}
    for number, raw in enumerate((repo_root / PIN_FILE).read_text().splitlines(), start=1):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if "=" not in line:
            raise SystemExit(f"{PIN_FILE}:{number}: expected `key = value`")
        key, _, value = line.partition("=")
        values[key.strip()] = value.strip()

    for required in ("url", "sha256", "header_version"):
        if required not in values:
            raise SystemExit(f"{PIN_FILE}: missing `{required}`")
    return values


DESTINATION = Path("third_party/vk.xml")


def sha256_of(data: bytes) -> str:
    digest = hashlib.sha256()
    digest.update(data)
    return digest.hexdigest()


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    pin = read_pin(repo_root)
    registry_url = pin["url"]
    expected_sha256 = pin["sha256"]
    expected_header_version = pin["header_version"]

    destination = repo_root / DESTINATION
    destination.parent.mkdir(parents=True, exist_ok=True)

    allow_update = "--update-pin" in sys.argv

    if destination.is_file():
        data = destination.read_bytes()
        if sha256_of(data) == expected_sha256:
            print(f"{DESTINATION} already present and matches the pin")
            return 0
        print(f"{DESTINATION} does not match the pin; re-fetching", file=sys.stderr)

    print(f"Fetching {registry_url}")
    try:
        with urllib.request.urlopen(registry_url, timeout=120) as response:
            data = response.read()
    except Exception as exc:  # noqa: BLE001 - report any transport failure plainly
        print(f"Could not fetch the registry: {exc}", file=sys.stderr)
        print(
            "\nThe Vulkan registry is reachable from github.com. If this host's "
            "egress policy blocks it, fetch vk.xml elsewhere and place it at "
            f"{DESTINATION}; the hash check below still applies.",
            file=sys.stderr,
        )
        return 1

    actual = sha256_of(data)
    if actual != expected_sha256:
        if not allow_update:
            print("Registry hash MISMATCH", file=sys.stderr)
            print(f"  expected {expected_sha256}", file=sys.stderr)
            print(f"  actual   {actual}", file=sys.stderr)
            print(
                "\nThe upstream registry has changed since the pin was recorded.\n"
                "This is not automatically an error - Khronos updates vk.xml "
                "regularly - but it must not pass unnoticed, because every "
                "generated binding depends on it.\n\n"
                "To adopt the new registry deliberately:\n"
                "  1. re-run with --update-pin to download it\n"
                "  2. update sha256 and header_version in build_scripts/vulkan_registry_pin.txt\n"
                "  3. regenerate the bindings and review the resulting diff\n"
                "  4. record the change in PROVENANCE_LEDGER.md",
                file=sys.stderr,
            )
            return 1
        print(f"--update-pin given; accepting {actual}", file=sys.stderr)

    destination.write_bytes(data)
    print(f"Wrote {DESTINATION} ({len(data)} bytes)")
    print(f"  sha256 {actual}")
    print(f"  pinned VK_HEADER_VERSION {expected_header_version}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
