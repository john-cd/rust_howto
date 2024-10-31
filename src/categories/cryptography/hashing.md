# Hashing

{{#include hashing.incl.md}}

## Calculate the SHA-256 digest of a file

[![ring][c-ring-badge]][c-ring]{{hi:ring}}  [![data-encoding][c-data_encoding-badge]][c-data_encoding]{{hi:data-encoding}}  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}

Writes some data to a file, then calculates the SHA-256{{hi:SHA-256}} [`digest::Digest`][c-digest::Digest]{{hi:digest::Digest}}⮳ of the file's contents using [`digest::Context`][c-digest::Context]{{hi:digest::Context}}⮳

```rust
{{#include ../../../deps/tests/cats/cryptography/sha_digest.rs}}
```

## Sign and verify a message with HMAC digest

[![ring][c-ring-badge]][c-ring]{{hi:ring}}  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}

Uses [`ring::hmac`][c-ring::hmac]{{hi:ring::hmac}}⮳ to creates a [`ring::signature::Signature`][c-ring::signature::Signature]{{hi:ring::signature::Signature}}⮳ of a string then verifies the signature{{hi:Signature}} is correct.

```rust
{{#include ../../../deps/tests/cats/cryptography/hmac.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO review password_hashing.md

review below from https://blessed.rs/crates

## General Purpose Hashing

For more algorithms, see Rust Crypto Hashes.

sha2{{hi:sha2}}

sha1{{hi:sha1}}

md-5{{hi:md-5}}

## AEAD Encryption

For more algorithms, see Rust Crypto AEADs.

aes-gcm-siv{{hi:aes-gcm-siv}}

aes-gcm{{hi:aes-gcm}}

chacha20poly1305{{hi:chacha20poly1305}}

## RSA

rsa{{hi:rsa}}

## Digital Signatures

For more algorithms, see Rust Crypto Signatures.

ed25519{{hi:ed25519}}
Use in conjunction with the ed25519-dalek crate.

ecdsa{{hi:ecdsa}}

dsa{{hi:dsa}}

## Certificate Formats

For more formats, see Rust Crypto Formats.

der{{hi:der}}

pem-rfc7468{{hi:pem-rfc7468}}

pkcs8{{hi:pkcs8}}

x509-cert{{hi:x509-cert}}

## TLS / SSL

rustls{{hi:rustls}}
A portable pure-rust high-level implementation of TLS. Implements TLS 1.2 and higher.

native-tls{{hi:native-tls}}
Delegates to the system TLS implementations on windows and macOS, and uses OpenSSL on linux.

## Utilities

subtle{{hi:subtle}}
Utilities for writing constant-time algorithms

zeroize{{hi:zeroize}}
Securely erase memory

</div>
