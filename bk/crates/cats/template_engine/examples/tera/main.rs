//! Runs the `tera` template example.
//!
//! TODO Reviewed ??-??-2026.

mod tera;

/// Runs the tera example.
fn main() -> anyhow::Result<()> {
    tera::run()?;
    Ok(())
}
