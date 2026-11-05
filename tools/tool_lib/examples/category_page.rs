//! Formats a crates-by-category page from an inline metadata snapshot.
//!
//! Run with `cargo run -p tool_lib --example category_page`.
//!
//! TODO Reviewed ??-??-2026.

use tool_lib::create_category_page_from_metadata;

/// Metadata snapshot in the format of `bk/master/crate_categories.toml`.
const METADATA: &str = r#"
[[categories]]
category = "Mathematics"
slug = "mathematics"
description = "Crates with a mathematical aspect."
crates = ["nalgebra", "rug"]
"#;

/// Prints the page; `serde` is not in the snapshot, so it is listed as uncategorized.
fn main() -> anyhow::Result<()> {
    let crate_names = ["nalgebra", "rug", "serde"].map(String::from);
    print!(
        "{}",
        create_category_page_from_metadata(&crate_names, METADATA)?
    );
    Ok(())
}
