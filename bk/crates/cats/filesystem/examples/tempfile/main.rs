//! Runs the temporary file example.
//!
//! TODO Reviewed ??-??-2026

mod tempfile;

fn main() -> anyhow::Result<()> {
    tempfile::run()?;
    Ok(())
}
