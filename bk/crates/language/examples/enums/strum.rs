#![allow(dead_code)]
// ANCHOR: example
//! `strum` provides derive macros that automatically implement various traits
//! for your enums, making it easy to convert them to and from strings, iterate
//! over their variants, and more.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! strum = { version = "0.27", features = ["derive"] }
//! ```
//!
//! TODO Reviewed ??-??-2026

use strum::Display;
use strum::EnumIter;
use strum::EnumString;
use strum::IntoEnumIterator as _; // Import the trait to use .iter().

// We derive a few `strum` traits in addition to standard `Debug` and
// `PartialEq`:
// - `Display` to easily print the enum variant as a string,
// - `EnumString` to parse a string into an enum variant,
// - `EnumIter` to iterate over all possible variants of the enum.
/// The cardinal directions, with `strum` derive macros.
#[derive(Debug, Display, EnumString, EnumIter, PartialEq)]
enum Direction {
    /// North, displayed as `N`.
    #[strum(serialize = "N")] // Customize the `Display` representation.
    North,
    /// East, displayed as `E`.
    #[strum(serialize = "E")]
    East,
    /// South, displayed as `S`.
    #[strum(serialize = "S")]
    South,
    /// West, displayed as `W`.
    #[strum(serialize = "W")]
    West,
}

/// Displays, parses and iterates over the variants of an enum.
fn main() {
    println!("--- Displaying Enum Variants ---");
    let dir = Direction::North;
    // Using the `Display` trait.
    println!("Current direction: {dir}");

    println!("\n--- Parsing Strings into Enums ---");
    let input_string_south = "South";
    let input_string_invalid = "Up";

    // Using the `EnumString` trait:
    match input_string_south.parse::<Direction>() {
        Ok(direction) => println!(
            "Parsed '{input_string_south}' successfully: {direction:?}.",
        ),
        Err(e) => println!("Failed to parse '{input_string_south}': {e}."),
    }

    match input_string_invalid.parse::<Direction>() {
        Ok(direction) => println!(
            "Parsed '{input_string_invalid}' successfully: {direction:?}.",
        ),
        Err(e) => {
            println!("Failed to parse '{input_string_invalid}': {e}.");
        }
    }

    println!("\n--- Iterating Over Enum Variants ---");
    // Using the `EnumIter` trait:
    println!("All possible directions:");
    for direction in Direction::iter() {
        println!("- {direction:?}");
    }
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
