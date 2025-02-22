# Cryptocurrencies

{{#include cryptocurrencies.incl.md}}

Notable crypto projects using Rust include:

- Parity Technologies allows building blockchain infrastructure like Polkadot and Substrate.
- `Solana` is a high-performance blockchain platform with smart contract functionality.
- The `NEAR` Protocol is a user-friendly and scalable blockchain platform.
- `Grin` is a privacy-focused cryptocurrency implementing the Mimblewimble protocol.
- `Rust-Bitcoin` is a library providing support for Bitcoin's core functionalities.

## Cryptocurrencies {#cryptocurrencies}

```rust,editable
{{#include ../../../crates/cats/cryptography_cryptocurrencies/tests/cryptocurrencies.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[cryptocurrencies: write (P3)](https://github.com/john-cd/rust_howto/issues/278)
Link to:

Cryptographic Primitives: ring, rust-crypto, sha2, secp256k1 (for elliptic curve [cryptography][p-cryptography], often used in Bitcoin)
Blockchain Data Structures: (Often implemented directly without dedicated crates, but libraries like bitcoin may provide some).
Bitcoin: bitcoin
Ethereum: web3 (for interacting with Ethereum), ethers
Other Cryptocurrencies: (Many cryptocurrencies have their own Rust libraries. Search for crates specific to the coin you're interested in.)
Wallet Development: (Often involves a combination of the above crates and custom logic)
Smart Contracts (Solidity): (Primarily a Solidity ecosystem, but Rust may be used for [testing][p-testing] or interacting with contracts via crates like ethers)

</div>
