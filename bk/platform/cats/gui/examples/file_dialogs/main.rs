//! Runs the file dialog example.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates opening a file dialog using the rfd crate.
mod rfd;

/// Runs the file dialogs example.
fn main() -> anyhow::Result<()> {
    rfd::run()
}
