// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Generate a category page from local TOML metadata and recipe links.
//!
//! This book lists crates in generated index pages. A script reads local TOML
//! metadata for each category and writes the Markdown page, so the pages stay
//! consistent and can be regenerated.

use anyhow::Result;

use crate::fixtures::CATEGORY_METADATA;
use crate::fixtures::create_fixture;

/// Prints the generated category index using the checked-in style of metadata.
///
/// # Errors
/// Returns an error if a step of the example fails.
pub fn example_body() -> Result<()> {
    let fixture = create_fixture()?;
    let book_root = fixture.path().join("book");
    let recipes = tool_lib::collect_published_recipe_links(book_root)?;
    let crate_names = ["serde".to_owned(), "serde_json".to_owned()];
    let page = tool_lib::create_category_page_from_metadata_with_recipe_links(
        &crate_names,
        CATEGORY_METADATA,
        &recipes,
    )?;
    println!("Offline category page:\n{page}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the example function and returns its result.
///
/// # Errors
/// Returns an error if the example operation fails.
pub fn run() -> Result<()> {
    example_body()
}

/// Verifies that local metadata renders crate and recipe links.
#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::fixtures::CATEGORY_METADATA;
    use crate::fixtures::create_fixture;

    /// Verifies that local metadata renders crate and recipe links.
    #[test]
    fn category_page_uses_local_metadata_and_recipe_links() -> Result<()> {
        let fixture = create_fixture()?;
        let book_root = fixture.path().join("book");
        let recipes = tool_lib::collect_published_recipe_links(book_root)?;
        let page = tool_lib::create_category_page_from_metadata_with_recipe_links(
            &["serde".to_owned(), "serde_json".to_owned()],
            CATEGORY_METADATA,
            &recipes,
        )?;
        assert!(page.contains("serde_json"));
        assert!(page.contains("Serialize JSON"));
        Ok(())
    }
}
