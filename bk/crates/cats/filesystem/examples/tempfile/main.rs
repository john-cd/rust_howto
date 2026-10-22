//! Runs the temporary file example.
//!
//! TODO Reviewed ??-??-2026

mod tempfile;

/// Runs the example from the `tempfile` module.
fn main() -> anyhow::Result<()> {
    tempfile::run()?;
    Ok(())
}
