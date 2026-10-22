//! Runs the basic authentication example.
//!
//! TODO Reviewed ??-??-2026

mod basic;

/// Runs the example from the `basic` module.
fn main() -> anyhow::Result<()> {
    basic::run()?;
    Ok(())
}
