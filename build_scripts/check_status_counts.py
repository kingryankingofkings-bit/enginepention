#!/usr/bin/env python3
"""Checks that the catalog's status summary agrees with its own table.

The summary line in FEATURE_REQUIREMENTS_CATALOG.md states how many
requirements sit in each state. It is written by hand and the table is edited
one row at a time, so the two drift apart silently - which is exactly the
failure the project's reporting rules exist to prevent. This recounts the table
and compares.

Exits non-zero on disagreement. No third-party dependencies; see
DEPENDENCY_BOUNDARY.md.
"""

import collections
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
CATALOG = ROOT / "FEATURE_REQUIREMENTS_CATALOG.md"
README = ROOT / "README.md"

ROW = re.compile(r"^\| (PN-[A-Z]{3}-\d{3}) \|.*\|\s*(.+?)\s*\|\s*$")
SUMMARY = re.compile(
    r"\*\*Current status: (\d+) `VERIFIED`, (\d+) `IMPLEMENTED_UNVERIFIED`, "
    r"(\d+) `BLOCKED`, (\d+)\s*\n?`NOT_STARTED`, of (\d+)\.\*\*"
)

# The README restates the same four numbers for a reader who never opens the
# catalog, which means it drifts for the same reason and has to be checked too.
README_SUMMARY = re.compile(
    r"Of (\d+) catalogued requirements: \*\*(\d+) are `VERIFIED`\*\*, (\d+) are\s*\n?"
    r"`IMPLEMENTED_UNVERIFIED`, (\d+) are `BLOCKED`[^,]*, and the\s*\n?"
    r"remaining (\d+) are `NOT_STARTED`\."
)

# A row may qualify its state with a note - "BLOCKED (`BLOCK-002`)" - so the
# state is whichever keyword the cell starts with.
STATES = (
    "NOT_STARTED",
    "RESEARCHED",
    "DESIGNED",
    "IMPLEMENTED_UNVERIFIED",
    "VERIFIED",
    "BLOCKED",
    "DEFERRED_BY_SCOPE",
)


def main() -> int:
    text = CATALOG.read_text(encoding="utf-8")

    counts: collections.Counter = collections.Counter()
    total = 0
    problems = []
    for line in text.splitlines():
        match = ROW.match(line)
        if match is None:
            continue
        total += 1
        cell = match.group(2)
        for state in sorted(STATES, key=len, reverse=True):
            if cell.startswith(state):
                counts[state] += 1
                break
        else:
            problems.append(f"{match.group(1)}: unrecognised state {cell!r}")

    summary = SUMMARY.search(text)
    if summary is None:
        problems.append(
            "no 'Current status: N `VERIFIED`, ... of N.' summary line found; the "
            "summary must stay in the form this check can read"
        )
    else:
        claimed = {
            "VERIFIED": int(summary.group(1)),
            "IMPLEMENTED_UNVERIFIED": int(summary.group(2)),
            "BLOCKED": int(summary.group(3)),
            "NOT_STARTED": int(summary.group(4)),
        }
        for state, value in claimed.items():
            if counts[state] != value:
                problems.append(
                    f"summary says {value} {state}, table has {counts[state]}"
                )
        if int(summary.group(5)) != total:
            problems.append(
                f"summary says {summary.group(5)} requirements, table has {total}"
            )

    readme = README_SUMMARY.search(README.read_text(encoding="utf-8"))
    if readme is None:
        problems.append(
            "README.md has no 'Of N catalogued requirements: ...' summary in the "
            "form this check can read"
        )
    else:
        for label, group, expected in (
            ("requirements", 1, total),
            ("VERIFIED", 2, counts["VERIFIED"]),
            ("IMPLEMENTED_UNVERIFIED", 3, counts["IMPLEMENTED_UNVERIFIED"]),
            ("BLOCKED", 4, counts["BLOCKED"]),
            ("NOT_STARTED", 5, counts["NOT_STARTED"]),
        ):
            if int(readme.group(group)) != expected:
                problems.append(
                    f"README says {readme.group(group)} {label}, table has {expected}"
                )

    unclaimed = sum(counts.values()) - sum(
        counts[state] for state in ("VERIFIED", "IMPLEMENTED_UNVERIFIED", "BLOCKED", "NOT_STARTED")
    )
    if unclaimed:
        problems.append(
            f"{unclaimed} requirement(s) are in a state the summary does not "
            "report; extend the summary rather than dropping them"
        )

    if problems:
        print("Requirement status summary does not match the table:", file=sys.stderr)
        for problem in problems:
            print(f"  {problem}", file=sys.stderr)
        return 1

    print(f"Requirement status summary matches the table ({total} requirements)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
