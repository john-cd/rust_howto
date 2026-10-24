# TODO Reviewed ??-??-2026

"""Focused integration test for mdbook-utils reference generation."""

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
    text = path.read_text(encoding="utf-8")

    def replace(match: re.Match[str]) -> str:
        included = (path.parent / match.group(1).strip()).resolve()
        if not included.is_relative_to(source_root.resolve()):
            raise AssertionError(f"include escapes the copied book source: {included}")
        return expand_includes(included, source_root)

    return INCLUDE.sub(replace, text)


class RefdefsIntegrationTests(unittest.TestCase):
    def test_review_marker_survives_generated_fragment_refresh(self) -> None:
        """Preserves the human review marker without duplicating it."""
        marker = b"<!-- TODO Reviewed ??-??-2026 -->\n"
        generated = b"# Reference Definitions\n\n[crate]: https://example.com\n"
        current = marker + b"# Reference Definitions\n\n[old]: https://example.com\n"

        self.assertEqual(
            refdefs._preserve_review_marker(current, generated), marker + generated
        )
        self.assertEqual(
            refdefs._preserve_review_marker(marker + generated, generated),
            marker + generated,
        )
        self.assertEqual(
            refdefs._preserve_review_marker(generated, generated), generated
        )

    def test_workspace_aliases_and_includes_are_deterministic(self) -> None:
        tool = refdefs._resolve_tool()
        with tempfile.TemporaryDirectory(
            prefix="rust-howto-refdefs-test-"
        ) as temporary:
            book_root = Path(temporary) / "book"
            shutil.copytree(FIXTURE, book_root)
            fragment = book_root / "src" / "refs" / "dependency-refs.md"
            manual_refs = book_root / "src" / "refs" / "crate-refs.md"
            link_refs = book_root / "src" / "refs" / "link-refs.md"
            chapter = book_root / "src" / "chapter.md"
            chapter_before = chapter.read_bytes()
            manual_before = manual_refs.read_bytes()
            link_refs_before = link_refs.read_bytes()

            check_output = io.StringIO()
            with contextlib.redirect_stderr(check_output):
                self.assertEqual(refdefs.process(book_root, "check", tool), 1)
            self.assertIn("stale", check_output.getvalue())
            self.assertFalse(fragment.exists())
            self.assertEqual(chapter.read_bytes(), chapter_before)
            self.assertEqual(manual_refs.read_bytes(), manual_before)
            self.assertEqual(link_refs.read_bytes(), link_refs_before)

            self.assertEqual(refdefs.process(book_root, "generate", tool), 0)
            generated = fragment.read_bytes()
            text = generated.decode("utf-8")
            self.assertIn(
                "[fixture_library_lib]: https://docs.rs/fixture_library_lib", text
            )
            self.assertIn(
                "[fixture_library_lib-github]: https://example.com/fixture-library",
                text,
            )
            self.assertEqual(chapter.read_bytes(), chapter_before)
            self.assertEqual(manual_refs.read_bytes(), manual_before)
            self.assertEqual(link_refs.read_bytes(), link_refs_before)

            expanded = expand_includes(chapter, book_root / "src")
            self.assertIn("[fixture library][fixture_library_lib]", expanded)
            self.assertIn(
                "[fixture_library_lib]: https://docs.rs/fixture_library_lib", expanded
            )
            self.assertIn("[manual]: https://example.com/manual", expanded)
            self.assertNotIn("{{#include", expanded)
            self.assertTrue(link_refs.is_file())

            stale = b"[stale]: https://example.com/stale\n"
            fragment.write_bytes(stale)
            drift_output = io.StringIO()
            with contextlib.redirect_stderr(drift_output):
                self.assertEqual(refdefs.process(book_root, "check", tool), 1)
            self.assertIn("stale", drift_output.getvalue())
            self.assertEqual(fragment.read_bytes(), stale)
            self.assertEqual(chapter.read_bytes(), chapter_before)
            self.assertEqual(manual_refs.read_bytes(), manual_before)
            self.assertEqual(link_refs.read_bytes(), link_refs_before)

            self.assertEqual(refdefs.process(book_root, "generate", tool), 0)
            self.assertEqual(fragment.read_bytes(), generated)

            timestamp = fragment.stat().st_mtime_ns
            self.assertEqual(refdefs.process(book_root, "generate", tool), 0)
            self.assertEqual(fragment.read_bytes(), generated)
            self.assertEqual(fragment.stat().st_mtime_ns, timestamp)
            self.assertEqual(refdefs.process(book_root, "check", tool), 0)


if __name__ == "__main__":
    unittest.main()
