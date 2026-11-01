//! Generate a category badge with the shared `templ` renderer.
//!
//! A badge is a small label, here showing a category, rendered from a template.
//! Rendering through one shared function keeps every badge in the book
//! identical.

// ANCHOR: example
use anyhow::Result;
use tool_lib::create_category_badge;

/// Renders a badge for a known category without requesting registry metadata.
fn render_category_badge() -> Result<String> {
    create_category_badge("Encoding", "encoding")
}

/// Prints the generated category badge.
fn main() -> Result<()> {
    println!("{}", render_category_badge()?);
    Ok(())
}
// ANCHOR_END: example

/// Runs the category badge example.
pub fn run() -> Result<()> {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the category badge output and runs the example entry point.
    #[test]
    fn renders_a_known_category_badge_offline() -> Result<()> {
        assert_eq!(
            render_category_badge()?,
            "[![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} "
        );
        main()
    }
}
