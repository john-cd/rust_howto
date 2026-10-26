# Purpose: Test the recipe-table updater and its title synchronization behavior.
# Usage: From bk/, run the matching just recipe and pass its arguments.
#
# TODO Reviewed ??-??-2026

from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

import fix_recipe_tables


class RecipeTableTests(unittest.TestCase):
    def test_existing_titles_preserve_ampersands_pipes_backslashes_and_regex_chars(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary) / "src" / "chapter"
            directory.mkdir(parents=True)
            chapter = directory / "table.md"
            include = directory / "table.incl.md"
            (directory / "refs.incl.md").write_text(
                "[ex~existing+(v1)?]: table.md#title\n", encoding="utf-8"
            )
            chapter.write_text(
                r"## C:\tmp & API | v1 + (x.y)? {#title}" + "\n", encoding="utf-8"
            )
            include.write_text(
                "| Recipe | Crates | Categories |\n"
                "|--------|--------|------------|\n"
                "| [Stale title][ex~existing+(v1)?] | crate | category |\n",
                encoding="utf-8",
            )

            self.assertTrue(fix_recipe_tables.update_chapter(chapter))
            generated = include.read_text(encoding="utf-8")
            self.assertIn(
                r"| [C:\tmp & API \| v1 + (x.y)?][ex~existing+(v1)?] | crate | category |",
                generated,
            )
            self.assertFalse(fix_recipe_tables.update_chapter(chapter))
            self.assertEqual(generated, include.read_text(encoding="utf-8"))

    def test_three_columns_adds_badges_before_hidden_and_is_idempotent(self) -> None:
        with tempfile.TemporaryDirectory(prefix="book root with spaces ") as temporary:
            directory = Path(temporary) / "src" / "chapter"
            directory.mkdir(parents=True)
            chapter = directory / "tables.md"
            include = directory / "tables.incl.md"
            (directory / "refs.incl.md").write_text(
                "[ex~existing]: tables.md#existing\n[ex~new]: tables.md#new\n",
                encoding="utf-8",
            )
            chapter.write_text(
                "## Existing {#existing}\n"
                "[![serde][c~serde~docs~badge]][c~serde~docs] "
                "[![cat~data][cat~data~badge]][cat~data]\n"
                "## New title & API | usage {#new}\n"
                "[![serde][c~serde~docs~badge]][c~serde~docs] "
                "[![cat~data][cat~data~badge]][cat~data]\n",
                encoding="utf-8",
            )
            include.write_text(
                "| Recipe | Crates | Categories |\n"
                "|--------|--------|------------|\n"
                "| [Old title][ex~existing] |  |  |\n"
                '<div class="hidden">\nprivate refs\n</div>\n',
                encoding="utf-8",
            )

            self.assertTrue(fix_recipe_tables.update_chapter(chapter))
            generated = include.read_text(encoding="utf-8")
            expected = (
                r"| [New title & API \| usage][ex~new] | "
                r"[![serde][c~serde~docs~badge]][c~serde~docs] | "
                r"[![cat~data][cat~data~badge]][cat~data] |"
            )
            self.assertIn(expected, generated)
            self.assertIn("| [Existing][ex~existing] |  |  |", generated)
            self.assertLess(
                generated.index(expected), generated.index('<div class="hidden">')
            )
            self.assertIn("private refs", generated)

            self.assertFalse(fix_recipe_tables.update_chapter(chapter))
            self.assertEqual(generated, include.read_text(encoding="utf-8"))

    def test_two_columns_use_the_declared_category_column_and_match_punctuation_labels(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="book root ") as temporary:
            directory = Path(temporary) / "src" / "category"
            directory.mkdir(parents=True)
            chapter = directory / "chapter with spaces.md"
            include = directory / "chapter with spaces.incl.md"
            (directory / "refs.incl.md").write_text(
                r"[ex~regex+name?(v1)]: <chapter with spaces.md#anchor>" + "\n",
                encoding="utf-8",
            )
            chapter.write_text(
                "{{#include chapter with spaces.incl.md}}\n"
                "## Oil & gas | API {#anchor}\n"
                "[![cat~network][cat~network~badge]][cat~network]\n",
                encoding="utf-8",
            )
            include.write_text(
                "| Recipe | Categories |\n|--------|------------|\n", encoding="utf-8"
            )

            self.assertTrue(fix_recipe_tables.update_chapter(chapter))
            generated = include.read_text(encoding="utf-8")
            self.assertIn(r"[Oil & gas \| API][ex~regex+name?(v1)]", generated)
            self.assertIn("[![cat~network][cat~network~badge]][cat~network]", generated)
            self.assertNotIn("categories | [![cat", generated)

    def test_one_column_and_missing_anchor_do_not_require_badges(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary) / "src" / "chapter"
            directory.mkdir(parents=True)
            chapter = directory / "one.md"
            include = directory / "one.incl.md"
            (directory / "refs.incl.md").write_text(
                "[ex~lost-anchor]: one.md#missing\n", encoding="utf-8"
            )
            chapter.write_text("{{#include one.incl.md}}\n", encoding="utf-8")
            include.write_text("| Recipe |\n|--------|\n", encoding="utf-8")

            self.assertTrue(fix_recipe_tables.update_chapter(chapter))
            self.assertIn(
                "| [lost anchor][ex~lost-anchor] |", include.read_text(encoding="utf-8")
            )

    def test_missing_include_is_created_only_when_wrapper_includes_it(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary) / "src" / "chapter"
            directory.mkdir(parents=True)
            chapter = directory / "missing with spaces.md"
            include = directory / "missing with spaces.incl.md"
            (directory / "refs.incl.md").write_text(
                "[ex~anchor]: <missing with spaces.md#anchor>\n", encoding="utf-8"
            )
            chapter.write_text(
                "{{#include missing with spaces.incl.md}}\n## Title {#anchor}\n",
                encoding="utf-8",
            )

            self.assertTrue(fix_recipe_tables.update_chapter(chapter))
            generated = include.read_text(encoding="utf-8")
            self.assertIn("| Recipe | Crates | Categories |", generated)
            self.assertIn("| [Title][ex~anchor] |  |  |", generated)

            include.unlink()
            chapter.write_text("## Title {#anchor}\n", encoding="utf-8")
            self.assertFalse(fix_recipe_tables.update_chapter(chapter))
            self.assertFalse(include.exists())


if __name__ == "__main__":
    unittest.main()
