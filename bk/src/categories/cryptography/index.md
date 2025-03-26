# Cryptography

[![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}

Cryptography protects data from unauthorized access, ensuring confidentiality, integrity, authenticity, and non-repudiation. Cryptography is used to secure emails, messages, and data transfers to protect them from eavesdropping. Data Integrity ensures that data has not been tampered with. Authentication means verifying the identity of users and devices.

This section covers

- Encryption: The process of converting plaintext into ciphertext using an algorithm and a key. Only someone with the correct key can decrypt the ciphertext back into plaintext.
- Hashing: A method of transforming data into a fixed-size value, which is unique to the original data. Hashes are used for verifying data integrity.
- Digital Signatures: A cryptographic technique that uses asymmetric keys to verify the authenticity and integrity of a message or document.

It covers both Symmetric Cryptography and Asymmetric Cryptography. The former uses the same key for both encryption and decryption. It's fast and efficient but requires secure key distribution. The latter uses a pair of keys (public and private). The public key is shared openly, while the private key is kept secret. This method enhances security but is slower than symmetric cryptography.

## Encryption

{{#include encryption.incl.md}}

## AEAD (Authenticated Encryption with Associated Data)

{{#include aead.incl.md}}

## Hashing

{{#include hashing.incl.md}}

## Password Hashing

{{#include password_hashing.incl.md}}

## HMAC (Hash-based Message Authentication Code)

{{#include hmac.incl.md}}

## Signatures

{{#include signature.incl.md}}

## Certificates

{{#include certificates.incl.md}}

## TLS (Transport Layer Security)

{{#include tls.incl.md}}

## Cryptographic Utilities

{{#include cryptography_utilities.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[cryptography: review / incorporate](https://github.com/john-cd/rust_howto/issues/274)

- ring.
- rust-crypto.

### `sodiumoxide`

[![sodiumoxide][c-sodiumoxide-badge]][c-sodiumoxide] [![sodiumoxide-crates.io][c-sodiumoxide-crates.io-badge]][c-sodiumoxide-crates.io] [![sodiumoxide-github][c-sodiumoxide-github-badge]][c-sodiumoxide-github] [![sodiumoxide-lib.rs][c-sodiumoxide-lib.rs-badge]][c-sodiumoxide-lib.rs]{{hi:sodiumoxide}}{{hi:NaCl}}{{hi:Libsodium}}{{hi:Crypto}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}

[`sodiumoxide`][c-sodiumoxide]⮳{{hi:sodiumoxide}} is a fast cryptographic library for Rust (bindings to [`libsodium`][c-libsodium]⮳{{hi:libsodium}}).

I. Core Concepts

A. Encryption:

1. Definition: Transforming plaintext into ciphertext for confidentiality.
2. Types: Symmetric (shared key) vs. Asymmetric (key pair).
3. Algorithms: Examples (AES, RSA).
4. Purpose: Confidentiality.

B. Hashing:

1. Definition: One-way function producing a fixed-size "digest."
2. Properties: Deterministic, collision-resistant, preimage-resistant.
3. Algorithms: Examples (SHA-256, BLAKE3).
4. Purpose: Data integrity.

C. Digital Signatures:

1. Definition: Cryptographic mechanism for authentication, integrity, and non-repudiation.
2. Process: Signing with private key, verifying with public key.
3. Algorithms: Examples (RSA, ECDSA, Ed25519).
4. Purpose: Authentication, integrity, non-repudiation.

D. Message Authentication Code (MAC):

1. Definition: Cryptographic checksum using a shared secret key.
2. HMAC (Hash-based MAC): Using a hash function with a secret key.
3. Purpose: Integrity and authentication (but not non-repudiation).

E. Password Hashing:

1. Definition: One-way hashing specifically for passwords.
2. Goals: Resistance to brute-force, rainbow table attacks.
3. Algorithms: Examples (bcrypt, Argon2, scrypt).
4. Salting: Adding a unique random value before hashing.
5. Purpose: Password protection.

II. Advanced Concepts and Protocols

A. Authenticated Encryption with Associated Data (AEAD):

1. Definition: Encryption scheme providing both confidentiality and authentication.
2. Associated Data: Additional data bound to the ciphertext but not encrypted.
3. Algorithms: Examples (AES-GCM, ChaCha20-Poly1305).
4. Purpose: Confidentiality and authentication.

B. Digital Certificates (X.509):

1. Definition: Digital documents binding a public key to an identity.
2. Components: Subject, issuer, public key, validity period, signature.
3. Certificate Authorities (CAs): Trusted third parties.
4. Purpose: Identity verification.

C. Transport Layer Security (TLS):

1. Definition: Cryptographic protocol for secure communication over a network.
2. Handshake: Establishing a secure connection.
3. Encryption: Protecting data in transit.
4. Authentication: Verifying server (and optionally client) identity.
5. Certificates: Used for authentication.
6. Purpose: Secure communication.

III. Relationships and Usage

A. Certificates are used in TLS to authenticate servers (and sometimes clients).
B. TLS uses encryption (often AEAD) to protect data in transit.
C. Digital signatures are used to sign certificates and other documents.
D. Hashing is used in digital signatures, HMACs, and password hashing.
E. Password hashing is used to protect user passwords.
F. HMACs are used for message authentication.
G. AEAD combines encryption and authentication.
</div>
