# Cryptocurrencies

{{#include cryptocurrencies.incl.md}}

Notable crypto projects using Rust include:

- Parity Technologies allows building blockchain infrastructure like [`Polkadot`][wikipedia~polkadot]↗{{hi:Polkadot}} and [`Substrate`][substrate~github]↗{{hi:Substrate}}.
- [`Solana`][c~solana~docs]↗{{hi:Solana}} is a high-performance blockchain platform with smart contract functionality.
- The [`NEAR`][near~website]↗{{hi:NEAR}} Protocol is a user-friendly and scalable blockchain platform.
- [`Grin`][c~grin~docs]↗{{hi:Grin}} is a privacy-focused cryptocurrency implementing the Mimblewimble protocol.
- [`rust-bitcoin`][c~rust-bitcoin~docs]{{hi:rust-bitcoin}} is a library providing support for Bitcoin's core functionalities.

## Cryptocurrencies {#cryptocurrencies}

```rust,editable
{{#include ../../../crates/cats/cryptography_cryptocurrencies/examples/cryptocurrencies/cryptocurrencies.rs:example}}
```

## Related Topics {#related-topics}

- Cryptographic Primitives: [`ring`][c~ring~docs]↗{{hi:ring}}, [`rust-crypto`][c~rust-crypto~docs]↗{{hi:rust-crypto}} suite, [`sha2`][c~sha2~docs]↗{{hi:sha2}}, [`secp256k1`][c~secp256k1~docs]↗{{hi:secp256k1}} (for elliptic curve [cryptography][p~cryptography], often used in Bitcoin).
- Blockchain Data Structures: These are often implemented directly without dedicated crates, but libraries like [`bitcoin`][c~bitcoin~docs]↗{{hi:bitcoin}} may provide some. See also [[data-structures | Data Structures]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
