// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates text styling using the `colored` crate.

use colored::Colorize as _;

/// Prints text with colors and styles.
fn main() {
    // Print styled text using foreground color and bold formatting.
    println!("{}", "Success".green().bold());

    // Print styled text with a yellow foreground on a blue background.
    println!("{}", "Warning".yellow().on_blue());

    // Print red, underlined text for emphasis.
    println!("{}", "Error".red().underline());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the colored example through its entry point.
    #[test]
    fn test_colored_example_entry_point() {
        main();
    }
}
