// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Collect published recipe links from the local book index.

use anyhow::Result;

use crate::fixtures::create_fixture;

/// Prints the crate-to-recipe links found in the fixture book.
pub fn run() -> Result<()> {
    let fixture = create_fixture()?;
    let book_root = fixture.path().join("book");
    let recipes = tool_lib::collect_published_recipe_links(book_root)?;
    println!("Published recipe links: {recipes:#?}");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use anyhow::Context as _;
    use anyhow::Result;

    use crate::fixtures::create_fixture;

    /// Verifies that published recipes are collected from local includes.
    #[test]
    fn published_recipe_collection_uses_local_includes() -> Result<()> {
        let fixture = create_fixture()?;
        let book_root = fixture.path().join("book");
        let recipes = tool_lib::collect_published_recipe_links(book_root)?;
        let serde_recipes = recipes
            .get("serde")
            .context("serde recipe link was collected")?;
        assert_eq!(
            serde_recipes
                .first()
                .map(|recipe| recipe.reference.as_str()),
            Some("ex~serialization~serde")
        );
        Ok(())
    }
}
