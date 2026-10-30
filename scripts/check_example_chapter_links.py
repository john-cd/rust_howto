#!/usr/bin/env python3
"""Report examples that no chapter uses, and chapters that have no example.

* An *orphan example* is a `.rs` file under `*/examples/` in `bk/crates` or
  `later/crates` that no chapter includes with `{{#include path.rs[:anchor]}}`
  (or `{{#rustdoc_include ...}}`). Entry points named `main.rs`, `lib.rs` and
  `mod.rs` are skipped (they only dispatch to sibling files), as are generated
  files and support modules whose directory's `main.rs` is included.
* A *chapter without an example* is a page under `bk/src`, `bk/drafts` or
  `later/src` that includes no `.rs` file. Pages whose only code is inline
  `rust` fences are listed separately. Index pages, reference lists, `.incl.md`
  tables and pages named in the exceptions file are skipped.
* A *broken include* names a `.rs` file that does not exist.
* An *unfenced include* is a `{{#include x.rs[:anchor]}}` line outside a code
  fence (in a page or an `.incl.md` fragment). mdBook pastes the raw Rust into
  the Markdown, so generic types turn into HTML tags and the code loses its
  formatting. Wrap it in a ```` ```rust ```` fence.
* A page that is not linked from its `SUMMARY.md` is listed too.

Exceptions: `scripts/example_chapter_exceptions.txt`, one path per line
(repository-relative, `#` starts a comment). A line ending in `/` matches a
directory prefix. Use sparingly: the goal is zero unexplained entries.

Usage: scripts/check_example_chapter_links.py [--csv] [--fail]
Exit status is 0 unless `--fail` is given and a gap remains.
"""

import argparse
import posixpath
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
EXAMPLE_ROOTS = ["bk/crates", "later/crates"]
BOOK_ROOTS = ["bk/src", "bk/drafts", "later/src"]
SKIP_PARTS = {"target", ".git", "node_modules", "temp"}
ENTRY_POINTS = {"main.rs", "lib.rs", "mod.rs", "build.rs"}
SKIP_PAGES = {"SUMMARY.md", "README.md", "index.md", "refs.incl.md"}
EXCEPTIONS_FILE = REPO / "scripts" / "example_chapter_exceptions.txt"

INCLUDE = re.compile(r"\{\{#(?:rustdoc_)?include\s+([^}\s:]+\.rs)(?::[^}\s]*)?\s*\}\}")
MD_INCLUDE = re.compile(r"\{\{#include\s+([^}\s:]+\.md)\s*\}\}")
INCLUDE_LINE = re.compile(r"^\s*\{\{#(?:rustdoc_)?include\s+\S+\.rs(?::\S*)?\s*\}\}\s*$")
FENCE = re.compile(r"^\s*(`{3,}|~{3,})")
RUST_FENCE = re.compile(r"^\s*```+\s*rust\b", re.M)
SUMMARY_LINK = re.compile(r"\]\(([^)\s#]+\.md)")


def rel(path: Path) -> str:
    return path.relative_to(REPO).as_posix()


def is_generated(name: str) -> bool:
    return name == "generated.rs" or name.endswith(("_generated.rs", "_capnp.rs"))


def load_exceptions() -> list[str]:
    if not EXCEPTIONS_FILE.exists():
        return []
    lines = [line.split("#", 1)[0].strip() for line in EXCEPTIONS_FILE.read_text().splitlines()]
    return [line for line in lines if line]


def excepted(path: str, exceptions: list[str]) -> bool:
    return any(path == e or (e.endswith("/") and path.startswith(e)) for e in exceptions)


def unfenced_includes(text: str) -> list[int]:
    """Return the 1-based line numbers of `.rs` includes outside a code fence."""
    lines, fence = [], None
    for number, line in enumerate(text.splitlines(), 1):
        match = FENCE.match(line)
        if match:
            marker = match.group(1)
            if fence is None:
                fence = marker
            elif marker[0] == fence[0] and len(marker) >= len(fence):
                fence = None
        elif fence is None and INCLUDE_LINE.match(line):
            lines.append(number)
    return lines


