//! Example building cargo plugins and executing shells
//!
//! TODO Reviewed ??-??-2026.

mod xshell;

/// Runs the building example.
fn main() -> anyhow::Result<()> {
    xshell::run()?;
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
