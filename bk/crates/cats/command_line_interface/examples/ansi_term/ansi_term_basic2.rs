// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates basic usage of the `nu-ansi-term` crate for colored and styled
//! terminal output.

use nu_ansi_term::Color;
use nu_ansi_term::Style;

/// Demonstrates basic usage of the `nu-ansi-term` crate for colored and styled
/// terminal output.
fn main() {
    println!(
        "{}, {} and {}",
        Color::Yellow.paint("This is colored"),
        Style::new().bold().paint("this is bold"),
        Color::Yellow.bold().paint("this is bold and colored")
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the ansi term basic2 example through its entry point.
    #[test]
    fn test_ansi_term_basic2_example_entry_point() {
        main();
    }
}
