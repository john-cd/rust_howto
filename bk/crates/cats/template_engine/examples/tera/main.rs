//! Runs the `tera` template example.
//!
//! TODO Reviewed ??-??-2026

mod tera;

/// Runs the example from the `tera` module.
fn main() -> anyhow::Result<()> {
    tera::run()?;
    Ok(())
}
