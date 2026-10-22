//! Runs the file watching example.
//!
//! TODO Reviewed ??-??-2026

mod notify;

/// Runs the example from the `notify` module.
fn main() -> anyhow::Result<()> {
    notify::run()?;
    Ok(())
}
