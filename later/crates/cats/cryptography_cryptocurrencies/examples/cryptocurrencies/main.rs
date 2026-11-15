//! Runs the cryptocurrencies example.
//!
//! TODO Reviewed ??-??-2026.

/// Queries an Ethereum balance and signs a message with `alloy`, without
mod cryptocurrencies;

/// Runs the cryptocurrencies example.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cryptocurrencies::run().await
}
