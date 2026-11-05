//! Lists the recipes that the published book links to each crate.
//!
//! Run with `cargo run -p tool_lib --example published_recipes`.

use std::path::Path;

use tool_lib::collect_published_recipe_links;

/// Parses the book's example index and prints up to five crates with their recipes.
fn main() -> anyhow::Result<()> {
    let book_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../bk");
    let links = collect_published_recipe_links(book_root)?;
    for (crate_name, recipes) in links.iter().take(5) {
        println!("{crate_name}:");
        for recipe in recipes {
            println!("  - {} ({})", recipe.title, recipe.reference);
        }
    }
    Ok(())
}
