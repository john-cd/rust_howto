#![allow(dead_code)]
// ANCHOR: example
use ansi_term::Colour;

/// Demonstrates basic usage of the `ansi_term` crate for colored output.
fn main() {
    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
