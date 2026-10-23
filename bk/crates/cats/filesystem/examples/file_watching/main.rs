//! Runs the file watching example.
//!
//! TODO Reviewed ??-??-2026.

mod notify;

/// Runs the file watching example.
fn main() -> anyhow::Result<()> {
    notify::run()?;
    Ok(())
}
