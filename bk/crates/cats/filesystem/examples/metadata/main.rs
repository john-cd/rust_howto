//! Runs the file metadata example.
//!
//! TODO Reviewed ??-??-2026.

mod infer;
mod metadata;

/// Runs the metadata example.
fn main() -> anyhow::Result<()> {
    metadata::run()?;
    infer::run()?;
    Ok(())
}
