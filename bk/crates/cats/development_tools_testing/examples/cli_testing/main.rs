//! Examples of testing command-line programs and reading coverage reports.
//!
//! TODO Reviewed ??-??-2026.

mod cli_test;
mod lcov_summary;

/// Runs the examples.
fn main() -> anyhow::Result<()> {
    cli_test::run()?;
    lcov_summary::run()?;
    Ok(())
}
