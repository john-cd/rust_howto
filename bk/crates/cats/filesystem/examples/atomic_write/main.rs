//! Runs the crash-safe atomic file replacement example.
//!
//! TODO Reviewed ??-??-2026.

/// Replace a file so that readers never see a half-written file.
mod atomic_write;

/// Runs the atomic write example.
fn main() -> anyhow::Result<()> {
    atomic_write::run()?;
    Ok(())
}
