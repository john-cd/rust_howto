#![allow(dead_code)]
// ANCHOR: example
use ansi_term::Colour;
use ansi_term::Style;

/// Demonstrates basic usage of the `ansi_term` crate for colored and styled
/// terminal output.
fn main() {
    println!(
        "{}, {} and {}",
        Colour::Yellow.paint("This is colored"),
        Style::new().bold().paint("this is bold"),
        Colour::Yellow.bold().paint("this is bold and colored")
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
