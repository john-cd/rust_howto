#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Basic example of using `nu-ansi-term` to print bold text.

use nu_ansi_term::Style;

/// Basic example of using `nu-ansi-term` to print bold text.
fn main() {
    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the ansi term basic1 example through its entry point.
    #[test]
    fn test_ansi_term_basic1_example_entry_point() {
        main();
    }
}
