//! Runs the symbolic links example.
//!
//! TODO Reviewed ??-??-2026.

mod symbolic_links;

/// Runs the symbolic links example.
fn main() -> anyhow::Result<()> {
    symbolic_links::run()?;
    Ok(())
}
