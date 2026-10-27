<!-- TODO Reviewed ??-??-2026 -->

# Cryptocurrencies

[![cat~cryptography::cryptocurrencies][cat~cryptography::cryptocurrencies~badge]][cat~cryptography::cryptocurrencies]{{hi:Cryptocurrencies}}

Rust cryptocurrency applications combine protocol clients, wallets, smart-contract frameworks, and cryptographic primitives. Choose libraries that match the protocol and protect private keys with an explicit key-management design.

{{#include cryptocurrencies.incl.md}}

## Bitcoin

[![bitcoin][c~bitcoin~docs~badge]][c~bitcoin~docs] [![bdk_wallet][c~bdk_wallet~docs~badge]][c~bdk_wallet~docs]{{hi:Bitcoin}}

[`bitcoin`][c~bitcoin~docs]↗ provides Bitcoin protocol types, parsing, and serialization; [`bdk_wallet`][c~bdk_wallet~docs]↗ provides descriptor-based wallet building blocks.

## Ethereum

[![alloy][c~alloy~docs~badge]][c~alloy~docs]{{hi:Ethereum}}

[`alloy`][c~alloy~docs]↗ provides Ethereum types, providers, signers, and network access. The older `ethers` ecosystem is deprecated in favor of Alloy; see the [upstream deprecation notice][ethers-rs~repo]↗.

## Solana Programs

[![solana-sdk][c~solana-sdk~docs~badge]][c~solana-sdk~docs] [![anchor-lang][c~anchor-lang~docs~badge]][c~anchor-lang~docs]{{hi:Solana}}

[`solana-sdk`][c~solana-sdk~docs]↗ provides Solana client and program types; [`anchor-lang`][c~anchor-lang~docs]↗ is part of Anchor, a framework for writing Solana programs.

## Cryptographic Primitives

[![k256][c~k256~docs~badge]][c~k256~docs] [![secp256k1][c~secp256k1~docs~badge]][c~secp256k1~docs] [![sha2][c~sha2~docs~badge]][c~sha2~docs]{{hi:Cryptographic primitives}}

[`k256`][c~k256~docs]↗ is a pure-Rust secp256k1 implementation, while [`secp256k1`][c~secp256k1~docs]↗ provides bindings to the Bitcoin Core library; [`sha2`][c~sha2~docs]↗ implements SHA-2. See [[cryptography | Cryptography]] for primitive selection and safe use.

## Related Topics {#related-topics .skip}

- [[cryptography | Cryptography]].
- [[blockchain | Blockchain]].
- [[security | Security]].
- [[development_tools | Development Tools]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
