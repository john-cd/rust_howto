//! Runs the behavioral pattern examples.
//!
//! TODO Reviewed ??-??-2026.

/// Encodes an article's lifecycle in its type.
mod typestate;
/// Implements an undo and redo history with reversible insert commands.
mod undo_redo;

/// Runs the typestate and undo/redo examples.
fn main() {
    typestate::run();
    undo_redo::run();
}
