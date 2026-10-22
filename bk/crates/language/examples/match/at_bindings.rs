//! Binding a value to a name while matching a pattern with `@`.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
/// A message with an identifier.
enum Message {
    /// A greeting.
    Hello {
        /// The identifier.
        id: i32,
    },
}

/// Matches an identifier against a range and binds it.
fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        // The `id_val` variable is bound to `msg.id` if it's in 3..=7.
        Message::Hello { id: id_val @ 3..=7 } => {
            println!("Found an id in range: {id_val}.");
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range (no binding here).");
        }
        Message::Hello { id } => {
            println!("Found some other id: {id}.");
        }
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
