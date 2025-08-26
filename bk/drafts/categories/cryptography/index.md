# Cryptography

[![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}}

Cryptography protects data from unauthorized access, ensuring confidentiality, integrity, authenticity, and non-repudiation. Cryptography is used to secure emails, messages, and data transfers to protect them from eavesdropping. Data Integrity ensures that data has not been tampered with. Authentication means verifying the identity of users and devices.

This section covers:

- Encryption: The process of converting plaintext into ciphertext using an algorithm and a key. Only someone with the correct key can decrypt the ciphertext back into plaintext.
- Hashing: A method of transforming data into a fixed-size value, which is unique to the original data. Hashes are used for verifying data integrity.
- Digital Signatures: A cryptographic technique that uses asymmetric keys to verify the authenticity and integrity of a message or document.

It covers both Symmetric Cryptography and Asymmetric Cryptography. The former uses the same key for both encryption and decryption. It's fast and efficient but requires secure key distribution. The latter uses a pair of keys (public and private). The public key is shared openly, while the private key is kept secret. This method enhances security but is slower than symmetric cryptography.

## Encryption

Transforming plaintext into ciphertext for confidentiality.

- Types: Symmetric (shared key) vs. Asymmetric (key pair).
- Algorithms: AES, RSA.
- Purpose: Confidentiality.

{{#include encryption.incl.md}}

## AEAD (Authenticated Encryption with Associated Data)

Encryption scheme providing both confidentiality and authentication.

- Associated Data: Additional data bound to the ciphertext but not encrypted.
- Algorithms: AES-GCM, ChaCha20-Poly1305.
- Purpose: Confidentiality and authentication.

{{#include aead.incl.md}}

## Password Hashing

One-way hashing specifically to protect passwords.

- Goals: Resistance to brute-force, rainbow table attacks.
- Algorithms: bcrypt, Argon2, scrypt; salting (adding a unique random value before hashing).
- Purpose: Password protection.

{{#include password_hashing.incl.md}}

## HMAC (Hash-based Message Authentication Code)

A Message Authentication Code (MAC) is a cryptographic checksum using a shared secret key.
HMAC (Hash-based MAC) uses a hash function with a secret key.

- Purpose: Integrity and authentication (but not non-repudiation).

{{#include hmac.incl.md}}

## Digital Signatures

Cryptographic mechanism for authentication, integrity, and non-repudiation.
Digital signatures are used to sign certificates and other documents.

- Process: Signing with private key, verifying with public key.
- Algorithms: RSA, ECDSA, Ed25519.
- Purpose: Authentication, message integrity, non-repudiation.

{{#include signature.incl.md}}

## Digital Certificates (X.509)

Digital documents binding a public key to an identity.

- Components: Subject, issuer, public key, validity period, signature.
- Certificate Authorities (CAs): Trusted third parties.
- Purpose: Identity verification.

{{#include certificates.incl.md}}

## TLS (Transport Layer Security)

Cryptographic protocol for secure communication over a network.

- Handshake: Establishing a secure connection.
- Encryption: TLS uses encryption (often AEAD) to protect data in transit.
- Authentication: Verifying server (and optionally client) identity.
- Certificates: Used for authentication.
- Purpose: Secure communication.

{{#include tls.incl.md}}

## Cryptographic Utilities

{{#include cryptography_utilities.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[cryptography: review](https://github.com/john-cd/rust_howto/issues/274)
</div>
