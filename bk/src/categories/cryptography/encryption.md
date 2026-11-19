<!-- TODO Reviewed 11-17-2026. -->

# Encryption

{{#include encryption.incl.md}}

Use symmetric authenticated encryption to protect application data. The shared key encrypts and authenticates a message; the nonce must follow the algorithm's requirements and must not be reused with the same key for AES-GCM or ChaCha20-Poly1305. Public-key algorithms are generally used to establish or wrap keys rather than encrypt large messages directly. Authentication failure must be treated as an error, and unauthenticated plaintext must not be used.

The AES-GCM and ChaCha20-Poly1305 examples use pure Rust implementations, so
compiling, testing, and linting them requires no additional native crypto
library. Running the key-generation examples requires the operating system's
secure random source; they do not need an external service.

## AES-256-GCM {#aes-256-gcm}

[`aes-gcm`][c~aes-gcm~docs]↗ provides AES-GCM authenticated encryption. The example generates a key and nonce from the operating system's random source, encrypts and decrypts a message, and tests that modified ciphertext is rejected.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p cryptography --example encryption`.

```rust,editable,noplayground
{{#include ../../../crates/cats/cryptography/examples/encryption/aes_gcm.rs:example}}
```

## ChaCha20-Poly1305 {#chacha20-poly1305}

[`chacha20poly1305`][c~chacha20poly1305~docs]↗ provides ChaCha20-Poly1305 authenticated encryption. The example verifies a round trip and confirms that modified tags and associated data fail authentication.

```rust,editable,noplayground
{{#include ../../../crates/cats/cryptography/examples/encryption/chacha20poly1305.rs:example}}
```

## RSA PKCS#1 v1.5 Compatibility Example {#rsa}

{{#badges rsa cryptography=Cryptography}}{{hi:Crypto}}{{hi:Security}}{{hi:Encryption}}{{hi:rsa}}

`rsa` implements RSA public-key operations. RSA encryption is limited to short messages, so protocols usually use it only to protect a symmetric key and use AEAD for the data. This example demonstrates PKCS#1 v1.5 padding for compatibility; it is not a recommendation for a new protocol. The direct `rsa` 0.10 dependency includes the fix for [RUSTSEC-2023-0071](https://rustsec.org/advisories/RUSTSEC-2023-0071.html), but that does not make this legacy padding scheme suitable for a new protocol. Do not expose decryption errors or timing to an attacker, because that can create a padding oracle. If a protocol requires RSA encryption, use a maintained library and its RSA-OAEP support; prefer an established protocol over designing a hybrid scheme yourself.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p cryptography --example rsa`.

```rust,editable,noplayground
{{#include ../../../crates/cats/cryptography/examples/rsa.rs:example}}
```

## Deprecated `sodiumoxide` API

{{#badges sodiumoxide cryptography=Cryptography}}{{hi:NaCl}}{{hi:Libsodium}}{{hi:Crypto}}

`sodiumoxide` is deprecated. Do not start new projects with it. When maintaining legacy code, evaluate a maintained alternative such as [`dryoc`][c~dryoc~docs]↗ or the relevant RustCrypto crates; check API compatibility and security status before migrating.

## Related Topics {#related-topics .skip}

- [AEAD](aead.md).
- [Password Hashing](password_hashing.md).
- [Digital Signatures](signature.md).
- [Cryptography category](index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
