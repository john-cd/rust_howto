//! Runs the market data examples.
//!
//! TODO Reviewed ??-??-2026.

mod yahoo_history;

/// Runs the market data example (requires network access).
fn main() -> anyhow::Result<()> {
    yahoo_history::run()
}
