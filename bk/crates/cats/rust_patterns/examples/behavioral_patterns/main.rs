//! Runs the behavioral pattern examples.
//!
//! TODO Reviewed ??-??-2026.

mod undo_redo;

/// Runs the undo/redo example.
fn main() {
    undo_redo::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn main_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
