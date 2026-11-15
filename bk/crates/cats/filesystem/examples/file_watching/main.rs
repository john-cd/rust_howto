//! Runs the file watching example.
//!
//! TODO Reviewed ??-??-2026.

/// Add to your `Cargo.toml`: ```toml [dependencies] notify = "8.2.0" ```
mod notify;

/// Runs the file watching example.
fn main() -> anyhow::Result<()> {
    notify::run()?;
    Ok(())
}
