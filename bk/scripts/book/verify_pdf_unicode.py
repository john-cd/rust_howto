#!/usr/bin/env python3
"""Check extracted PDF text for lost Unicode and visible source markers."""

from __future__ import annotations

from pathlib import Path
import re
import sys

from pypdf import PdfReader


INVALID_CODE_POINTS = ("\ufffd", "\uffff")
FORBIDDEN_MARKERS = (
    "{{!crate",
    "{{#badges",
    "{{#{{#include}}",
    "cat~*",
    r"cat\~\*",
)
EMOJI_CHARACTER_LITERAL = re.compile(r"let\s+s\s*:\s*char\s*=\s*'😀'\s*;")


def main() -> int:
    """Verify required Unicode text and reject replacement glyphs in a PDF."""
    if len(sys.argv) != 2:
        print(f"usage: {Path(sys.argv[0]).name} <book.pdf>", file=sys.stderr)
        return 2

    pdf_path = Path(sys.argv[1])
    if not pdf_path.is_file():
        print(f"PDF does not exist: {pdf_path}", file=sys.stderr)
        return 2

    failures: list[str] = []
    replacement_counts = {code_point: 0 for code_point in INVALID_CODE_POINTS}
    found_emoji_literal = False
    found_pi = False
    reader = PdfReader(pdf_path)

    for page_number, page in enumerate(reader.pages, start=1):
        extracted = page.extract_text() or ""
        if page_number % 250 == 0:
            print(f"Checked {page_number} of {len(reader.pages)} pages...", flush=True)
        for code_point in INVALID_CODE_POINTS:
            replacement_counts[code_point] += extracted.count(code_point)
        for marker in FORBIDDEN_MARKERS:
            if marker in extracted:
                failures.append(f"page {page_number}: unresolved marker {marker!r}")
        found_emoji_literal = found_emoji_literal or bool(EMOJI_CHARACTER_LITERAL.search(extracted))
        found_pi = found_pi or "π" in extracted

    for code_point, count in replacement_counts.items():
        if count:
            failures.append(f"found {count} U+{ord(code_point):04X} replacement characters")
    if not found_emoji_literal:
        failures.append("the emoji character literal is missing or corrupted")
    if not found_pi:
        failures.append("the Greek pi symbol is missing or corrupted")

    if failures:
        print("PDF text verification failed:", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    print(f"Verified Unicode text in {pdf_path} ({len(reader.pages)} pages).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
