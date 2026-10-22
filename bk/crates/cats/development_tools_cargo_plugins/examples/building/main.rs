//! Example building cargo plugins and executing shells
//!
//! TODO Reviewed ??-??-2026

mod xshell;

/// Runs the `xshell` example.
fn main() -> anyhow::Result<()> {
    xshell::run()?;
    Ok(())
}
