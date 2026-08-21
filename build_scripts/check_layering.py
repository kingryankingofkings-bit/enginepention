#!/usr/bin/env python3
"""Enforce the module dependency direction and the single-graphics-API rule.

Requirement: PN-PLT-018
Decision:    ADR-0004

Two rules, both of which erode instantly if left to convention:

  1. Dependencies point one way. A module may include headers only from itself
     and from modules strictly below it in the graph.
  2. A graphics API header appears in exactly one module. If D3D12 or Vulkan
     types spread beyond the RHI, the second backend becomes a rewrite rather
     than a port - which is the thing that keeps the backend ordering in
     ADR-0001 reversible at all.

Exit status 0 if the tree conforms, 1 otherwise.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

# Each module maps to the set of modules it is permitted to depend on. The graph
# is declared once, here, so adding a module means editing one place rather than
# hunting for a hard-coded list.
ALLOWED_DEPENDENCIES: dict[str, set[str]] = {
    "math": set(),
    "core": {"math"},
    "platform": {"math", "core"},
    "jobs": {"math", "core", "platform"},
    "scene": {"math", "core", "platform", "jobs"},
    "assets": {"math", "core", "platform", "jobs", "scene"},
    "rhi": {"math", "core", "platform", "jobs"},
    "rendergraph": {"math", "core", "platform", "jobs", "rhi"},
    "render": {"math", "core", "platform", "jobs", "scene", "assets", "rhi", "rendergraph"},
    "world": {"math", "core", "platform", "jobs", "scene", "assets"},
    "physics": {"math", "core", "platform", "jobs", "scene"},
    "animation": {"math", "core", "platform", "jobs", "scene"},
    "audio": {"math", "core", "platform", "jobs", "scene"},
    "ai": {"math", "core", "platform", "jobs", "scene", "world"},
    "network": {"math", "core", "platform", "jobs", "scene"},
    "scripting": {"math", "core", "platform", "jobs", "scene"},
    "ui": {"math", "core", "platform", "jobs", "scene", "rhi", "rendergraph"},
    "runtime": {
        "math", "core", "platform", "jobs", "scene", "assets", "rhi",
        "rendergraph", "render", "world", "physics", "animation", "audio",
        "ai", "network", "scripting", "ui",
    },
    # Test-only. Outside the runtime graph: nothing that ships may depend on it,
    # and it depends on nothing itself.
    "testing": set(),
}

# Modules permitted to name a graphics API header. Exactly one.
GRAPHICS_API_OWNERS = {"rhi"}

GRAPHICS_API_HEADERS = re.compile(
    r"""\#\s*include\s*[<"]\s*(
        d3d12(?:_\w+)?\.h | dxgi\d*\.h | d3dcommon\.h | dxcapi\.h |
        vulkan/[\w./]+ | vulkan\.h | vk_\w+\.h
    )""",
    re.VERBOSE | re.IGNORECASE,
)

ENGINE_INCLUDE = re.compile(r'#\s*include\s*[<"]pn/(\w+)/')

SOURCE_SUFFIXES = (".hpp", ".cpp", ".h", ".cc", ".inl")



def _is_generated(path: Path) -> bool:
    """True for paths inside a build or generated tree.

    The checks walk the source tree directly rather than asking git, so they
    must skip generated output themselves. A stray nested build/ directory
    otherwise fails the authorship check on CMake's own compiler-probe file.
    """
    return any(part in {"build", "out", "CMakeFiles", ".git"} or part.startswith("build-")
               for part in path.parts)

def module_of(path: Path, engine_root: Path) -> str | None:
    try:
        relative = path.relative_to(engine_root)
    except ValueError:
        return None
    return relative.parts[0] if relative.parts else None


def check(repo_root: Path) -> list[str]:
    engine_root = repo_root / "engine"
    failures: list[str] = []

    if not engine_root.is_dir():
        return ["engine/ directory not found"]

    for path in sorted(engine_root.rglob("*")):
        if _is_generated(path):
            continue
        if path.suffix not in SOURCE_SUFFIXES or not path.is_file():
            continue

        module = module_of(path, engine_root)
        if module is None:
            continue

        relative = path.relative_to(repo_root)
        text = path.read_text(encoding="utf-8")

        # Rule 2: graphics API confinement.
        for match in GRAPHICS_API_HEADERS.finditer(text):
            if module not in GRAPHICS_API_OWNERS:
                failures.append(
                    f"{relative}: module '{module}' includes the graphics API header "
                    f"'{match.group(1)}'. Only {sorted(GRAPHICS_API_OWNERS)} may. "
                    f"(ADR-0004 rule 1)"
                )

        # Rule 1: dependency direction.
        if module not in ALLOWED_DEPENDENCIES:
            failures.append(
                f"{relative}: module '{module}' is not declared in "
                f"ALLOWED_DEPENDENCIES. Add it to the graph in this script."
            )
            continue

        permitted = ALLOWED_DEPENDENCIES[module]
        for match in ENGINE_INCLUDE.finditer(text):
            target = match.group(1)
            if target == module:
                continue
            # Test sources may use the test framework regardless of layer.
            if target == "testing" and "tests" in path.parts:
                continue
            if target not in permitted:
                failures.append(
                    f"{relative}: module '{module}' includes from '{target}', which is "
                    f"not below it in the dependency graph. Permitted: "
                    f"{sorted(permitted) or 'nothing'}. (ADR-0004 rule 2)"
                )

    return failures


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    failures = check(repo_root)

    if failures:
        print("Module layering check FAILED", file=sys.stderr)
        for failure in failures:
            print(f"  {failure}", file=sys.stderr)
        print(
            "\nA cycle or an upward edge means a module boundary is wrong. Fix the "
            "boundary rather than widening the graph to accommodate the include.",
            file=sys.stderr,
        )
        return 1

    print("Module layering check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
