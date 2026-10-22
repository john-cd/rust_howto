//! Demonstrates basic usage of the `nu-ansi-term` crate for colored and styled
//! terminal output.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
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

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
