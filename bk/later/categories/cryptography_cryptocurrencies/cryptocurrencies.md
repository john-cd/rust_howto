# Cryptocurrencies

{{#include cryptocurrencies.incl.md}}

Notable crypto projects using Rust include:

- Parity Technologies allows building blockchain infrastructure like `Polkadot` and `Substrate`.
- `Solana` is a high-performance blockchain platform with smart contract functionality.
- The `NEAR` Protocol is a user-friendly and scalable blockchain platform.
- [`Grin`][c~grin~docs]⮳{{hi:Grin}} is a privacy-focused cryptocurrency implementing the Mimblewimble protocol.
- `Rust-Bitcoin` is a library providing support for Bitcoin's core functionalities.

## Cryptocurrencies {#cryptocurrencies}

```rust,editable
{{#include ../../../crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs:example}}
```

## Related Topics {#skip}

- Cryptographic Primitives: [`ring`][c~ring~docs]⮳{{hi:ring}}, [`rust-crypto`][c~rust_crypto~docs]⮳{{hi:rust-crypto}} suite, [`sha2`][c~sha2~docs]⮳{{hi:sha2}}, `secp256k1` (for elliptic curve [cryptography][p~cryptography], often used in Bitcoin).
- Blockchain Data Structures: These are often implemented directly without dedicated crates, but libraries like `bitcoin` may provide some. See also [[data-structures | Data Structures]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
