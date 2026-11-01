//! Runs the basic authentication example.
//!
//! TODO Reviewed 10-27-2026.

mod basic;

/// Runs the basic authentication example.
fn main() -> anyhow::Result<()> {
    basic::run()?;
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
