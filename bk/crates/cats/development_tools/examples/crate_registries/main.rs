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

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example end to end and asserts that it succeeds.
    #[test]
    #[ignore = "requires crates.io API access"]
    fn test_main_runs() {
        assert!(main().is_ok());
    }
}
