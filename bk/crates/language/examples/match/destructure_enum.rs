#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Destructuring an enum in a `match`.

/// A message of different kinds.
enum Message {
    /// A message without data.
    Quit,
    /// A message with coordinates.
    Move {
        /// The x coordinate.
        x: i32,
        /// The y coordinate.
        y: i32,
    },
    /// A message with text.
    Write(String),
}

/// Matches a message and extracts its data.
fn main() {
    let msg = Message::Write(String::from("hello"));

    match msg {
        Message::Write(text) => println!("Text message: {text}"),
        Message::Move { x, y } => println!("Move to x: {x}, y: {y}"),
        Message::Quit => println!("Quit"),
        // Must be exhaustive.
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
