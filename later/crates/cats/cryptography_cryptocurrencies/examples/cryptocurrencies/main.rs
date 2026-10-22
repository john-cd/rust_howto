//! Runs the cryptocurrencies example.
//!
//! TODO Reviewed ??-??-2026

mod cryptocurrencies;

/// Runs the example from the `cryptocurrencies` module.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    cryptocurrencies::run()
}
