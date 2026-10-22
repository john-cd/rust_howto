//! Runs the file watching example.
//!
//! TODO Reviewed ??-??-2026

mod notify;

fn main() -> anyhow::Result<()> {
    notify::run()?;
    Ok(())
}
