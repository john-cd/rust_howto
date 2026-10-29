// TODO Reviewed ??-??-2026.
// TODO (#707): implement real Alloy interactions or revise this conceptual walkthrough.

// ANCHOR: example
//! This example demonstrates basic interactions with an Ethereum-compatible
//! blockchain using the `alloy` library.
//!
//! It shows how to:
//! 1. Configure an RPC endpoint and target address.
//! 2. Show where chain calls would be made.
//! 3. Print a compile-safe walkthrough of the flow.

use std::error::Error;

/// Prints the steps to query the balance of an address, without connecting to a
/// network.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let rpc_url = "https://eth.llamarpc.com";
    let target_address = "d8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

    println!("RPC endpoint: {rpc_url}");
    println!("Target address: {target_address}");
    println!("Step 1: connect provider to endpoint");
    println!("Step 2: request current block number");
    println!("Step 3: request address balance");
    println!("In production, wire these steps to your Ethereum client crate.");

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider() -> Result<(), Box<dyn Error + Send + Sync>> {
        main()
    }
}
