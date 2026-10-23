<!-- TODO Reviewed ??-??-2026 -->
# Message Authentication Code

{{#include hmac.incl.md}}

## Sign and Verify a Message with a HMAC Digest {#hmac}

[![ring][c~ring~docs~badge]][c~ring~docs] [![ring~crates.io][c~ring~crates.io~badge]][c~ring~crates.io] [![ring~repo][c~ring~repo~badge]][c~ring~repo] [![ring~lib.rs][c~ring~lib.rs~badge]][c~ring~lib.rs]{{hi:ring}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Rand}}{{hi:Rsa}}{{hi:ECC}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

HMAC (Hash-based Message Authentication Code) combines a secret key with a cryptographic hash to authenticate bytes. The sender computes a tag and the receiver checks it with the same key. A valid tag shows that the data matches and that some holder of the shared key created it; it does not identify which holder. HMAC does not encrypt data.

Use HMAC when a protocol needs integrity and shared-key authentication for data that is not already protected by an authenticated encryption or authentication mechanism. Choose the hash and tag length required by the protocol; HMAC-SHA-256 is a common general-purpose choice. See [RFC 2104][rfc~2104]↗ for the construction.

## Key and Message Handling {#key-and-message-handling .skip}

- Generate keys with a cryptographically secure random source or derive them with a standard key-derivation function. Do not use a password directly as an HMAC key.
- Keep keys private, replace compromised keys, and use separate keys for separate protocols or purposes. Include protocol context and version in the authenticated message when the same format could be interpreted in more than one context.
- Define an unambiguous byte encoding for all fields before computing the tag. Authenticate every field whose value affects authorization or interpretation.
- Verify tags with the cryptographic library's verification API. Do not compare tags with ordinary equality or hand-write a comparison loop.
- Do not use HMAC as a digital signature, password-hashing function, or ad hoc key-derivation scheme. Use signatures for public verification, password-hashing algorithms for passwords, and a standard KDF for derived keys.

The example uses [`ring::hmac`][c~ring::hmac~docs]↗{{hi:ring::hmac}} to create a tag and then verify it with the library. The signing and verification parties must use the same key and the same message bytes.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/sign/hmac.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[hashing | Hashing]] describes the hash functions used by HMAC.
- [[encryption | Encryption]] and [[aead | Authenticated Encryption]] cover confidentiality and authenticated encryption.
- [[signature | Digital Signatures]] covers public-key authentication.
- [[password_hashing | Password Hashing]] explains why passwords need a dedicated, deliberately expensive algorithm.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1181)
</div>
