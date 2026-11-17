//! Runs the behavioral pattern examples.
//!
//! TODO Reviewed ??-??-2026.

/// Implements an undo and redo history with reversible insert commands.
mod undo_redo;
/// Encodes an article's lifecycle in its type.
mod typestate;

/// Runs the typestate and undo/redo examples.
fn main() {
    typestate::run();
    undo_redo::run();
}
