//! Runs the standard input and output example.
//!
//! TODO Reviewed ??-??-2026

mod stdin_stdout;

/// Runs the example from the `stdin_stdout` module.
fn main() -> anyhow::Result<()> {
    stdin_stdout::run()?;
    Ok(())
}
