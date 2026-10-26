<!-- TODO Reviewed ??-??-2026 -->

# Compute Digital Signatures

{{#include signature.incl.md}}

See the overview above for what a signature proves, how to choose an algorithm, and how to store private keys. The recipes demonstrate signing and verifying data with the listed Rust crates.

## `ed25519` {#ed25519}

[![ed25519~website][c~ed25519~website~badge]][c~ed25519~website] [![ed25519][c~ed25519~docs~badge]][c~ed25519~docs] [![ed25519~crates.io][c~ed25519~crates.io~badge]][c~ed25519~crates.io] [![ed25519~repo][c~ed25519~repo~badge]][c~ed25519~repo] [![ed25519~lib.rs][c~ed25519~lib.rs~badge]][c~ed25519~lib.rs]{{hi:ed25519}}{{hi:Crypto}}{{hi:Signature}}{{hi:ECC}}{{hi:Curve25519}}{{hi:Signing}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

The [`ed25519`][c~ed25519~docs]↗{{hi:ed25519}} crate is a support library for Edwards Digital Signature Algorithm (EdDSA) over Curve25519 (as specified in RFC 8032), providing signature type definitions and PKCS#8 private key decoding/encoding support.

It doesn't contain an implementation of Ed25519, but instead contains an [`ed25519::Signature`][c~ed25519::Signature~docs]↗{{hi:ed25519::Signature}} type which other [crates][p~crates] can use in conjunction with the signature::Signer and signature::Verifier [traits][p~traits].

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/sign/ed25519.rs:example}}
```

## `ed25519-dalek` {#ed25519-dalek}

[![ed25519-dalek~website][c~ed25519-dalek~website~badge]][c~ed25519-dalek~website] [![ed25519-dalek][c~ed25519-dalek~docs~badge]][c~ed25519-dalek~docs] [![ed25519-dalek~crates.io][c~ed25519-dalek~crates.io~badge]][c~ed25519-dalek~crates.io] [![ed25519-dalek~repo][c~ed25519-dalek~repo~badge]][c~ed25519-dalek~repo] [![ed25519-dalek~lib.rs][c~ed25519-dalek~lib.rs~badge]][c~ed25519-dalek~lib.rs]{{hi:ed25519-dalek}}{{hi:Cryptography}}{{hi:Ed25519}}{{hi:Signature}}{{hi:ECC}}{{hi:Curve25519}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`ed25519-dalek`][c~ed25519-dalek~docs]↗{{hi:ed25519-dalek}} contains [`ed25519`][c~ed25519~docs]↗{{hi:ed25519}} EdDSA key generation, signing, and verification algorithms in pure Rust.

A key pair, consisting of a private key and a public key, is generated. The private key is kept secret, while the public key is shared with others.
To sign a message, a hash of the message is created. This hash is then signed using the private key, producing the digital signature.
The recipient uses the sender's public key to verify the signature. If the signature is valid, it confirms that the message is authentic and has not been altered.

[`ed25519-dalek`][c~ed25519-dalek~docs]↗{{hi:ed25519-dalek}} is compatible with [`#![no_std]`][book~rust-reference~no_std] environments, making it suitable for [embedded][p~embedded] systems or situations where the standard library is not available.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/sign/ed25519_dalek.rs:example}}
```

## `ecdsa` {#ecdsa}

[![ecdsa~website][c~ecdsa~website~badge]][c~ecdsa~website] [![ecdsa][c~ecdsa~docs~badge]][c~ecdsa~docs] [![ecdsa~crates.io][c~ecdsa~crates.io~badge]][c~ecdsa~crates.io] [![ecdsa~repo][c~ecdsa~repo~badge]][c~ecdsa~repo] [![ecdsa~lib.rs][c~ecdsa~lib.rs~badge]][c~ecdsa~lib.rs]{{hi:ecdsa}}{{hi:Crypto}}{{hi:Secp256k1}}{{hi:Signature}}{{hi:ECC}}{{hi:NIST}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

ECDSA (Elliptic Curve Digital Signature Algorithm) is a widely used digital signature scheme that leverages the properties of elliptic curve cryptography. It is a variant of the Digital Signature Algorithm (DSA) that offers the same level of security with smaller key sizes, compared to DSA and RSA.

[`ecdsa`][c~ecdsa~docs]↗{{hi:ecdsa}} contains a pure Rust implementation of ECDSA, as specified in FIPS 186-4 (Digital Signature Standard), providing RFC6979 deterministic signatures as well as support for added entropy.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/sign/ecdsa.rs:example}}
```

## `dsa` {#dsa}

[![dsa~website][c~dsa~website~badge]][c~dsa~website] [![dsa][c~dsa~docs~badge]][c~dsa~docs] [![dsa~crates.io][c~dsa~crates.io~badge]][c~dsa~crates.io] [![dsa~repo][c~dsa~repo~badge]][c~dsa~repo] [![dsa~lib.rs][c~dsa~lib.rs~badge]][c~dsa~lib.rs]{{hi:dsa}}{{hi:Crypto}}{{hi:Signature}}{{hi:NIST}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

DSA (Digital Signature Algorithm) is a Federal Information Processing Standard (FIPS) for digital signatures. It's a public-key cryptosystem used for creating digital signatures, which provide [authentication][p~authentication] and integrity for digital data. While once widely used, DSA has been superseded by algorithms like ECDSA and Ed25519 due to factors like key size and performance.

[`dsa`][c~dsa~docs]↗{{hi:dsa}} is a pure Rust implementation of the Digital Signature Algorithm (DSA) as specified in FIPS 186-4 (Digital Signature Standard), providing RFC6979 deterministic
signatures as well as support for added entropy.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/sign/dsa.rs:example}}
```

For more [algorithms][p~algorithms], see Rust Crypto Signatures:

- ed25519{{hi:ed25519}}. Use in conjunction with the [`ed25519-dalek`][c~ed25519-dalek~docs]↗{{hi:ed25519-dalek}} crate.
- ecdsa{{hi:ecdsa}}.
- dsa{{hi:dsa}}.

## Related Topics {#related-topics .skip}

- [[hashing | Cryptographic hashing]].
- [[certificates | Digital certificates]].
- [[tls | TLS]].
- [Cryptography category](index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
