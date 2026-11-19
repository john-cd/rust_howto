# TODO Reviewed ??-??-2026.

"""Exercise the import checks used by the rendered Playground audit."""

import unittest

from check_rendered_entry_links import unavailable_playground_imports


class PlaygroundImportTests(unittest.TestCase):
    """Check external packages and local modules in runnable blocks."""

    def test_keeps_available_imports_and_local_modules(self) -> None:
        """A supported package and a same-named local module remain usable."""
        code = "use serde::Deserialize;\nmod postage;\nuse postage::Message;"
        self.assertEqual(
            unavailable_playground_imports(code, {"serde", "postage"}, {"serde"}),
            set(),
        )

    def test_finds_unavailable_modern_imports_and_aliases(self) -> None:
        """Modern imports and renamed crates still require an available package."""
        for snippet in (
            "use postage::broadcast;",
            "use postage as channels;",
            "extern crate postage as channels;",
        ):
            with self.subTest(snippet=snippet):
                self.assertEqual(
                    unavailable_playground_imports(snippet, {"postage"}, set()),
                    {"postage"},
                )


if __name__ == "__main__":
    unittest.main()
