#!/usr/bin/env python3
"""Rewrite crate badge sets in section headers to the `{{#badges}}` directive (#2132).

Idempotent: a badge block is rewritten only if it consists of the four crate badges
(docs.rs, crates.io, repository, lib.rs) with one `{{hi:<crate>}}`, optional extra
`{{hi:...}}` terms, then category badges that each carry a `{{hi:<Title>}}` (the directive
always emits one). Anything else is left untouched.

Usage (from the repository root): migrate_badge_sets.py [--check] [ROOT ...]   (default roots: bk/src later/src bk/drafts)
"""
import re
import sys
from pathlib import Path

TOKEN = re.compile(
    r"\[!\[(?P<alt>[^\]]*)\]\[(?P<bref>[^\]]+)~badge\]\]\[(?P<ref>[^\]]+)\]"
    r"|\{\{hi:(?P<hi>[^}]*)\}\}"
)


def tokenize(block: str):
    pos, out = 0, []
    for m in TOKEN.finditer(block):
        if block[pos:m.start()].strip():
            return None
        pos = m.end()
        if m["hi"] is not None:
            out.append(("hi", m["hi"]))
        elif m["bref"] == m["ref"]:
            out.append(("badge", m["alt"], m["ref"]))
        else:
            return None
    if block[pos:].strip():
        return None
    return out


def convert(block: str):
    toks = tokenize(block)
    if not toks or toks[0][0] != "badge":
        return None
    m = re.fullmatch(r"c~(.+)~docs", toks[0][2])
    if not m or toks[0][1] != m[1]:
        return None
    name = m[1]
    i = 1
    hi_seen = False
    if toks[i:i + 1] == [("hi", name)]:
        hi_seen, i = True, i + 1
    for suffix in ("crates.io", "repo", "lib.rs"):
        if toks[i:i + 1] != [("badge", f"{name}~{suffix}", f"c~{name}~{suffix}")]:
            return None
        i += 1
    if not hi_seen:
        if toks[i:i + 1] != [("hi", name)]:
            return None
        i += 1
    extras = []
    while i < len(toks) and toks[i][0] == "hi":
        extras.append(toks[i][1])
        i += 1
    cats = []
    while i < len(toks):
        t = toks[i]
        c = re.fullmatch(r"cat~(.+)", t[2]) if t[0] == "badge" else None
        if not c or t[1] != f"cat~{c[1]}":
            return None
        if i + 1 >= len(toks) or toks[i + 1][0] != "hi":
            return None  # directive always adds a title index entry
        title = toks[i + 1][1]
        cats.append(f'{c[1]}="{title}"' if re.search(r"[\s,]", title) else f"{c[1]}={title}")
        i += 2
    if re.search(r"[\s}]", name):
        return None
    return "{{#badges " + " ".join([name, *cats]) + "}}" + "".join(f"{{{{hi:{e}}}}}" for e in extras)


def migrate(text: str):
    blocks = re.split(r"(\n[ \t]*\n)", text)
    changed = 0
    for k in range(0, len(blocks), 2):
        b = blocks[k]
        if "~lib.rs~badge" not in b:
            continue
        # keep leading/trailing context lines (e.g. headings) outside the badge lines
        lines = b.split("\n")
        idx = [j for j, l in enumerate(lines) if l.startswith("[![") and "~badge]" in l]
        if not idx or idx != list(range(idx[0], idx[-1] + 1)):
            continue
        new = convert("\n".join(lines[idx[0]:idx[-1] + 1]))
        if new:
            lines[idx[0]:idx[-1] + 1] = [new]
            blocks[k] = "\n".join(lines)
            changed += 1
    return "".join(blocks), changed


# Draft whose inline triple backticks confuse the Markdown parser; directives are not expanded there.
EXCLUDE = {"bk/drafts/categories/development-tools_testing/testing.md"}


def main(argv):
    check = "--check" in argv
    roots = [a for a in argv if not a.startswith("--")] or ["bk/src", "later/src", "bk/drafts"]
    total = 0
    for root in roots:
        for p in sorted(Path(root).rglob("*.md")):
            if p.as_posix() in EXCLUDE:
                continue
            text = p.read_text(encoding="utf-8")
            new, n = migrate(text)
            if n:
                total += n
                print(f"{p}: {n}")
                if not check:
                    p.write_text(new, encoding="utf-8", newline="")
    print(f"{total} block(s) {'would be ' if check else ''}rewritten")
    return 1 if check and total else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
