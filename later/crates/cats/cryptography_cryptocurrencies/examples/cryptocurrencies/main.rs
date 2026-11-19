//! Runs the offline cryptocurrency examples.
//!
//! TODO Reviewed ??-??-2026.

/// Checks a Bitcoin address and constructs its output script.
mod bitcoin_address;
/// Reads mocked Ethereum balances and verifies a message signature.
mod cryptocurrencies;
/// Derives and verifies a Solana program address.
mod solana_address;

/// Runs the Ethereum, Bitcoin, and Solana recipes.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cryptocurrencies::run().await?;
    bitcoin_address::run()?;
    solana_address::run()
}
