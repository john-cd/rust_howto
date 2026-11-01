//! Runs the file metadata example.
//!
//! TODO Reviewed ??-??-2026.

mod infer;
mod metadata;

/// Runs the metadata example.
fn main() -> anyhow::Result<()> {
    metadata::run()?;
    infer::run()?;
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
