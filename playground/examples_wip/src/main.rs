//! Placeholder crate for work-in-progress examples.
//!
//! TODO Reviewed ??-??-2026.

/// Points to the examples folder.
fn main() {
    println!("{MESSAGE}");
}

/// The message printed by the placeholder.
const MESSAGE: &str = "Look in the examples folder.";

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the program default command successfully.
    #[test]
    fn main_runs() {
        main();
    }

    /// Includes the available examples in the displayed help message.
    #[test]
    fn message_mentions_examples() {
        assert!(MESSAGE.contains("examples"));
    }
}
