#![allow(dead_code)]

// TODO Reviewed ??-??-2026
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
