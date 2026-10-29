// TODO Reviewed ??-??-2026.

#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the snippet declares items to illustrate them, not to call them"
    )
)]
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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the destructure enum example through its entry point.
    #[test]
    fn test_destructure_enum_example_entry_point() {
        main();
    }

    /// Every variant can be destructured.
    #[test]
    fn test_destructure_variants() {
        let describe = |m: Message| match m {
            Message::Write(text) => format!("write {text}"),
            Message::Move { x, y } => format!("move {x},{y}"),
            Message::Quit => "quit".to_owned(),
        };
        assert_eq!(describe(Message::Write("hi".into())), "write hi");
        assert_eq!(describe(Message::Move { x: 1, y: -2 }), "move 1,-2");
        assert_eq!(describe(Message::Quit), "quit");
    }
}
