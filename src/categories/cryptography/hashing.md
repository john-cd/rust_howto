# Hashing

{{#include hashing.incl.md}}

## Calculate the SHA-256 digest of a file

[![ring][c-ring-badge]][c-ring]{{hi:ring}}  [![data_encoding][c-data_encoding-badge]][c-data_encoding]{{hi:data_encoding}}  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}

Writes some data to a file, then calculates the SHA-256{{hi:SHA-256}} [`digest::Digest`][c-digest::Digest]{{hi:digest::Digest}}⮳ of the file's contents using [`digest::Context`][c-digest::Context]{{hi:digest::Context}}⮳

```rust
{{#include ../../../deps/tests/sha-digest.rs}}
```

## Sign and verify a message with HMAC digest

[![ring][c-ring-badge]][c-ring]{{hi:ring}}  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}

Uses [`ring::hmac`][c-ring::hmac]{{hi:ring::hmac}}⮳ to creates a [`ring::signature::Signature`][c-ring::signature::Signature]{{hi:ring::signature::Signature}}⮳ of a string then verifies the signature{{hi:Signature}} is correct.

```rust
{{#include ../../../deps/tests/hmac.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO review password_hashing.md

review below from https://blessed.rs/crates

## General Purpose Hashing

For more algorithms, see Rust Crypto Hashes.

sha2

sha1

md-5

## AEAD Encryption

For more algorithms, see Rust Crypto AEADs.

aes-gcm-siv

aes-gcm

chacha20poly1305

## RSA

rsa

## Digital Signatures

For more algorithms, see Rust Crypto Signatures.

ed25519
Use in conjunction with the ed25519-dalek crate.

ecdsa

dsa

## Certificate Formats

For more formats, see Rust Crypto Formats.

der

pem-rfc7468

pkcs8

x509-cert

## TLS / SSL

rustls
A portable pure-rust high-level implementation of TLS. Implements TLS 1.2 and higher.

native-tls
Delegates to the system TLS implementations on windows and macOS, and uses OpenSSL on linux.

## Utilities

subtle
Utilities for writing constant-time algorithms

zeroize
Securely erase memory

</div>
