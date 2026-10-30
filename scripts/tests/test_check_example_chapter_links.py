#!/usr/bin/env python3
"""Regression tests for the unfenced-include audit.

Run: python3 scripts/tests/test_check_example_chapter_links.py
"""

import sys
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))

from check_example_chapter_links import unfenced_includes  # noqa: E402

FIXTURE = (HERE / "fixtures" / "generic_snippet.rs").read_text()
INCLUDE = "{{#include fixtures/generic_snippet.rs:example}}"


class UnfencedIncludes(unittest.TestCase):
    def test_fixture_has_markup_hazards(self):
        # The fixture is the kind of source that breaks when pasted as prose.
        for needle in ("HashMap<String, Vec<Option<T>>>", "#[derive(", "<b>", "//"):
            self.assertIn(needle, FIXTURE)

    def test_bare_include_is_reported(self):
        self.assertEqual(unfenced_includes(f"Text\n\n{INCLUDE}\n"), [3])

    def test_fenced_include_is_accepted(self):
        for tag in ("rust", "rust,editable", "rust,noplayground"):
            self.assertEqual(unfenced_includes(f"```{tag}\n{INCLUDE}\n```\n"), [])

    def test_include_after_a_closed_fence_is_reported(self):
        text = f"```rust\n{INCLUDE}\n```\n{INCLUDE}\n"
        self.assertEqual(unfenced_includes(text), [4])

    def test_longer_fence_is_not_closed_by_shorter_one(self):
        text = f"````md\n```\n{INCLUDE}\n```\n````\n"
        self.assertEqual(unfenced_includes(text), [])

    def test_markdown_includes_are_ignored(self):
        self.assertEqual(unfenced_includes("{{#include page.incl.md}}\n"), [])


if __name__ == "__main__":
    unittest.main()
