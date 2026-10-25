// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Generate an alphabetical crate section with published recipe links.

use anyhow::Result;

use crate::fixtures::create_fixture;

/// Prints one alphabetical index section from the local recipe index.
pub fn run() -> Result<()> {
    let fixture = create_fixture()?;
    let book_root = fixture.path().join("book");
    let recipes = tool_lib::collect_published_recipe_links(book_root)?;
    let page = tool_lib::create_alphabetical_crate_page_section(
        "S",
        vec!["serde", "serde_json"],
        &recipes,
    )?;
    println!("Alphabetical section:\n{page}");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::fixtures::create_fixture;

    /// Verifies that alphabetical output includes recipe links for each crate.
    #[test]
    fn alphabetical_page_includes_published_recipe_links() -> Result<()> {
        let fixture = create_fixture()?;
        let book_root = fixture.path().join("book");
        let recipes = tool_lib::collect_published_recipe_links(book_root)?;
        let page = tool_lib::create_alphabetical_crate_page_section(
            "S",
            vec!["serde", "serde_json"],
            &recipes,
        )?;
        assert!(page.contains("[Serialize JSON][ex~serialization~serde]"));
        Ok(())
    }
}
