#!/usr/bin/env python3
"""Rewrite compatible crate badge sets in Markdown blocks to `{{#badges}}` (#2132, #2145).

Idempotent: complete standard crate badge groups (docs.rs, crates.io, repository, lib.rs)
and their category badges are replaced while unsupported groups are left untouched. Website
badges and standalone `{{hi:...}}` terms retain their original positions. A category without
`{{hi:<Title>}}` uses `slug=` to omit its index term.

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
    """Parse a badge block into badge and index-term tokens, or reject mixed text."""
    pos, out = 0, []
    for m in TOKEN.finditer(block):
        if block[pos : m.start()].strip():
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


def crate_badge_name(token, suffix: str):
    """Return the canonical crate name for a standard crate badge token."""
    if token[0] != "badge":
        return None
    match = re.fullmatch(r"c~(.+)~" + re.escape(suffix), token[2])
    if not match:
        return None
    name = match[1]
    alt = name if suffix == "docs" else f"{name}~{suffix}"
    return name if token[1] == alt else None


def category_badge_name(token):
    """Return the category slug for a canonical category badge token."""
    if token[0] != "badge":
        return None
    match = re.fullmatch(r"cat~(.+)", token[2])
    if not match or token[1] != f"cat~{match[1]}":
        return None
    return match[1]


def convert_group(tokens):
    """Convert one complete crate badge set and its following category badges."""
    i = 0
    website = None
    if crate_badge_name(tokens[0], "website"):
        website = tokens[0]
        i += 1

    name = crate_badge_name(tokens[i], "docs") if i < len(tokens) else None
    if not name:
        return None
    i += 1

    hi_seen = False
    if tokens[i : i + 1] == [("hi", name)]:
        hi_seen, i = True, i + 1
    for suffix in ("crates.io", "repo", "lib.rs"):
        if i >= len(tokens) or crate_badge_name(tokens[i], suffix) != name:
            return None
        i += 1
    if not hi_seen:
        if tokens[i : i + 1] != [("hi", name)]:
            return None
        i += 1

    extras = []
    while i < len(tokens) and tokens[i][0] == "hi":
        extras.append(tokens[i][1])
        i += 1
    parts = ["{{#badges " + name + "}}"]
    parts.extend(f"{{{{hi:{extra}}}}}" for extra in extras)
    categories = []

    def flush_categories():
        """Emit any collected category badges and their title index terms."""
        if categories:
            parts.append("{{#categories " + ",".join(categories) + "}}")
            categories.clear()

    while i < len(tokens):
        category = category_badge_name(tokens[i])
        if category:
            if i + 1 < len(tokens) and tokens[i + 1][0] == "hi":
                title = tokens[i + 1][1]
                categories.append(
                    f'{category}="{title}"'
                    if title and re.search(r"[\s,]", title)
                    else f"{category}={title}"
                )
                i += 2
            else:
                categories.append(f"{category}=")
                i += 1
        elif tokens[i][0] == "hi":
            flush_categories()
            parts.append(f"{{{{hi:{tokens[i][1]}}}}}")
            i += 1
        else:
            return None
    flush_categories()

    if re.search(r"[\s}]", name):
        return None
    if website:
        # Keep this badge as Markdown: the standalone badge directive adds its own index term.
        website_markdown = f"[![{website[1]}][{website[2]}~badge]][{website[2]}]"
        parts.insert(0, website_markdown)
    return " ".join(parts)


def convert(block: str):
    """Convert complete crate badge sets while retaining unsupported groups verbatim."""
    tokens = tokenize(block)
    if not tokens:
        return None

    spans = [(match.start(), match.end()) for match in TOKEN.finditer(block)]
    starts = []
    i = 0
    while i < len(tokens):
        website_name = crate_badge_name(tokens[i], "website")
        if website_name:
            if i + 1 < len(tokens) and crate_badge_name(tokens[i + 1], "docs") == website_name:
                starts.append(i)
                i += 2
                continue
        if crate_badge_name(tokens[i], "docs"):
            starts.append(i)
        i += 1

    replacements = []
    for index, start in enumerate(starts):
        end = starts[index + 1] if index + 1 < len(starts) else len(tokens)
        name = crate_badge_name(tokens[start], "website") or crate_badge_name(
            tokens[start], "docs"
        )
        for position in range(start + 1, end):
            token = tokens[position]
            if token[0] == "badge" and not category_badge_name(token) and not any(
                crate_badge_name(token, suffix) == name
                for suffix in ("docs", "crates.io", "repo", "lib.rs")
            ):
                end = position
                break

        group = convert_group(tokens[start:end])
        if group is not None:
            replacements.append((spans[start][0], spans[end - 1][1], group))

    if not replacements:
        return None

    converted = block
    for start, end, replacement in reversed(replacements):
        converted = converted[:start] + replacement + converted[end:]
    return converted


def convert_table_row(line: str):
    """Convert complete badge sets contained in individual Markdown table cells."""
    stripped = line.strip()
    if not stripped.startswith("|") or not stripped.endswith("|"):
        return line, 0

    cells = line.split("|")
    changed = 0
    for index in range(1, len(cells) - 1):
        cell = cells[index]
        value = cell.strip()
        if "~lib.rs~badge" not in value:
            continue
        converted = convert(value)
        if converted:
            leading = cell[: len(cell) - len(cell.lstrip())]
            trailing = cell[len(cell.rstrip()) :]
            cells[index] = leading + converted + trailing
            changed += 1
    return "|".join(cells), changed


def migrate(text: str):
    """Rewrite compatible badge blocks in Markdown text and return the count."""
    blocks = re.split(r"(\n[ \t]*\n)", text)
    changed = 0
    for k in range(0, len(blocks), 2):
        b = blocks[k]
        if "~lib.rs~badge" not in b:
            continue
        # keep leading/trailing context lines (e.g. headings) outside the badge lines
        lines = b.split("\n")
        idx = [
            j
            for j, line in enumerate(lines)
            if line.startswith("[![") and "~badge]" in line
        ]
        if idx and idx == list(range(idx[0], idx[-1] + 1)):
            new = convert("\n".join(lines[idx[0] : idx[-1] + 1]))
            if new:
                lines[idx[0] : idx[-1] + 1] = [new]
                blocks[k] = "\n".join(lines)
                changed += 1
                continue
        for line_index, line in enumerate(lines):
            if "~lib.rs~badge" not in line:
                continue
            new, count = convert_table_row(line)
            if count:
                lines[line_index] = new
                changed += count
        blocks[k] = "\n".join(lines)
    return "".join(blocks), changed


# Draft whose inline triple backticks confuse the Markdown parser; directives are not expanded there.
EXCLUDE = {"bk/drafts/categories/development-tools_testing/testing.md"}


def main(argv):
    """Migrate the selected Markdown roots, or report pending changes in check mode."""
    check = "--check" in argv
    roots = [a for a in argv if not a.startswith("--")] or [
        "bk/src",
        "later/src",
        "bk/drafts",
    ]
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
