//! Runs the file metadata example.
//!
//! TODO Reviewed ??-??-2026.

/// Detect a file's type from its first bytes.
mod infer;
/// Reading the metadata of files and directories.
mod metadata;

/// Runs the metadata example.
fn main() -> anyhow::Result<()> {
    metadata::run()?;
    infer::run()?;
    Ok(())
}
