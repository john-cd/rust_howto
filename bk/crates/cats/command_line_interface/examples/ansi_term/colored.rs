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

    #[test]
    fn colorized_strings_contain_ansi_codes_only_when_color_is_enabled() {
        colored::control::set_override(true);
        let green = "Success".green().bold().to_string();
        assert!(green.starts_with("\u{1b}["), "{green:?}");
        assert!(green.contains("Success"));
        assert!(green.contains("32"), "green is ANSI 32: {green:?}");
        assert!(green.ends_with("\u{1b}[0m"));
        assert!("Error".red().underline().to_string().contains("31"));

        colored::control::set_override(false);
        assert_eq!("Warning".yellow().on_blue().to_string(), "Warning");
        colored::control::unset_override();
    }
}
