"""Behavior tests for the `{{#badges}}` migration script."""

import unittest

from migrate_badge_sets import convert, migrate


CRATE_BADGE_BLOCK = "\n".join(
    (
        "[![num][c~num~docs~badge]][c~num~docs]{{hi:num}}",
        "[![num~crates.io][c~num~crates.io~badge]][c~num~crates.io]",
        "[![num~repo][c~num~repo~badge]][c~num~repo]",
        "[![num~lib.rs][c~num~lib.rs~badge]][c~num~lib.rs]",
        "[![cat~science][cat~science~badge]][cat~science]",
        "[![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}",
    )
)


class BadgeSetMigrationTests(unittest.TestCase):
    """Verify the migration preserves supported badge and index semantics."""

    def test_convert_omits_missing_category_title_and_keeps_explicit_title(
        self,
    ) -> None:
        """Converts untitled categories with `slug=` and keeps titled categories."""
        self.assertEqual(
            convert(CRATE_BADGE_BLOCK),
            "{{#badges num science= algorithms=Algorithms}}",
        )

    def test_migrate_is_idempotent_and_skips_mixed_and_table_badges(self) -> None:
        """Migrates a plain block once and leaves mixed and table rows unchanged."""
        mixed_block = (
            CRATE_BADGE_BLOCK + " [![num~website][c~num~website~badge]][c~num~website]"
        )
        table_row = (
            "| Crate | [![num][c~num~docs~badge]][c~num~docs] "
            "[![num~lib.rs][c~num~lib.rs~badge]][c~num~lib.rs] |"
        )
        source = f"# Heading\n\n{CRATE_BADGE_BLOCK}\n\n{mixed_block}\n\n{table_row}\n"

        migrated, count = migrate(source)
        expected = (
            "# Heading\n\n{{#badges num science= algorithms=Algorithms}}\n\n"
            f"{mixed_block}\n\n{table_row}\n"
        )

        self.assertEqual((migrated, count), (expected, 1))
        self.assertEqual(migrate(migrated), (expected, 0))


if __name__ == "__main__":
    unittest.main()
