//! Runs the symbolic links example.
//!
//! TODO Reviewed ??-??-2026.

mod symbolic_links;

/// Runs the symbolic links example.
fn main() -> anyhow::Result<()> {
    symbolic_links::run()?;
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
