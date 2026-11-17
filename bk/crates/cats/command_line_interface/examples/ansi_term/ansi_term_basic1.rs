// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Basic example of using `nu-ansi-term` to print bold text.
//!
//! ANSI escape sequences are special characters that terminals interpret as
//! styling instead of text. `nu-ansi-term` generates them, so you can print
//! bold, coloured text.

use nu_ansi_term::Style;

/// Basic example of using `nu-ansi-term` to print bold text.
fn main() {
    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );
}
// ANCHOR_END: example

/// Basic example of using `nu-ansi-term` to print bold text.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the ansi term basic1 example through its entry point.
    #[test]
    fn test_ansi_term_basic1_example_entry_point() {
        main();
    }

    /// Verifies that the bold style wraps text in the bold code.
    #[test]
    fn bold_style_wraps_text_in_the_bold_code() {
        assert_eq!(
            Style::new().bold().paint("This is Bold").to_string(),
            "\u{1b}[1mThis is Bold\u{1b}[0m"
        );
    }

    /// Leaves the text unchanged when processing the plain style.
    #[test]
    fn plain_style_leaves_text_unchanged() {
        assert_eq!(Style::new().paint("plain").to_string(), "plain");
    }
}
