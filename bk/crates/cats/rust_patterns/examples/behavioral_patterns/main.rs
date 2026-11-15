//! Runs the behavioral pattern examples.
//!
//! TODO Reviewed ??-??-2026.

/// Implements an undo and redo history with reversible insert commands.
mod undo_redo;

/// Runs the undo/redo example.
fn main() {
    undo_redo::run();
}
