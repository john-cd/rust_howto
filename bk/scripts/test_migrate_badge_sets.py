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
OTHER_CRATE_BADGE_BLOCK = "\n".join(
    (
        "[![serde][c~serde~docs~badge]][c~serde~docs]{{hi:serde}}",
        "[![serde~crates.io][c~serde~crates.io~badge]][c~serde~crates.io]",
        "[![serde~repo][c~serde~repo~badge]][c~serde~repo]",
        "[![serde~lib.rs][c~serde~lib.rs~badge]][c~serde~lib.rs]",
    )
)
UNINDEXED_CRATE_BADGE_BLOCK = " ".join(
    (
        "[![unstyled][c~unstyled~docs~badge]][c~unstyled~docs]",
        "[![unstyled~crates.io][c~unstyled~crates.io~badge]][c~unstyled~crates.io]",
        "[![unstyled~repo][c~unstyled~repo~badge]][c~unstyled~repo]",
        "[![unstyled~lib.rs][c~unstyled~lib.rs~badge]][c~unstyled~lib.rs]",
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
            "{{#badges num}} {{#categories science=,algorithms=Algorithms}}",
        )

    def test_convert_keeps_website_badge_and_converts_multiple_crates(self) -> None:
        """Converts multiple crate sets while retaining a website badge verbatim."""
        website = "[![num~website][c~num~website~badge]][c~num~website]"
        source = " ".join((website, CRATE_BADGE_BLOCK, OTHER_CRATE_BADGE_BLOCK))

        self.assertEqual(
            convert(source),
            " ".join(
                (
                    website,
                    "{{#badges num}} {{#categories science=,algorithms=Algorithms}}",
                    "{{#badges serde}}",
                )
            ),
        )

    def test_convert_preserves_index_term_order_around_categories(self) -> None:
        """Keeps standalone index terms before and after category terms in order."""
        source = " ".join(
            (
                "[![num][c~num~docs~badge]][c~num~docs]{{hi:num}}",
                "[![num~crates.io][c~num~crates.io~badge]][c~num~crates.io]",
                "[![num~repo][c~num~repo~badge]][c~num~repo]",
                "[![num~lib.rs][c~num~lib.rs~badge]][c~num~lib.rs]",
                "{{hi:before}}",
                "[![cat~science][cat~science~badge]][cat~science]",
                "[![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}",
                "{{hi:after}}",
            )
        )

        self.assertEqual(
            convert(source),
            "{{#badges num}} {{hi:before}} "
            "{{#categories science=,algorithms=Algorithms}} {{hi:after}}",
        )

    def test_convert_keeps_unindexed_groups_and_converts_complete_groups(self) -> None:
        """Converts complete groups beside groups whose index terms cannot be preserved."""
        self.assertEqual(
            convert(f"{UNINDEXED_CRATE_BADGE_BLOCK} {OTHER_CRATE_BADGE_BLOCK}"),
            f"{UNINDEXED_CRATE_BADGE_BLOCK} " + "{{#badges serde}}",
        )
        self.assertEqual(
            convert(f"{OTHER_CRATE_BADGE_BLOCK} {UNINDEXED_CRATE_BADGE_BLOCK}"),
            "{{#badges serde}} " + UNINDEXED_CRATE_BADGE_BLOCK,
        )

    def test_migrate_is_idempotent_and_preserves_unsupported_badges(self) -> None:
        """Migrates complete groups once and keeps unsupported badge groups intact."""
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
            "# Heading\n\n{{#badges num}} {{#categories science=,algorithms=Algorithms}}\n\n"
            "{{#badges num}} {{#categories science=,algorithms=Algorithms}} "
            "[![num~website][c~num~website~badge]][c~num~website]\n\n"
            f"{table_row}\n"
        )

        self.assertEqual((migrated, count), (expected, 2))
        self.assertEqual(migrate(migrated), (expected, 0))

    def test_migrate_converts_complete_badge_sets_inside_table_cells(self) -> None:
        """Converts a complete badge set in one cell and leaves split sets alone."""
        table_badges = " ".join(CRATE_BADGE_BLOCK.splitlines())
        split_row = (
            "| Split set | [![num][c~num~docs~badge]][c~num~docs] "
            "[![num~lib.rs][c~num~lib.rs~badge]][c~num~lib.rs] |"
        )
        source = (
            "| Crate | Badges | Notes |\n| --- | --- | --- |\n"
            f"| num | {table_badges} | keep this column |\n{split_row}\n"
        )
        expected = (
            "| Crate | Badges | Notes |\n| --- | --- | --- |\n"
            "| num | {{#badges num}} {{#categories science=,algorithms=Algorithms}} | "
            "keep this column |\n"
            f"{split_row}\n"
        )

        migrated, count = migrate(source)

        self.assertEqual((migrated, count), (expected, 1))
        self.assertEqual(migrate(migrated), (expected, 0))


if __name__ == "__main__":
    unittest.main()
