//! Crate registries examples.
//!
//! TODO Reviewed ??-??-2026

mod crates_io;

/// Runs the `crates.io` examples.
fn main() -> anyhow::Result<()> {
    crates_io::run()?;
    Ok(())
}
