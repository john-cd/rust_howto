# Purpose: Fail when a TODO placeholder is visible in published Markdown.
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
"""Fail when a TODO placeholder is visible in published Markdown."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
TODO_PLACEHOLDER = re.compile(r"\bTODO\b(?:\s+[\w?!.:/-]+){0,3}\s*$")
FENCE = re.compile(r"^\s{0,3}(`{3,}|~{3,})")
REVIEW_MARKER = re.compile(r"^\s*<!--\s*TODO Reviewed \?\?-\?\?-\d{4}\s*-->\s*$")


def visible_todo_lines(path: Path) -> list[tuple[int, str]]:
    in_fence = False
    fence_char = ""
    in_hidden_div = False
    matches = []

    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if REVIEW_MARKER.match(line):
            continue

        fence = FENCE.match(line)
        if fence:
            marker = fence.group(1)
            if not in_fence:
                in_fence = True
                fence_char = marker[0]
            elif marker[0] == fence_char and len(marker) >= 3:
                in_fence = False
            continue
        if in_fence:
            continue

        if re.search(r"<div\s+class=[\"']hidden[\"']\s*>", line, re.IGNORECASE):
            in_hidden_div = True
        if not in_hidden_div and TODO_PLACEHOLDER.search(line):
            matches.append((line_number, line.strip()))
        if re.search(r"</div\s*>", line, re.IGNORECASE):
            in_hidden_div = False

    return matches


def main() -> int:
    paths = [ROOT / "LICENSE", *sorted((ROOT / "bk" / "src").rglob("*.md"))]
    failures = []
    for path in paths:
        failures.extend((path, line, text) for line, text in visible_todo_lines(path))

    if failures:
        for path, line, text in failures:
            print(f"{path.relative_to(ROOT)}:{line}: visible TODO: {text}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
