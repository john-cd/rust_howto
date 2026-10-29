//! Runs the market data examples.
//!
//! TODO Reviewed ??-??-2026.

mod yahoo_history;

/// Runs the market data example (requires network access).
fn main() -> anyhow::Result<()> {
    yahoo_history::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    #[ignore = "needs network access"]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
