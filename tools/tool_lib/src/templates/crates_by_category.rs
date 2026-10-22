//! The page of crates by category.
//!
//! TODO Reviewed ??-??-2026

use std::collections::BTreeSet;
use std::collections::HashSet;

use anyhow::Context as _;
use anyhow::Result;
use serde::Deserialize;

use super::RecipeLinksByCrate;

/// Slug of the category of the crates that have no category.
const UNCATEGORIZED_SLUG: &str = "uncategorized";

/// Description of the category of the crates that have no category.
const UNCATEGORIZED_DESCRIPTION: &str =
    "No crates.io category is recorded for these crates in the book's current inventory.";

/// Header of the "crates by category" table.
const CATEGORY_PAGE_HEADER: &str =
    "# Crates by Category\n\n| Category | Description | Crates |\n| --- | --- | --- |\n";

/// The footer of the generated page: a link reference and the includes.
const CATEGORY_PAGE_FOOTER: &str = "\n[All Valid Categories][crates.io~category_slugs]\u{2197}.\n\n{{#include refs.incl.md}}\n{{#include ../refs/link-refs.md}}\n\n<div class=\"hidden\">\n</div>\n";

/// The categories file.
#[derive(Deserialize)]
struct CategoryCatalog {
    /// The categories.
    categories: Vec<CategoryRecord>,
}

/// A category and its crates.
#[derive(Deserialize)]
struct CategoryRecord {
    /// Name of the category.
    category: String,
    /// Slug of the category on `crates.io`.
    slug: String,
    /// Description of the category.
    description: String,
    /// Names of the crates in the category.
    #[serde(default)]
    crates: Vec<String>,
}

/// `Create_category_and_crates`
/// Returns one line in the "crates organized by category" table:
/// | <category badge> | <description> | <crate badge> <crate badge>... |
///
/// category: category name e.g. Encoding
/// slug: category slug used by the crates.io website e.g. encoding
/// description: description of the category
/// `crate_names`: list of the names of the crates to include in the category (per
/// lib.rs)
pub fn create_category_and_crates(
    category: &str,
    slug: &str,
    description: &str,
    crate_names: Vec<&str>,
) -> Result<String> {
    create_category_row(
        category,
        slug,
        description,
        crate_names,
        &RecipeLinksByCrate::new(),
    )
}

/// Returns one row of the table, with links to the recipes that use each crate.
fn create_category_row(
    category: &str,
    slug: &str,
    description: &str,
    crate_names: Vec<&str>,
    recipe_links: &RecipeLinksByCrate,
) -> Result<String> {
    let category = category.trim();
    let slug = slug.trim();
    let description = description.trim();
    let mut row = if slug == UNCATEGORIZED_SLUG {
        String::from("| Uncategorized | ")
    } else {
        format!("| [![cat~{slug}][cat~{slug}~badge]][cat~{slug}]{{{{hi:{category}}}}} | ")
    };
    row.push_str(description);
    row.push_str(" | ");
    let mut previous_crate_had_recipes = false;
    for name in crate_names {
        let links = recipe_links
            .get(name)
            .map(Vec::as_slice)
            .unwrap_or_default();
        if !row.ends_with(" | ") {
            row.push_str(if previous_crate_had_recipes || !links.is_empty() {
                "<br>"
            } else {
                " "
            });
        }
        row.push_str("[![");
        row.push_str(name);
        row.push_str("][c~");
        row.push_str(name);
        row.push_str("~docs~badge]][c~");
        row.push_str(name);
        row.push_str("~docs]{{hi:");
        row.push_str(name);
        row.push_str("}}");
        for link in links {
            row.push_str("<br>&nbsp;&nbsp;\u{21b3} [");
            row.push_str(&link.title);
            row.push_str("][");
            row.push_str(&link.reference);
            row.push(']');
        }
        previous_crate_had_recipes = !links.is_empty();
    }
    row.push('|');
    Ok(row)
}

/// Render the external crates.io category index from a checked-in metadata snapshot.
///
/// Crates not listed in the snapshot are added to the uncategorized row. Entries in
/// the snapshot that are absent from the current inventory are ignored, so crate
/// renames do not leave stale badges behind.
pub fn create_category_page_from_metadata(
    crate_names: &[String],
    metadata: &str,
) -> Result<String> {
    create_category_page_from_metadata_with_recipe_links(
        crate_names,
        metadata,
        &RecipeLinksByCrate::new(),
    )
}

