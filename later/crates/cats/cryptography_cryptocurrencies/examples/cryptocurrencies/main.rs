//! Runs the cryptocurrencies example.
//!
//! TODO Reviewed ??-??-2026.

mod cryptocurrencies;

/// Runs the cryptocurrencies example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    cryptocurrencies::run()
}
