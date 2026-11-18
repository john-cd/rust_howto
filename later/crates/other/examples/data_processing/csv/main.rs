//! Runs the CSV read/write example.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates typed CSV reading and writing.
mod csv;

/// Runs the CSV read/write example.
fn main() -> anyhow::Result<()> {
    csv::run()
}
