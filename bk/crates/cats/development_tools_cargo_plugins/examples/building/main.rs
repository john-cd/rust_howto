//! Example building cargo plugins and executing shells
//!
//! TODO Reviewed ??-??-2026.

/// This example demonstrates the usage of the `xshell` crate for shell
mod xshell;

/// Runs the building example.
fn main() -> anyhow::Result<()> {
    xshell::run()?;
    Ok(())
}
