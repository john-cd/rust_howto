"""Regression tests for validation of rendered entry pages."""

from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

CHECKER = Path(__file__).parents[1] / "check_rendered_entry_links.py"
PAGES = (
    "index.html",
    "introduction.html",
    "introduction/index.html",
    "categories/development-tools/benchmark_regressions.html",
    "categories/development-tools/profiling.html",
    "appendices/contributing/mdBook_pre-processors.html",
    "appendices/contributing/documentation_generation.html",
)


class RenderedEntryLinksTests(unittest.TestCase):
    """Reject incomplete artifacts and accept all required rendered pages."""

    def run_checker(
        self, root: Path, *options: str
    ) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(CHECKER), str(root), *options],
            capture_output=True,
            check=False,
            text=True,
        )

    def write_required_pages(self, root: Path) -> None:
        for page in PAGES:
            path = root / page
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("<!doctype html><html></html>\n", encoding="utf-8")

    def test_missing_or_non_directory_root_fails(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            missing = root / "missing"
            result = self.run_checker(missing, "--all-chapters")
            self.assertEqual(result.returncode, 1)
            self.assertIn("does not exist", result.stderr)

            not_directory = root / "book.html"
            not_directory.write_text("<html></html>", encoding="utf-8")
            result = self.run_checker(not_directory)
            self.assertEqual(result.returncode, 1)

    def test_empty_artifact_fails_and_keeps_required_pages_in_all_chapters_mode(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            result = self.run_checker(root, "--all-chapters")
            self.assertEqual(result.returncode, 1)
            self.assertIn("index.html: missing required page", result.stdout)

            self.write_required_pages(root)
            (root / "introduction.html").write_text("", encoding="utf-8")
            result = self.run_checker(root, "--all-chapters")
            self.assertEqual(result.returncode, 1)
            self.assertIn("introduction.html: empty required page", result.stdout)

            (root / "introduction.html").write_text(
                "<!doctype html><html></html>\n", encoding="utf-8"
            )
            (root / "appendices/contributing/documentation_generation.html").unlink()
            result = self.run_checker(root, "--all-chapters")
            self.assertEqual(result.returncode, 1)
            self.assertIn(
                "appendices/contributing/documentation_generation.html: missing required page",
                result.stdout,
            )

    def test_all_chapters_checks_required_and_discovered_pages(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.write_required_pages(root)
            (root / "toc.html").write_text(
                '<a href="extra.html">Extra</a>', encoding="utf-8"
            )
            (root / "extra.html").write_text("<html></html>\n", encoding="utf-8")

            result = self.run_checker(root, "--all-chapters")

            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_inventory_rejects_missing_and_empty_chapter(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.write_required_pages(root)
            (root / "toc.html").write_text(
                '<a href="chapter%20name.html">Chapter</a>', encoding="utf-8"
            )
            missing = self.run_checker(root, "--all-chapters")
            self.assertEqual(missing.returncode, 1)
            self.assertIn("chapter name.html: missing required page", missing.stdout)

            (root / "chapter name.html").touch()
            empty = self.run_checker(root, "--all-chapters")
            self.assertEqual(empty.returncode, 1)
            self.assertIn("chapter name.html: empty required page", empty.stdout)

    def test_word_index_mode_requires_its_input(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            result = self.run_checker(Path(temporary), "--word-index")
            self.assertEqual(result.returncode, 1)
            self.assertIn("missing required page: word_index.html", result.stdout)


if __name__ == "__main__":
    unittest.main()
