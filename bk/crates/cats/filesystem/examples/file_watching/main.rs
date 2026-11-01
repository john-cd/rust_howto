//! Runs the file watching example.
//!
//! TODO Reviewed ??-??-2026.

mod notify;

/// Runs the file watching example.
fn main() -> anyhow::Result<()> {
    notify::run()?;
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
