// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Queries an Ethereum balance and signs a message with `alloy`, without
//! network access.
//!
//! The provider talks to a mocked transport that returns canned JSON-RPC
//! responses. Replace `connect_mocked_client` with
//! `connect_http("https://...".parse()?)` to query a real node. The ignored
//! test `live_node_balance_works` does exactly that: run it with
//! `ALLOY_RPC_URL=https://... cargo test --example cryptocurrencies -- --ignored`.

#[cfg(test)]
use std::env;

use alloy::primitives::Address;
use alloy::primitives::Bytes;
use alloy::primitives::U256;
use alloy::primitives::address;
use alloy::primitives::utils::format_ether;
use alloy::primitives::utils::parse_ether;
use alloy::providers::Provider;
use alloy::providers::ProviderBuilder;
use alloy::providers::mock::Asserter;
use alloy::signers::Signer;
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;

/// An example account (`vitalik.eth`).
const ACCOUNT: Address = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

sol! {
    /// The part of the ERC-20 token interface used by this example.
    #[sol(rpc)]
    interface IERC20 {
        function balanceOf(address owner) external view returns (uint256);
    }
}

/// An example ERC-20 token contract (USDC on Ethereum mainnet).
const TOKEN: Address = address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");

/// Returns the ERC-20 token balance of `account` in the token's smallest unit.
async fn token_balance(
    provider: &impl Provider,
    token: Address,
    account: Address,
) -> anyhow::Result<U256> {
    let contract = IERC20::new(token, provider);
    Ok(contract.balanceOf(account).call().await?)
}

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
#[cfg(test)]
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
    // Queue the responses to the two JSON-RPC calls made below, in order:
    // `eth_getBalance` (1.5 ether, expressed in wei), then the `eth_call`
    // behind `balanceOf` (a 32-byte big-endian integer).
    let asserter = Asserter::new();
    asserter.push_success(&parse_ether("1.5")?);
    asserter.push_success(&Bytes::from(
        U256::from(42_000_000_u64).to_be_bytes::<32>(),
    ));
    let provider = ProviderBuilder::new().connect_mocked_client(asserter);

    let balance = balance_in_ether(&provider, ACCOUNT).await?;
    println!("Balance of {ACCOUNT}: {balance} ETH");

    let tokens = token_balance(&provider, TOKEN, ACCOUNT).await?;
    println!("Token balance of {ACCOUNT}: {tokens}");

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

    /// The ERC-20 `balanceOf` call is ABI-encoded by `sol!` and its
    /// `eth_call` result is decoded from the mocked response.
    #[tokio::test]
    async fn erc20_balance_is_decoded() -> anyhow::Result<()> {
        let asserter = Asserter::new();
        let raw = U256::from(42_000_000_u64).to_be_bytes::<32>();
        asserter.push_success(&Bytes::from(raw));
        let provider = ProviderBuilder::new().connect_mocked_client(asserter);
        let balance = token_balance(&provider, TOKEN, ACCOUNT).await?;
        assert_eq!(balance, U256::from(42_000_000_u64));
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
