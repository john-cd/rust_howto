#!/usr/bin/env bash
# Purpose: Replace inline Markdown links with reference-style links.
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

usage() {
  echo "Usage: $0 [--write] <root folder of book>" >&2
}

write=0
if [[ "${1:-}" == "--write" ]]; then
  write=1
  shift
fi
if [[ $# -ne 1 ]]; then
  usage
  exit 2
fi

root="$(realpath -- "$1")"
if [[ ! -d "$root/src" ]]; then
  echo "Error: '$root' does not contain src/." >&2
  exit 1
fi

python3 - "$root" "$write" <<'PY'
from __future__ import annotations

import difflib
import hashlib
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
write = sys.argv[2] == "1"
link_pattern = re.compile(r"(?<!!)\[([^\]\n]+)\]\((https?://[^\s)]+)\)")
excluded = {"SUMMARY.md", "refs.incl.md"}
refs: dict[Path, set[tuple[str, str]]] = {}
changes: list[tuple[Path, str, str]] = []

def transform(path: Path, text: str) -> str:
    fenced = False
    fence_marker = ""
    output: list[str] = []
    definitions: set[tuple[str, str]] = set()

    for line in text.splitlines(keepends=True):
        fence = re.match(r"^\s{0,3}(```+|~~~+)", line)
        if fence:
            marker = fence.group(1)
            if not fenced:
                fenced, fence_marker = True, marker[0]
            elif marker[0] == fence_marker:
                fenced = False
            output.append(line)
            continue
        if fenced:
            output.append(line)
            continue

        def replace(match: re.Match[str]) -> str:
            label, url = match.groups()
            if "github.com/" in url and "/issues/" in url:
                return match.group(0)
            digest = hashlib.sha256(url.encode()).hexdigest()[:12]
            ref = f"url~{digest}"
            definitions.add((ref, url))
            return f"[{label}][{ref}]"

        # Avoid rewriting links inside inline code spans.
        spans = re.split(r"(`+[^\n]*?`+)", line)
        for index in range(0, len(spans), 2):
            spans[index] = link_pattern.sub(replace, spans[index])
        output.append("".join(spans))

    if definitions:
        refs_path = path.parent / "refs.incl.md"
        refs.setdefault(refs_path, set()).update(definitions)
    return "".join(output)

for base in (root / "src", root / "drafts", root.parent / "later" / "src"):
    if not base.is_dir():
        continue
    for path in sorted(base.rglob("*.md")):
        if path.name in excluded or path.name.endswith("refs.md"):
            continue
        before = path.read_text(encoding="utf-8")
        after = transform(path, before)
        if after != before:
            changes.append((path, before, after))

if not changes:
    print("No eligible inline HTTP(S) links found.")
    sys.exit(0)

if not write:
    for path, before, after in changes:
        print("".join(difflib.unified_diff(
            before.splitlines(keepends=True),
            after.splitlines(keepends=True),
            fromfile=str(path),
            tofile=str(path) + " (updated)",
        )))
    print(f"Previewed {len(changes)} file(s); rerun with --write to apply.")
    sys.exit(0)

for path, _, after in changes:
    path.write_text(after, encoding="utf-8", newline="")

for path, definitions in refs.items():
    existing = path.read_text(encoding="utf-8") if path.exists() else ""
    present = set(re.findall(r"^\[(url~[0-9a-f]{12})\]:", existing, re.M))
    additions = [
        f"[{ref}]: {url}\n"
        for ref, url in sorted(definitions)
        if ref not in present
    ]
    if additions:
        separator = "" if not existing or existing.endswith("\n\n") else "\n"
        path.write_text(existing + separator + "".join(additions), encoding="utf-8", newline="")

print(f"Updated {len(changes)} Markdown file(s) and their reference files.")
PY
