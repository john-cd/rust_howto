//! Runs the clipboard example.
//!
//! TODO Reviewed ??-??-2026.

/// This example copies text to and reads text from the system clipboard with
mod arboard;

/// Runs the clipboard example.
fn main() -> anyhow::Result<()> {
    arboard::run()
}
