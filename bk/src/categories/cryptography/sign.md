# Signature

## Sign and verify a message with a HMAC digest {#hmac}

[![ring][c-ring-badge]][c-ring] [![ring-crates.io][c-ring-crates.io-badge]][c-ring-crates.io] [![ring-github][c-ring-github-badge]][c-ring-github] [![ring-lib.rs][c-ring-lib.rs-badge]][c-ring-lib.rs]{{hi:ring}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Rand}}{{hi:Rsa}}{{hi:ECC}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Uses [`ring::hmac`][c-ring::hmac]{{hi:ring::hmac}}⮳ to creates a [`ring::signature::Signature`][c-ring::signature::Signature]{{hi:ring::signature::Signature}}⮳ of a string then verifies the signature{{hi:Signature}} is correct.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/sign/hmac.rs:example}}
```

## Compute digital signatures {#skip1}

For more algorithms, see Rust Crypto Signatures:

- ed25519{{hi:ed25519}}. Use in conjunction with the `ed25519-dalek` crate.
- ecdsa{{hi:ecdsa}}
- dsa{{hi:dsa}}

### `ed25519` {#ed25519}

[![ed25519-website][c-ed25519-website-badge]][c-ed25519-website] [![ed25519][c-ed25519-badge]][c-ed25519] [![ed25519-crates.io][c-ed25519-crates.io-badge]][c-ed25519-crates.io] [![ed25519-github][c-ed25519-github-badge]][c-ed25519-github] [![ed25519-lib.rs][c-ed25519-lib.rs-badge]][c-ed25519-lib.rs]{{hi:ed25519}}{{hi:Crypto}}{{hi:Signature}}{{hi:ECC}}{{hi:Curve25519}}{{hi:Signing}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

This crate is a support library for Edwards Digital Signature Algorithm (EdDSA) over Curve25519 (as specified in RFC 8032), providing signature type definitions and PKCS#8 private key decoding/encoding support.

It doesn't contain an implementation of Ed25519, but instead contains an ed25519::Signature type which other crates can use in conjunction with the signature::Signer and signature::Verifier traits.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/sign/ed25519.rs:example}}
```

### `ed25519-dalek` {#ed25519-dalek}

[![ed25519-dalek-website][c-ed25519_dalek-website-badge]][c-ed25519_dalek-website] [![ed25519-dalek][c-ed25519_dalek-badge]][c-ed25519_dalek] [![ed25519-dalek-crates.io][c-ed25519_dalek-crates.io-badge]][c-ed25519_dalek-crates.io] [![ed25519-dalek-github][c-ed25519_dalek-github-badge]][c-ed25519_dalek-github] [![ed25519-dalek-lib.rs][c-ed25519_dalek-lib.rs-badge]][c-ed25519_dalek-lib.rs]{{hi:ed25519-dalek}}{{hi:Cryptography}}{{hi:Ed25519}}{{hi:Signature}}{{hi:ECC}}{{hi:Curve25519}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

`ed25519-dalek` contains ed25519 EdDSA key generation, signing, and verification algorithms in pure Rust.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/sign/ed25519_dalek.rs:example}}
```

### `ecdsa` {#ecdsa}

[![ecdsa-website][c-ecdsa-website-badge]][c-ecdsa-website] [![ecdsa][c-ecdsa-badge]][c-ecdsa] [![ecdsa-crates.io][c-ecdsa-crates.io-badge]][c-ecdsa-crates.io] [![ecdsa-github][c-ecdsa-github-badge]][c-ecdsa-github] [![ecdsa-lib.rs][c-ecdsa-lib.rs-badge]][c-ecdsa-lib.rs]{{hi:ecdsa}}{{hi:Crypto}}{{hi:Secp256k1}}{{hi:Signature}}{{hi:ECC}}{{hi:NIST}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

`ecdsa` contains a pure Rust implementation of the Elliptic Curve Digital Signature Algorithm (ECDSA) as specified in FIPS 186-4 (Digital Signature Standard), providing RFC6979 deterministic signatures as well as support for added entropy.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/sign/ecdsa.rs:example}}
```

### `dsa` {#dsa}

[![dsa-website][c-dsa-website-badge]][c-dsa-website] [![dsa][c-dsa-badge]][c-dsa] [![dsa-crates.io][c-dsa-crates.io-badge]][c-dsa-crates.io] [![dsa-github][c-dsa-github-badge]][c-dsa-github] [![dsa-lib.rs][c-dsa-lib.rs-badge]][c-dsa-lib.rs]{{hi:dsa}}{{hi:Crypto}}{{hi:Signature}}{{hi:NIST}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the Digital Signature Algorithm (DSA) as specified in FIPS 186-4 (Digital Signature Standard), providing RFC6979 deterministic
signatures as well as support for added entropy.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/sign/dsa.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 write
</div>
