//! Runs the basic authentication example.
//!
//! TODO Reviewed ??-??-2026.

mod basic;

/// Runs the basic authentication example.
fn main() -> anyhow::Result<()> {
    basic::run()?;
    Ok(())
}