def walk(root: str, suffix: str):
    for path in sorted((REPO / root).rglob(f"*{suffix}")):
        if not SKIP_PARTS.intersection(path.relative_to(REPO).parts):
            yield path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--csv", action="store_true", help="print kind,path rows")
    parser.add_argument("--fail", action="store_true", help="exit 1 if a gap remains")
    args = parser.parse_args()
    exceptions = load_exceptions()

    pages = [p for root in BOOK_ROOTS for p in walk(root, ".md")]
    included: set[str] = set()
    with_example: set[str] = set()
    inline_only: set[str] = set()
    broken: list[str] = []
    unfenced: list[str] = []
    for page in pages:
        text = page.read_text(encoding="utf-8", errors="replace")
        unfenced += [f"{rel(page)}:{n}" for n in unfenced_includes(text)]
        # Fragments the page includes (for example `page.incl.md`) count too.
        for fragment in MD_INCLUDE.findall(text):
            fragment_path = page.parent / fragment
            if fragment_path.is_file():
                text += "\n" + fragment_path.read_text(encoding="utf-8", errors="replace")
        found = False
        for target in INCLUDE.findall(text):
            resolved = posixpath.normpath(posixpath.join(rel(page.parent), target))
            included.add(resolved)
            found = True
        for target in INCLUDE.findall(text):
            resolved = posixpath.normpath(posixpath.join(rel(page.parent), target))
            if not (REPO / resolved).exists():
                broken.append(f"{rel(page)} -> {target}")
        if found:
            with_example.add(rel(page))
        elif RUST_FENCE.search(text):
            inline_only.add(rel(page))

    orphan_examples = []
    for root in EXAMPLE_ROOTS:
        for path in walk(root, ".rs"):
            r = rel(path)
            if "/examples/" not in r or path.name in ENTRY_POINTS:
                continue
            if is_generated(path.name):
                continue
            # A support module of an included multi-file example is covered
            # when the directory's entry point is included.
            if (
                f"{posixpath.dirname(r)}/main.rs" in included
                or f"{posixpath.dirname(r)}/mod.rs" in included
            ):
                continue
            if r not in included and not excepted(r, exceptions):
                orphan_examples.append(r)

    no_example, inline = [], []
    for page in pages:
        r = rel(page)
        if page.name in SKIP_PAGES or page.name.endswith(".incl.md") or "/refs/" in r:
            continue
        if excepted(r, exceptions) or r in with_example:
            continue
        (inline if r in inline_only else no_example).append(r)

    unlinked = []
    for root in BOOK_ROOTS:
        summary = REPO / root / "SUMMARY.md"
        if not summary.exists():
            continue
        linked = {
            posixpath.normpath(posixpath.join(root, m))
            for m in SUMMARY_LINK.findall(summary.read_text(encoding="utf-8", errors="replace"))
        }
        for page in walk(root, ".md"):
            r = rel(page)
            if (
                page.name in {"SUMMARY.md", "README.md"}
                or page.name.endswith(".incl.md")
                or "/refs/" in r
                or r in linked
                or excepted(r, exceptions)
            ):
                continue
            unlinked.append(r)

    groups = [
        ("orphan-example", "Examples that no chapter includes", orphan_examples),
        ("chapter-without-example", "Chapters with no example (no .rs include, no rust fence)", no_example),
        ("chapter-inline-only", "Chapters whose only code is inline `rust` fences", inline),
        ("broken-include", "Includes that point to a missing file", broken),
        ("unfenced-include", "Rust includes outside a code fence (rendered as prose/HTML)", unfenced),
        ("not-in-summary", "Pages not linked from their SUMMARY.md", unlinked),
    ]
    if args.csv:
        for kind, _, items in groups:
            for item in items:
                print(f"{kind},{item}")
    else:
        for kind, title, items in groups:
            print(f"== {title}: {len(items)}")
            for item in items:
                print(f"  {item}")
    gaps = len(orphan_examples) + len(no_example) + len(unlinked) + len(broken) + len(unfenced)
    return 1 if args.fail and gaps else 0


if __name__ == "__main__":
    sys.exit(main())
