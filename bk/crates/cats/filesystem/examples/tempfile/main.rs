//! Runs the temporary file example.
//!
//! TODO Reviewed ??-??-2026.

mod tempfile;

/// Runs the tempfile example.
fn main() -> anyhow::Result<()> {
    tempfile::run()?;
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
