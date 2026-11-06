// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Constructing category badges using `tool_lib`.
//!
//! This example demonstrates how to use `tool_lib::create_category_badge` to
//! construct Markdown category badges and index markers for crates.io categories.

use anyhow::Result;

/// Constructs category badges for sample categories and prints them.
fn run_example() -> Result<()> {
    // Generate a badge for the 'command-line-interface' category.
    let cli_badge =
        tool_lib::create_category_badge("Command line interface", "command-line-interface")?;
    println!("Generated CLI category badge:\n{cli_badge}\n");

    // Generate a badge for the 'encoding' category.
    let encoding_badge = tool_lib::create_category_badge("Encoding", "encoding")?;
    println!("Generated Encoding category badge:\n{encoding_badge}");

    Ok(())
}

/// Runs the category badge example binary.
fn main() -> Result<()> {
    run_example()
}
// ANCHOR_END: example

/// Runs the category badge example for programmatic execution.
pub fn run() -> Result<()> {
    run_example()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the category badge example runs successfully.
    #[test]
    fn test_category_badge_example_entry_point() {
        run().expect("category badge example should run without errors");
    }

    /// Verifies that the generated category badge contains expected markers.
    #[test]
    fn test_category_badge_output_format() {
        let badge = tool_lib::create_category_badge("Networking", "network-programming")
            .expect("category badge rendering should succeed");
        assert!(badge.contains("[![cat~network-programming][cat~network-programming~badge]]"));
        assert!(badge.contains("[cat~network-programming]"));
        assert!(badge.contains("{{hi:Networking}}"));
    }
}
