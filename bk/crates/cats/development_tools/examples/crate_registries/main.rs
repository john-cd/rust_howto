//! Crate registries examples.
//!
//! TODO Reviewed ??-??-2026.

/// Examples of use of the 'crates.io' API.
mod crates_io;

/// Runs the crate registries example.
fn main() -> anyhow::Result<()> {
    crates_io::run()?;
    Ok(())
}
