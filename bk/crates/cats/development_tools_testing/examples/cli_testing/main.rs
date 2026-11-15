//! Examples of testing command-line programs and reading coverage reports.
//!
//! TODO Reviewed ??-??-2026.

/// Compute line coverage per file from an LCOV report and enforce a minimum.
mod lcov_summary;

/// Runs the examples.
fn main() -> anyhow::Result<()> {
    lcov_summary::run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example end to end and asserts that it succeeds.
    #[test]
    fn test_main_runs() {
        assert!(main().is_ok());
    }
}
