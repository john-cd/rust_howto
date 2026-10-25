// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Generate a category page from local TOML metadata and recipe links.

use anyhow::Result;

use crate::fixtures::CATEGORY_METADATA;
use crate::fixtures::create_fixture;

/// Prints the generated category index using the checked-in style of metadata.
pub fn run() -> Result<()> {
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
        let page =
            tool_lib::create_category_page_from_metadata_with_recipe_links(
                &["serde".to_owned(), "serde_json".to_owned()],
                CATEGORY_METADATA,
                &recipes,
            )?;
        assert!(page.contains("serde_json"));
        assert!(page.contains("Serialize JSON"));
        Ok(())
    }
}
