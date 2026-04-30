#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates basic interactions with an Ethereum-compatible
//! blockchain using the `alloy` library.
//!
//! It shows how to:
//! 1. Create a provider connecting to a public HTTP RPC endpoint.
//! 2. Retrieve the current block number.
//! 3. Fetch the balance of a specific address.

use alloy::primitives::address;
use alloy::providers::Provider;
use alloy::providers::ProviderBuilder;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize a provider using a public RPC URL.
    let rpc_url = "https://eth.llamarpc.com".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // 2. Fetch the latest block number from the network.
    let block_number = provider.get_block_number().await?;
    println!("Latest Ethereum block number: {block_number}");

    // 3. Check the balance of a known account (e.g., vitalik.eth).
    let target_address = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let balance = provider.get_balance(target_address).await?;

    println!("Balance of {target_address}: {balance}");

    Ok(())
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_provider() -> Result<()> {
        main().await
    }
}
