//! Renders category and Rust By Example badges without any network access.
//!
//! Run with `cargo run -p tool_lib --example badges`.
//!
//! TODO Reviewed ??-??-2026.

use tool_lib::create_category_badge;
use tool_lib::create_rbe_badge;

/// Prints a category badge and a Rust By Example badge.
fn main() -> anyhow::Result<()> {
    println!("{}", create_category_badge("Encoding", "encoding")?);
    println!("{}", create_rbe_badge("attributes")?);
    Ok(())
}
