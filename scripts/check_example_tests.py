#!/usr/bin/env python3
"""List example source files that do not follow the testing convention.

Convention (see the tracking issue for "Test all book examples"):

* every example file defines a private `fn main()`;
* a file that belongs to a multi-file example (its directory has a `main.rs`
  that is not the file itself) also defines `pub fn run()`, called by the
  directory's `main.rs`;
* every file with a `main()` has a `#[cfg(test)]` module with a test that
  calls `main()` and asserts on observable behavior (the script can only
  flag files with no assert/unwrap/expect at all; review the rest by hand). A test module may state
  `output-only` in a comment when the example has nothing but printing to check.

Usage: scripts/check_example_tests.py [--summary] [--csv] [ROOT ...]
Default roots: bk/crates later/crates xmpl playground tools.
Exit status is 0 (the report is informational).
"""

import argparse
import re
import sys
from collections import defaultdict
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DEFAULT_ROOTS = ["bk/crates", "later/crates", "xmpl", "playground", "tools"]
SKIP_PARTS = {"target", ".git", "node_modules"}

FN_MAIN = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+main\s*\(", re.M)
FN_RUN = re.compile(r"^\s*pub(?:\([^)]*\))?\s+(?:async\s+)?fn\s+run\s*\(", re.M)
CFG_TEST = re.compile(r"#\[cfg\(test\)\]")
TEST_ATTR = re.compile(r"#\[(?:\w+::)*test\b")
MAIN_CALL = re.compile(r"(?<![\w:])(?:super::)?main\s*\(\s*\)")


def crate_root(path: Path) -> str:
    for parent in path.parents:
        if (parent / "Cargo.toml").exists():
            return parent.relative_to(REPO).as_posix()
    return "?"


def classify(path: Path):
    text = path.read_text(encoding="utf-8", errors="replace")
    has_main = bool(FN_MAIN.search(text))
    has_run = bool(FN_RUN.search(text))
    has_cfg_test = bool(CFG_TEST.search(text)) or bool(TEST_ATTR.search(text))
    tail = text[CFG_TEST.search(text).start():] if CFG_TEST.search(text) else ""
    calls_main = bool(MAIN_CALL.search(tail)) if tail else False
    asserts = bool(
        re.search(r"assert|should_panic|output-only", text, re.I)
        or re.search(r"\.expect\(|\.unwrap\(", tail)
    )
    return has_main, has_run, has_cfg_test, calls_main, asserts


def is_multi_file_member(path: Path) -> bool:
    return path.name != "main.rs" and (path.parent / "main.rs").exists()


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("roots", nargs="*", default=DEFAULT_ROOTS)
    ap.add_argument("--summary", action="store_true", help="only per-crate counts")
    ap.add_argument("--csv", action="store_true", help="file,problem rows")
    args = ap.parse_args()

    problems = defaultdict(list)  # crate -> [(file, problem)]
    totals = defaultdict(lambda: [0, 0])  # crate -> [files with main, compliant]
    for root in args.roots:
        for path in sorted((REPO / root).rglob("*.rs")):
            rel = path.relative_to(REPO)
            if SKIP_PARTS & set(rel.parts):
                continue
            # Only example sources: examples/, src/bin/, or tests are excluded.
            parts = rel.parts
            if "examples" not in parts and "bin" not in parts and "src" not in parts:
                continue
            if "tests" in parts or path.name == "build.rs":
                continue
            has_main, has_run, has_test, calls_main, asserts = classify(path)
            if not has_main:
                continue
            crate = crate_root(path)
            totals[crate][0] += 1
            issues = []
            if is_multi_file_member(path) and not has_run:
                issues.append("missing pub fn run()")
            if not has_test:
                issues.append("no tests")
            elif not calls_main and path.name != "main.rs":
                issues.append("no test calls main()")
            elif not asserts:
                issues.append("tests make no assertions")
            if issues:
                problems[crate].append((rel.as_posix(), "; ".join(issues)))
            else:
                totals[crate][1] += 1

    if args.csv:
        for crate in sorted(problems):
            for f, p in problems[crate]:
                print(f"{f},{p}")
        return
    grand = [0, 0]
    for crate in sorted(totals):
        n, ok = totals[crate]
        grand[0] += n
        grand[1] += ok
        print(f"{crate}: {ok}/{n} compliant")
        if not args.summary:
            for f, p in problems.get(crate, []):
                print(f"    {f}: {p}")
    print(f"TOTAL: {grand[1]}/{grand[0]} compliant")


if __name__ == "__main__":
    sys.exit(main())
