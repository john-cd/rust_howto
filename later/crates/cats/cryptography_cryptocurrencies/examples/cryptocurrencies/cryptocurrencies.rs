// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Queries an Ethereum balance and signs a message with `alloy`, without
//! network access.
//!
//! The provider talks to a mocked transport that returns canned JSON-RPC
//! responses. Replace `connect_mocked_client` with
//! `connect_http("https://...".parse()?)` to query a real node.

use std::env;

use alloy::primitives::Address;
use alloy::primitives::address;
use alloy::primitives::utils::format_ether;
use alloy::primitives::utils::parse_ether;
use alloy::providers::Provider;
use alloy::providers::ProviderBuilder;
use alloy::providers::mock::Asserter;
use alloy::signers::Signer;
use alloy::signers::local::PrivateKeySigner;

/// An example account (`vitalik.eth`).
const ACCOUNT: Address = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

/// Returns the balance of `account`, formatted in ether.
async fn balance_in_ether(
    provider: &impl Provider,
    account: Address,
) -> anyhow::Result<String> {
    let wei = provider.get_balance(account).await?;
    Ok(format_ether(wei))
}

/// Signs `message` with a fresh random key and checks that the signer's
/// address can be recovered from the signature.
async fn sign_and_recover(message: &str) -> anyhow::Result<Address> {
    let signer = PrivateKeySigner::random();
    let signature = signer.sign_message(message.as_bytes()).await?;
    let recovered = signature.recover_address_from_msg(message)?;
    assert_eq!(recovered, signer.address());
    Ok(recovered)
}

/// Queries the same balance from a live node selected with `ALLOY_RPC_URL`.
async fn live_node_balance() -> anyhow::Result<()> {
    let url = env::var("ALLOY_RPC_URL")?;
    let url = url.parse()?;
    let provider = ProviderBuilder::new().connect_http(url);
    let balance = balance_in_ether(&provider, ACCOUNT).await?;
    println!("Balance of {ACCOUNT}: {balance} ETH");
    Ok(())
}

/// Runs the example against a mocked node.
async fn main() -> anyhow::Result<()> {
    // Queue the response to the one JSON-RPC call made below
    // (`eth_getBalance`): 1.5 ether, expressed in wei.
    let asserter = Asserter::new();
    asserter.push_success(&parse_ether("1.5")?);
    let provider = ProviderBuilder::new().connect_mocked_client(asserter);

    let balance = balance_in_ether(&provider, ACCOUNT).await?;
    println!("Balance of {ACCOUNT}: {balance} ETH");

    let signer_address = sign_and_recover("hello, Ethereum").await?;
    println!("Recovered signer: {signer_address}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub async fn run() -> anyhow::Result<()> {
    main().await
}

#[cfg(test)]
mod tests {
    use alloy::primitives::U256;

    use super::*;

    /// The recipe runs end to end against the mocked node.
    #[tokio::test]
    async fn main_runs() -> anyhow::Result<()> {
        main().await
    }

    /// The balance is read from the mocked response and formatted in ether.
    #[tokio::test]
    async fn balance_is_formatted_in_ether() -> anyhow::Result<()> {
        let asserter = Asserter::new();
        asserter.push_success(&U256::from(2_500_000_000_000_000_000_u64));
        let provider = ProviderBuilder::new().connect_mocked_client(asserter);
        let balance = balance_in_ether(&provider, ACCOUNT).await?;
        assert_eq!(balance, "2.500000000000000000");
        Ok(())
    }

    /// A node error is reported instead of panicking.
    #[tokio::test]
    async fn node_errors_are_propagated() {
        let asserter = Asserter::new();
        asserter.push_failure_msg("node unavailable");
        let provider = ProviderBuilder::new().connect_mocked_client(asserter);
        assert!(balance_in_ether(&provider, ACCOUNT).await.is_err());
    }

    /// Signing and recovery round-trip.
    #[tokio::test]
    async fn signature_recovers_the_signer() -> anyhow::Result<()> {
        let first = sign_and_recover("one").await?;
        let second = sign_and_recover("one").await?;
        assert_ne!(first, second, "each call uses a fresh random key");
        Ok(())
    }

    /// Queries a live Ethereum node when explicitly enabled by the developer.
    #[tokio::test]
    #[ignore = "requires ALLOY_RPC_URL for an Ethereum JSON-RPC endpoint"]
    async fn live_node_balance_works() -> anyhow::Result<()> {
        live_node_balance().await
    }
}
