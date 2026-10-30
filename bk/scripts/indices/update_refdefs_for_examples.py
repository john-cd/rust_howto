#!/usr/bin/env python3
"""Rebuild the example references of `src/indices/refs.incl.md`.

Cross-platform replacement for `update_refdefs_for_index_of_examples.sh`.
Every `[ex~...]: target` definition found in another `src/**/refs.incl.md` is
copied into `src/indices/refs.incl.md`, with the target rewritten relative to
`src/indices/`. Non-example lines (such as the leading comment) are kept.

Usage (from `bk/`):
    python scripts/indices/update_refdefs_for_examples.py generate
    python scripts/indices/update_refdefs_for_examples.py check

`generate` rewrites the file (hand-written entries are kept; derived ones are
added or corrected, and the entries are sorted by label); `check` exits 1 if a
derived entry is missing or differs.
"""

import argparse
import posixpath
import re
import sys
from pathlib import Path

EX_DEF = re.compile(r"^(\[ex~.+?\]):\s*(\S+)\s*$")


LABEL = re.compile(r"^(\[[^\]]+\]):\s*(\S+)\s*$")


def derived(src: Path, target: Path) -> dict[str, str]:
    """Return `{label: destination}` for every example reference to copy."""
    refs: dict[str, str] = {}
    for fragment in sorted(src.rglob("refs.incl.md")):
        if fragment == target:
            continue
        rel_dir = fragment.parent.relative_to(src).as_posix()
        for line in fragment.read_text(encoding="utf-8").splitlines():
            match = EX_DEF.match(line)
            if not match:
                continue
            label, dest = match.groups()
            if re.match(r"[a-z][a-z0-9+.-]*:", dest):
                refs[label] = dest
                continue
            path, _, anchor = dest.partition("#")
            new = posixpath.normpath(posixpath.join("..", rel_dir, path))
            refs[label] = new + ("#" + anchor if anchor else "")
    return refs


def build(book_root: Path) -> tuple[str, list[str]]:
    """Return the wanted file text and the derived labels that are missing or differ.

    Entries already in the file are kept, including hand-written ones that no
    chapter fragment defines; derived entries are added or corrected.
    """
    src = book_root / "src"
    target = src / "indices" / "refs.incl.md"
    header, refs = [], {}
    for line in target.read_text(encoding="utf-8").splitlines():
        match = LABEL.match(line)
        if match:
            refs[match.group(1)] = match.group(2)
        elif line.strip():
            header.append(line)
    stale = []
    for label, dest in derived(src, target).items():
        if refs.get(label) != dest:
            stale.append(label)
            refs[label] = dest
    body = [f"{label}: {dest}" for label, dest in sorted(refs.items())]
    return "\n".join(header) + "\n\n" + "\n".join(body) + "\n", stale


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("action", choices=["generate", "check"])
    parser.add_argument("--book-root", type=Path, default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    target = args.book_root / "src" / "indices" / "refs.incl.md"
    wanted, stale = build(args.book_root)
    if args.action == "check":
        for label in stale:
            print(f"missing or outdated in {target}: {label}", file=sys.stderr)
        return 1 if stale else 0
    target.write_text(wanted, encoding="utf-8", newline="\n")
    print("DONE")
    return 0


if __name__ == "__main__":
    sys.exit(main())
