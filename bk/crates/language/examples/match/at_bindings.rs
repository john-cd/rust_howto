// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Binding a value to a name while matching a pattern with `@`.

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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the at bindings example through its entry point.
    #[test]
    fn test_at_bindings_example_entry_point() {
        main();
    }

    /// Each range selects the matching arm.
    #[test]
    fn test_at_binding_arms() {
        fn classify(id: i32) -> &'static str {
            match (Message::Hello { id }) {
                Message::Hello { id: 3..=7 } => "in range",
                Message::Hello { id: 10..=12 } => "other range",
                Message::Hello { .. } => "other",
            }
        }
        assert_eq!(classify(5), "in range");
        assert_eq!(classify(11), "other range");
        assert_eq!(classify(8), "other");
        assert_eq!(classify(2), "other");
    }
}
