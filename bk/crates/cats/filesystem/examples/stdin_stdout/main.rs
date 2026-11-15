//! Runs the standard input and output example.
//!
//! TODO Reviewed ??-??-2026.

/// Reading from standard input and writing to standard output.
mod stdin_stdout;

/// Runs the stdin stdout example.
fn main() -> anyhow::Result<()> {
    stdin_stdout::run()?;
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
