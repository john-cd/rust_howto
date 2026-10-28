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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
