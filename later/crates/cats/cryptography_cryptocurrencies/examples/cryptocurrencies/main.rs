//! Runs the cryptocurrencies example.
//!
//! TODO Reviewed ??-??-2026.

mod cryptocurrencies;

/// Runs the cryptocurrencies example.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cryptocurrencies::run().await
}
