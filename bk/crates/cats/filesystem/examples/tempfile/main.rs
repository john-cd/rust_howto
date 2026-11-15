//! Runs the temporary file example.
//!
//! TODO Reviewed ??-??-2026.

/// Creating temporary files and directories with `tempfile`.
mod tempfile;

/// Runs the tempfile example.
fn main() -> anyhow::Result<()> {
    tempfile::run()?;
    Ok(())
}
