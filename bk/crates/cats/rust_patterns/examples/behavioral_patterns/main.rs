//! Runs the behavioral pattern examples.
//!
//! TODO Reviewed ??-??-2026.

/// Implements an undo and redo history with reversible insert commands.
mod undo_redo;
/// Runs the example.
/// Runs the undo/redo example.
fn main() {
    undo_redo::run();
}

/// Verifies that the behavioral pattern examples.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
