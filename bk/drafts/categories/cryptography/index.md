<!-- TODO Reviewed ??-??-2026 -->

# Cryptography

[![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}}

Cryptography provides tools for confidentiality, integrity, and authentication. A cryptographic hash can detect changes when its expected value is trusted, but it does not authenticate a sender by itself. A digital signature can prove that a message was signed with a particular private key; linking that key to a person or organization requires a separate trust model.

Prefer high-level, well-reviewed APIs and established protocols; do not implement cryptographic primitives yourself. Check the security and maintenance status of the specific crate and algorithm you choose, and review [RustSec advisories][rustsec~advisories]. The [RustCrypto project][rustcrypto~github] provides focused crates for many common algorithms and formats; `ring` and `aws-lc-rs` are other options, and `rustls` is a TLS implementation. `sodiumoxide` is deprecated; for its style of API, consider [`dryoc`][c~dryoc~docs], or use the relevant RustCrypto crates directly.

## Encryption

Use authenticated encryption with a shared key for most application data. Public-key cryptography is usually used to establish or wrap keys rather than encrypting large messages directly. See [Encryption](encryption.md) for algorithms and APIs such as `aes-gcm`, `chacha20poly1305`, and legacy `rsa`.

## AEAD (Authenticated Encryption with Associated Data)

Authenticated encryption with associated data (AEAD) protects a message's confidentiality and integrity while allowing unencrypted metadata to be authenticated. See [AEAD](aead.md) for `aes-gcm`, `aes-gcm-siv`, and `chacha20poly1305` recipes; never reuse a nonce with the same key.

## Password Hashing

Store a slow, salted password hash rather than a plaintext or fast general-purpose hash. See [Password Hashing](password_hashing.md) for Argon2id with the `argon2` and `password-hash` APIs, plus `scrypt`, `bcrypt`, and PBKDF2 compatibility options.

## HMAC (Hash-based Message Authentication Code)

Use a message authentication code when parties share a secret and need integrity and authentication. See [HMAC](hmac.md) for the `hmac` and `ring` APIs. Because both parties hold the key, HMAC does not provide public verifiability.

## Digital Signatures

Sign with a private key and verify with the corresponding public key. See [Digital Signatures](signature.md) for Ed25519 (`ed25519-dalek`), P-256 ECDSA, RSA compatibility, and the `signature` traits. Protect private keys and choose an identity and key-distribution scheme separately.

## Digital Certificates (X.509)

X.509 certificates bind a public key to claims that a verifier evaluates under a trust policy. See [Digital Certificates](certificates.md) for DER, PEM, PKCS#8, and X.509 formats using crates such as `x509-cert`, `der`, and `pem-rfc7468`.

## TLS (Transport Layer Security)

Use a maintained TLS implementation instead of assembling a protocol from primitives. See [TLS](tls.md) for `rustls` with `ring` or `aws-lc-rs` providers, and `native-tls` when the operating system's TLS stack is needed.

## Cryptographic Utilities

See [Cryptographic Utilities](cryptography_utilities.md) for operating-system randomness (`getrandom`), constant-time helpers (`subtle`), and clearing sensitive values (`zeroize`).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