/// Render the external crates.io category index and link each crate to its published recipes.
pub fn create_category_page_from_metadata_with_recipe_links(
    crate_names: &[String],
    metadata: &str,
    recipe_links: &RecipeLinksByCrate,
) -> Result<String> {
    let catalog: CategoryCatalog =
        toml::from_str(metadata).context("parse crate category metadata")?;
    let inventory: BTreeSet<String> = crate_names
        .iter()
        .map(|name| name.trim())
        .filter(|name| !name.is_empty() && *name != "std")
        .map(str::to_owned)
        .chain(
            recipe_links
                .keys()
                .filter(|name| name.as_str() != "std")
                .cloned(),
        )
        .collect();

    let mut slugs = HashSet::new();
    let mut groups: Vec<(String, String, String, BTreeSet<String>)> = Vec::new();
    let mut uncategorized_index = None;
    let mut categorized = BTreeSet::new();

    for record in catalog.categories {
        let category = record.category.trim().to_owned();
        let slug = record.slug.trim().to_owned();
        let description = record.description.trim().to_owned();
        anyhow::ensure!(!category.is_empty(), "category name cannot be empty");
        anyhow::ensure!(!slug.is_empty(), "category slug cannot be empty");
        anyhow::ensure!(
            slugs.insert(slug.clone()),
            "duplicate category slug in crate category metadata: {slug}"
        );

        let names: BTreeSet<String> = record
            .crates
            .iter()
            .map(|name| name.trim())
            .filter(|name| inventory.contains(*name))
            .map(str::to_owned)
            .collect();
        categorized.extend(names.iter().cloned());

        if slug == UNCATEGORIZED_SLUG {
            uncategorized_index = Some(groups.len());
        }
        groups.push((category, slug, description, names));
    }

    let uncategorized: BTreeSet<String> = inventory.difference(&categorized).cloned().collect();
    if !uncategorized.is_empty() {
        let index = if let Some(index) = uncategorized_index {
            index
        } else {
            anyhow::ensure!(
                slugs.insert(UNCATEGORIZED_SLUG.to_owned()),
                "crate category metadata reserves the uncategorized slug"
            );
            let index = groups.len();
            groups.push((
                "Uncategorized".to_owned(),
                UNCATEGORIZED_SLUG.to_owned(),
                UNCATEGORIZED_DESCRIPTION.to_owned(),
                BTreeSet::new(),
            ));
            index
        };
        groups[index].3.extend(uncategorized);
    }

    let mut markdown = String::from(CATEGORY_PAGE_HEADER);
    for (category, slug, description, names) in groups {
        if names.is_empty() {
            continue;
        }
        let crate_names = names.iter().map(String::as_str).collect();
        markdown.push_str(&create_category_row(
            &category,
            &slug,
            &description,
            crate_names,
            recipe_links,
        )?);
        markdown.push('\n');
    }
    markdown.push_str(CATEGORY_PAGE_FOOTER);
    Ok(markdown)
}

#[cfg(test)]
mod tests {
    use super::super::RecipeLink;
    use super::*;

    #[test]
    fn test_create_category_and_crates() {
        let category = " Encoding ";
        let slug = " encoding ";
        let description = " Description of the category ";
        let crate_names = vec!["base64", "hex"];

        let result = create_category_and_crates(category, slug, description, crate_names)
            .expect("input should parse");

        // Check for the category badge and name
        assert!(result.contains("[![cat~encoding][cat~encoding~badge]][cat~encoding]"));
        assert!(result.contains("{{hi:Encoding}}"));

        // Check for the description
        assert!(result.contains("Description of the category"));

        // Check for the crate badges
        assert!(result.contains("[![base64][c~base64~docs~badge]][c~base64~docs]"));
        assert!(result.contains("{{hi:base64}}"));
        assert!(result.contains("[![hex][c~hex~docs~badge]][c~hex~docs]"));
        assert!(result.contains("{{hi:hex}}"));
    }

    #[test]
    fn test_create_category_and_crates_empty() {
        let category = "Empty";
        let slug = "empty";
        let description = "Empty category";
        let crate_names = vec![];

        let result = create_category_and_crates(category, slug, description, crate_names)
            .expect("input should parse");

        assert!(result.contains("[![cat~empty][cat~empty~badge]][cat~empty]"));
        assert!(result.contains("{{hi:Empty}}"));
        assert!(result.contains("Empty category"));

        // Ensure it doesn't contain crate badges (it should end with a '|')
        assert!(result.ends_with('|'));
    }

    #[test]
    fn category_row_preserves_existing_markdown_entities() {
        let row = create_category_and_crates(
            "Game engines",
            "game-engines",
            "A &quot;one-stop-shop&quot; category &amp; description.",
            vec!["glam"],
        )
        .expect("input should parse");

        assert!(row.contains("A &quot;one-stop-shop&quot; category &amp; description."));
        assert!(!row.contains("&amp;quot;"));
        assert!(!row.contains("&amp;amp;"));
    }

