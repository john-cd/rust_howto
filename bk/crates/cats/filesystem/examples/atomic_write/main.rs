//! Runs the crash-safe atomic file replacement example.
//!
//! TODO Reviewed ??-??-2026.

mod atomic_write;

/// Runs the atomic write example.
fn main() -> anyhow::Result<()> {
    atomic_write::run()?;
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
