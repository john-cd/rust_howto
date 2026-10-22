//! Runs the basic authentication example.
//!
//! TODO Reviewed ??-??-2026

mod basic;

fn main() -> anyhow::Result<()> {
    basic::run()?;
    Ok(())
}
