//! Runs the symbolic links example.
//!
//! TODO Reviewed ??-??-2026

mod symbolic_links;

/// Runs the example from the `symbolic_links` module.
fn main() -> anyhow::Result<()> {
    symbolic_links::run()?;
    Ok(())
}
