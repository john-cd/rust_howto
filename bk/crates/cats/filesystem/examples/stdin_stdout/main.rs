//! Runs the standard input and output example.
//!
//! TODO Reviewed ??-??-2026.

mod stdin_stdout;

/// Runs the stdin stdout example.
fn main() -> anyhow::Result<()> {
    stdin_stdout::run()?;
    Ok(())
}
