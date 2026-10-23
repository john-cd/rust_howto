//! Runs the file metadata example.
//!
//! TODO Reviewed ??-??-2026.

mod metadata;

/// Runs the metadata example.
fn main() -> anyhow::Result<()> {
    metadata::run()?;
    Ok(())
}
