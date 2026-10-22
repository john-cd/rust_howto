//! Runs the file metadata example.
//!
//! TODO Reviewed ??-??-2026

mod metadata;

/// Runs the example from the `metadata` module.
fn main() -> anyhow::Result<()> {
    metadata::run()?;
    Ok(())
}