    #[test]
    fn category_page_from_metadata_handles_duplicates_renames_and_missing_categories() {
        let inventory = ["shared", "shared", "new-name", "std"].map(str::to_owned);
        let metadata = r#"
            [[categories]]
            category = "Data structures"
            slug = "data-structures"
            description = "Structures and collections."
            crates = ["shared", "shared", "old-name"]

            [[categories]]
            category = "Uncategorized"
            slug = "uncategorized"
            description = "No metadata is recorded."
            crates = ["old-name", "removed"]
        "#;

        let page =
            create_category_page_from_metadata(&inventory, metadata).expect("input should parse");

        assert_eq!(page.matches("[![shared][c~shared~docs~badge]").count(), 1);
        assert_eq!(
            page.matches("[![new-name][c~new-name~docs~badge]").count(),
            1
        );
        assert!(!page.contains("[![old-name][c~old-name~docs~badge]"));
        assert!(!page.contains("[![removed][c~removed~docs~badge]"));
        assert!(!page.contains("[![std][c~std~docs~badge]"));
        assert!(page.contains("| Uncategorized | No metadata is recorded. |"));
    }

    #[test]
    fn category_page_from_metadata_nests_recipe_links_under_their_crates() {
        let inventory = ["glam", "vello"].map(str::to_owned);
        let metadata = r#"
            [[categories]]
            category = "Graphics"
            slug = "graphics"
            description = "Graphics crates."
            crates = ["vello", "glam"]
        "#;
        let recipe_links = RecipeLinksByCrate::from([(
            "glam".to_owned(),
            vec![RecipeLink {
                title: "Draw with glam".to_owned(),
                reference: "ex~graphics~glam".to_owned(),
            }],
        )]);

        let page = create_category_page_from_metadata_with_recipe_links(
            &inventory,
            metadata,
            &recipe_links,
        )
        .expect("input should parse");

        assert!(page.contains(
            "[![glam][c~glam~docs~badge]][c~glam~docs]{{hi:glam}}<br>&nbsp;&nbsp;\u{21b3} [Draw with glam][ex~graphics~glam]<br>[![vello]"
        ));
    }

    #[test]
    fn category_page_from_metadata_includes_crates_found_only_in_recipes() {
        let inventory = ["known".to_owned(), "std".to_owned()];
        let metadata = r#"
            [[categories]]
            category = "Data structures"
            slug = "data-structures"
            description = "Data structures crates."
            crates = ["known"]
        "#;
        let recipe_links = RecipeLinksByCrate::from([(
            "recipe-only".to_owned(),
            vec![RecipeLink {
                title: "Use recipe-only".to_owned(),
                reference: "ex~examples~recipe-only".to_owned(),
            }],
        )]);

        let page = create_category_page_from_metadata_with_recipe_links(
            &inventory,
            metadata,
            &recipe_links,
        )
        .expect("input should parse");

        assert!(page.contains(
            "| Uncategorized | No crates.io category is recorded for these crates in the book's current inventory. | [![recipe-only][c~recipe-only~docs~badge]][c~recipe-only~docs]{{hi:recipe-only}}<br>&nbsp;&nbsp;\u{21b3} [Use recipe-only][ex~examples~recipe-only]|"
        ));
    }

    #[test]
    fn category_page_from_metadata_is_deterministic() {
        let metadata = r#"
            [[categories]]
            category = "A category"
            slug = "a-category"
            description = "Example category."
            crates = ["beta", "alpha"]
        "#;
        let first = vec!["beta".to_owned(), "alpha".to_owned(), "alpha".to_owned()];
        let second = vec!["alpha".to_owned(), "beta".to_owned()];

        assert_eq!(
            create_category_page_from_metadata(&first, metadata).expect("input should parse"),
            create_category_page_from_metadata(&second, metadata).expect("input should parse")
        );
    }

    #[test]
    fn category_page_from_metadata_rejects_duplicate_slugs() {
        let metadata = r#"
            [[categories]]
            category = "First"
            slug = "same"
            description = "First category."
            crates = ["crate-a"]

            [[categories]]
            category = "Second"
            slug = "same"
            description = "Second category."
            crates = ["crate-b"]
        "#;

        let error = create_category_page_from_metadata(&["crate-a".to_owned()], metadata)
            .expect_err("input should be rejected");

        assert!(error.to_string().contains("duplicate category slug"));
    }
}
