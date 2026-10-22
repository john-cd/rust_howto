//! Enums with unit-like, struct-like and tuple-like variants.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
/// An enum to represent different types of messages.
#[derive(Debug)]
enum Message {
    /// A variant without data.
    Quit, // Unit-like variant (no fields).
    /// A variant with named fields.
    Move {
        /// Distance to move in the x direction.
        x: i32,
        /// Distance to move in the y direction.
        y: i32,
    }, // Struct-like variant (named fields).
    /// A variant that holds a string.
    Write(String), // Tuple-like variant (numbered fields).
    /// A variant that holds three color components.
    ChangeColor(i32, i32, i32), // Another tuple-like variant.
}

/// Define methods on enums.
impl Message {
    /// Prints a description of the message.
    fn call(&self) {
        // The `match` keyword allows us to compare a value against a series of
        // patterns. Each pattern can have associated code that will be
        // run if the value matches the pattern.
        match self {
            Message::Quit => println!("The Quit variant has no data to show."),
            Message::Move { x, y } => println!(
                "Move in the x direction {x} and in the y direction {y}"
            ),
            Message::Write(text) => println!("Message: {text}"),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}")
            }
        }
        // Enums make your code safer, because the compiler knows all the
        // possible variants a value can have, and makes sure the
        // `match` expression handles all possibilities.
    }
}

/// Creates a message of each kind and calls it.
fn main() {
    // `msg` is assigned one of the variants.
    // Note the :: between the name of the type and the name of the variant.
    let msg = Message::Quit;
    println!("{msg:?}");
    // Or
    let msg = Message::Move { x: 10, y: 15 };
    println!("{msg:?}");
    // Or
    let msg = Message::ChangeColor(127, 0, 0);
    println!("{msg:?}");

    let msg = Message::Write(String::from("hello"));
    msg.call();
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
