// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates basic usage of the maintained `nu-ansi-term` crate.

use nu_ansi_term::Color;

/// Demonstrates basic usage of the maintained `nu-ansi-term` crate.
fn main() {
    println!(
        "This is {} in color, {} in color and {} in color",
        Color::Red.paint("red"),
        Color::Blue.paint("blue"),
        Color::Green.paint("green")
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the ansi term basic example through its entry point.
    #[test]
    fn test_ansi_term_basic_example_entry_point() {
        main();
    }

    #[test]
    fn painted_text_is_wrapped_in_ansi_color_codes() {
        assert_eq!(
            Color::Red.paint("red").to_string(),
            "\u{1b}[31mred\u{1b}[0m"
        );
        assert_eq!(
            Color::Blue.paint("blue").to_string(),
            "\u{1b}[34mblue\u{1b}[0m"
        );
        assert_eq!(
            Color::Green.paint("green").to_string(),
            "\u{1b}[32mgreen\u{1b}[0m"
        );
    }
}
