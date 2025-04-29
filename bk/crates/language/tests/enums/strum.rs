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

use strum::Display;
use strum::EnumIter;
use strum::EnumString;
use strum::IntoEnumIterator; // Import the trait to use .iter().

// We derive a few `strum` traits in addition to standard `Debug` and `PartialEq`:
// - `Display` to easily print the enum variant as a string,
// - `EnumString` to parse a string into an enum variant,
// - `EnumIter` to iterate over all possible variants of the enum.
#[derive(Debug, Display, EnumString, EnumIter, PartialEq)]
enum Direction {
    #[strum(serialize = "N")] // Customize the `Display` representation.
    North,
    #[strum(serialize = "E")]
    East,
    #[strum(serialize = "S")]
    South,
    #[strum(serialize = "W")]
    West,
}

fn main() {
    println!("--- Displaying Enum Variants ---");
    let dir = Direction::North;
    // Using the `Display` trait.
    println!("Current direction: {}", dir); // Prints "Current direction: North".

    println!("\n--- Parsing Strings into Enums ---");
    let input_string_south = "South";
    let input_string_invalid = "Up";

    // Using the EnumString trait:
    match input_string_south.parse::<Direction>() {
        Ok(direction) => println!(
            "Parsed '{}' successfully: {:?}.",
            input_string_south, direction
        ),
        Err(e) => println!("Failed to parse '{}': {}.", input_string_south, e),
    }

    match input_string_invalid.parse::<Direction>() {
        Ok(direction) => println!(
            "Parsed '{}' successfully: {:?}.",
            input_string_invalid, direction
        ),
        Err(e) => {
            println!("Failed to parse '{}': {}.", input_string_invalid, e)
        }
    }

    println!("\n--- Iterating Over Enum Variants ---");
    // Using the `EnumIter` trait:
    println!("All possible directions:");
    for direction in Direction::iter() {
        println!("- {:?}", direction);
    }
    // This will print:
    // - North
    // - East
    // - South
    // - West
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
