<!-- TODO Reviewed ??-??-2026. -->

# Offline Ethereum, Bitcoin, and Solana Examples

{{#include cryptocurrencies.incl.md}}

Run all three examples from the repository root. They use public fixtures and a mocked RPC transport, so they need no node, SDK, funded account, or private key from the reader.

```console
just cargo run --manifest-path later/crates/Cargo.toml --locked -p cryptography_cryptocurrencies --example cryptocurrencies
```

## Read a Mocked Ethereum Balance and Verify a Signature {#cryptocurrencies}

[Alloy][c~alloy~docs] exposes Ethereum JSON-RPC methods through a provider. The mock queues the results of `eth_getBalance` and `eth_call` in request order. The recipe reports 1.5 ETH and a raw token balance of 42,000,000. Ether uses 18 decimal places. An ERC-20 uses the precision defined by its contract, so look up `decimals()` before displaying a raw token balance.

The recipe also signs a message with a fresh local key and recovers the same address from the signature. The printed address changes on each run. A signed message is separate from an on-chain transaction.

```rust,editable
{{#include ../../../crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs:example}}
```

The ignored `live_node_balance_works` test reads `ALLOY_RPC_URL` and sends a read-only balance request to an Ethereum JSON-RPC endpoint. The mock tests are sufficient to exercise the example without exposing credentials.

## Validate a Bitcoin Address and Derive Its Output Script {#bitcoin-address}

Parsing with [rust-bitcoin][c~bitcoin~docs] returns an `Address<NetworkUnchecked>`. Call `require_network` with the network your application expects before producing an output script. The fixture produces a pay-to-script-hash script beginning with `a914` and ending with `87`. Some address formats share prefixes across test networks, so retain the wallet or node's configured network independently.

```rust,editable
{{#include ../../../crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/bitcoin_address.rs:example}}
```

## Derive and Verify a Solana Program Address {#solana-program-address}

A program-derived address (PDA) is an account identifier derived from a program ID, application seeds, and a one-byte bump. The program ID distinguishes programs. In this example, the fixed `record` label and an eight-byte record ID distinguish records within the program. Keep the seed encoding stable across clients and programs.

Use [solana-pubkey][c~solana-pubkey~docs] with its `curve25519` feature to derive the address on the host. The example prints the same address and bump on each run, then verifies the result by deriving it again with the explicit bump. See the [Solana PDA documentation][solana~pda] for the runtime rules.

```rust,editable
{{#include ../../../crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/solana_address.rs:example}}
```

## Related Topics {.skip}

- [[cryptography | Cryptography]].
- [[security | Protect Secrets]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
