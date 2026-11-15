//! Runs the market data examples.
//!
//! TODO Reviewed ??-??-2026.

/// Fetch daily bars from Yahoo!
mod yahoo_history;

/// Runs the market data example (requires network access).
fn main() -> anyhow::Result<()> {
    yahoo_history::run()
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    #[ignore = "needs network access"]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
