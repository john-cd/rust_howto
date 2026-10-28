# Purpose: Integration coverage for merging dependency refs into curated crate refs.
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
# TODO Reviewed ??-??-2026.

"""Integration coverage for merging dependency refs into curated crate refs."""

from __future__ import annotations

import contextlib
import io
from pathlib import Path
import re
import shutil
import tempfile
import unittest

import refdefs

FIXTURE = Path(__file__).parent / "testdata" / "refdefs"
INCLUDE = re.compile(r"\{\{#include\s+([^}]+)}}")


def expand_includes(path: Path, source_root: Path) -> str:
    """Expand fixture includes while rejecting paths outside its source tree."""
    text = path.read_text(encoding="utf-8")

    def replace(match: re.Match[str]) -> str:
        """Expand one include relative to its containing Markdown file."""
        included = (path.parent / match.group(1).strip()).resolve()
        if not included.is_relative_to(source_root.resolve()):
            raise AssertionError(f"include escapes the copied book source: {included}")
        return expand_includes(included, source_root)

    return INCLUDE.sub(replace, text)


class RefdefsIntegrationTests(unittest.TestCase):
    """Verify canonical labels, curated destinations, and read-only checks."""

    def test_normalization_preserves_curated_links_and_review_marker(self) -> None:
        """Merge hyphenated packages without replacing curated docs or repositories."""
        current = b"<!-- TODO Reviewed ??-??-2026 -->\n\n[c~fixture-library~docs]: https://example.com/guide\n"
        generated = b"[fixture_library_lib]: https://docs.rs/fixture_library_lib\n[fixture_library_lib-crate]: https://crates.io/crates/fixture_library_lib\n[fixture_library_lib-github]: https://example.com/fixture.git (*)\n[]: https://docs.rs/\n"
        merged = refdefs._merge_refs(FIXTURE, current, generated)
        self.assertTrue(merged.startswith(b"<!-- TODO Reviewed ??-??-2026 -->\n"))
        self.assertIn(b"[c~fixture-library~docs]: https://example.com/guide\n", merged)
        self.assertIn(b"[c~fixture-library~crates.io]: https://crates.io/crates/fixture-library\n", merged)
        self.assertIn(b"[c~fixture-library~repo]: https://example.com/fixture\n", merged)
        self.assertNotIn(b"[]:", merged)
        self.assertEqual(refdefs._merge_refs(FIXTURE, merged, generated), merged)

    def test_workspace_aliases_and_includes_are_deterministic(self) -> None:
        """Merge alias metadata, retain manual refs, detect omissions, and avoid no-op writes."""
        tool = refdefs._resolve_tool()
        with tempfile.TemporaryDirectory(prefix="rust-howto-refdefs-test-") as temporary:
            book_root = Path(temporary) / "book"
            shutil.copytree(FIXTURE, book_root)
            fragment = book_root / "src/refs/crate-refs.md"
            chapter = book_root / "src/chapter.md"
            chapter_before = chapter.read_bytes()
            original = fragment.read_bytes()
            with contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(refdefs.process(book_root, "check", tool), 1)
            self.assertEqual(fragment.read_bytes(), original)
            self.assertEqual(refdefs.process(book_root, "generate", tool), 0)
            generated = fragment.read_bytes()
            self.assertIn(b"[c~fixture-library~docs]: https://docs.rs/fixture-library", generated)
            self.assertIn(b"[c~fixture-library~repo]: https://example.com/fixture-library", generated)
            self.assertIn(b"[manual]: https://example.com/manual", generated)
            self.assertEqual(chapter.read_bytes(), chapter_before)
            expanded = expand_includes(chapter, book_root / "src")
            self.assertIn("[fixture library][c~fixture-library~docs]", expanded)
            self.assertIn("[c~fixture-library~docs]: https://docs.rs/fixture-library", expanded)
            self.assertNotIn("{{#include", expanded)
            fragment.write_bytes(original)
            with contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(refdefs.process(book_root, "check", tool), 1)
            self.assertEqual(fragment.read_bytes(), original)
            self.assertEqual(refdefs.process(book_root, "generate", tool), 0)
            self.assertEqual(fragment.read_bytes(), generated)
            timestamp = fragment.stat().st_mtime_ns
            self.assertEqual(refdefs.process(book_root, "generate", tool), 0)
            self.assertEqual(fragment.stat().st_mtime_ns, timestamp)
            self.assertEqual(refdefs.process(book_root, "check", tool), 0)
            self.assertFalse((book_root / "src/refs/dependency-refs.md").exists())


if __name__ == "__main__":
    unittest.main()
