//! Runs the basic authentication example.
//!
//!
//! TODO Reviewed ??-??-2026.

/// Builds a Basic-authenticated request locally without sending credentials.
mod basic;

/// Runs the basic authentication example.
fn main() -> anyhow::Result<()> {
    basic::run()?;
    Ok(())
}
