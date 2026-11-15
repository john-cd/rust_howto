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

    #[test]
    fn main_runs() {
        main();
    }

    #[test]
    fn message_mentions_examples() {
        assert!(MESSAGE.contains("examples"));
    }
}
