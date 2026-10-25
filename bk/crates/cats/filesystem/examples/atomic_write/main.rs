//! Runs the crash-safe atomic file replacement example.
//!
//! TODO Reviewed ??-??-2026.

mod atomic_write;

/// Runs the atomic write example.
fn main() -> anyhow::Result<()> {
    atomic_write::run()?;
    Ok(())
}
