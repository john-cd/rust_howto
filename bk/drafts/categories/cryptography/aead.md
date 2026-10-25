<!-- TODO Reviewed ??-??-2026 -->

# Encrypt with AEAD

{{#include aead.incl.md}}

Authenticated encryption combines confidentiality with an integrity check. AEAD also authenticates associated data (AAD) without encrypting it. A protocol can use AAD for visible context such as a record header, version, or sequence number, so a receiver can detect tampering with that context as well as with the ciphertext.

An AEAD operation takes a key, nonce, plaintext, and AAD. Encryption returns ciphertext and an authentication tag (often packaged together); decryption succeeds only when the computed tag matches. Changing the ciphertext, AAD, key, or nonce will normally make verification fail. Do not use decrypted bytes unless the library reports successful authentication.

Typical uses include protecting records in a network protocol, encrypting file or database contents, and authenticating visible metadata such as a record type or version. In each case, the application must define how keys and nonces are managed and how AAD is encoded. The examples below demonstrate library calls; the fixed nonce in the AES examples is for one-message demonstrations only and must not be reused with the same key.

## Safe AEAD Use {#safe-use .skip}

- Follow the algorithm and protocol's nonce requirements. For common AES-GCM and ChaCha20-Poly1305 uses, every encryption under one key must have a distinct nonce. A nonce is not secret, but it must be stored or transmitted so the receiver can use the same value.
- Plan nonce allocation across processes, devices, and restarts that share a key. A per-key counter or protocol-defined sequence is easier to audit than ad hoc state. Random nonces can be appropriate when the algorithm's usage limits and collision probability are accounted for, but they do not guarantee strict uniqueness.
- Authenticate all context that affects how a message is interpreted. Encode multiple AAD fields unambiguously, for example with fixed-width fields or explicit lengths.
- Treat authentication failure as a normal error path. Do not return partial plaintext or reveal detailed tag-comparison results to an attacker.
- Use a maintained AEAD implementation and let it handle encryption and tag verification. Do not combine a cipher and a MAC yourself.

AES-GCM-SIV reduces the harm of accidental nonce reuse, but it is a distinct algorithm and is not wire-compatible with AES-GCM. Misuse resistance does not remove the need to follow the algorithm's limits and protocol rules.

AEAD proves that a message was created by someone holding the symmetric key. It does not distinguish among parties that share that key or provide public non-repudiation. Use a digital signature when third parties must verify the sender.

AEAD is commonly used in secure communication protocols such as [TLS][p~tls] (Transport Layer Security). For the AEAD interface and nonce guidance, see [RFC 5116][rfc~5116]↗. For ChaCha20-Poly1305, see [RFC 8439][rfc~8439]↗. Also see:

- ["Towards a safer footgun"][codahale~website]↗.
- [AES-GCM-SIV][imperialviolet~website]↗.

## `aes-gcm-siv` {#aes-gcm-siv}

[![aes-gcm-siv][c~aes-gcm-siv~docs~badge]][c~aes-gcm-siv~docs] [![aes-gcm-siv~crates.io][c~aes-gcm-siv~crates.io~badge]][c~aes-gcm-siv~crates.io] [![aes-gcm-siv~repo][c~aes-gcm-siv~repo~badge]][c~aes-gcm-siv~repo] [![aes-gcm-siv~lib.rs][c~aes-gcm-siv~lib.rs~badge]][c~aes-gcm-siv~lib.rs]{{hi:aes-gcm-siv}}{{hi:Aes}}{{hi:Encryption}}{{hi:AEAD}}{{hi:Siv}}{{hi:Aes-gcm}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`aes-gcm-siv`][c~aes-gcm-siv~docs]↗{{hi:aes-gcm-siv}} is a pure Rust implementation of the AES-GCM-SIV Misuse-Resistant Authenticated Encryption Cipher (RFC 8452) with optional architecture-specific hardware acceleration.

AES-GCM-SIV (Advanced [Encryption][p~encryption] Standard - Galois/Counter Mode - Synthetic Initialization Vector) is a mode of operation for AES that provides nonce misuse resistance. This means it can securely handle situations where a cryptographic nonce (a number used only once) might be reused accidentally.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/aead/aes_gcm_siv.rs:example}}
```

## `aes-gcm` {#aes-gcm}

[![aes-gcm][c~aes-gcm~docs~badge]][c~aes-gcm~docs] [![aes-gcm~crates.io][c~aes-gcm~crates.io~badge]][c~aes-gcm~crates.io] [![aes-gcm~repo][c~aes-gcm~repo~badge]][c~aes-gcm~repo] [![aes-gcm~lib.rs][c~aes-gcm~lib.rs~badge]][c~aes-gcm~lib.rs]{{hi:aes-gcm}}{{hi:Aes}}{{hi:Encryption}}{{hi:GCM}}{{hi:AEAD}}{{hi:Ghash}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`aes-gcm`][c~aes-gcm~docs]↗{{hi:aes-gcm}} is a pure Rust implementation of the AES-GCM (Galois/Counter Mode) Authenticated Encryption with Associated Data (AEAD) Cipher with optional architecture-specific hardware acceleration.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/aead/aes_gcm.rs:example}}
```

## ChaCha20Poly1305 {#chacha20poly1305}

[![chacha20poly1305~website][c~chacha20poly1305~website~badge]][c~chacha20poly1305~website] [![chacha20poly1305][c~chacha20poly1305~docs~badge]][c~chacha20poly1305~docs] [![chacha20poly1305~crates.io][c~chacha20poly1305~crates.io~badge]][c~chacha20poly1305~crates.io] [![chacha20poly1305~repo][c~chacha20poly1305~repo~badge]][c~chacha20poly1305~repo] [![chacha20poly1305~lib.rs][c~chacha20poly1305~lib.rs~badge]][c~chacha20poly1305~lib.rs]{{hi:chacha20poly1305}}{{hi:Chacha20}}{{hi:Poly1305}}{{hi:AEAD}}{{hi:Xchacha20}}{{hi:Xchacha20poly1305}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`chacha20poly1305`][c~chacha20poly1305~docs]↗{{hi:chacha20poly1305}} is a pure Rust implementation of the ChaCha20Poly1305 Authenticated Encryption with Additional Data Cipher (RFC 8439) with optional architecture-specific hardware acceleration. Also contains implementations of the XChaCha20Poly1305 extended nonce variant of ChaCha20Poly1305, and the reduced-round ChaCha8Poly1305 and ChaCha12Poly1305 lightweight variants.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/aead/chacha20poly1305.rs:example}}
```

For more [algorithms][p~algorithms], see Rust Crypto AEADs: aes-gcm-siv{{hi:aes-gcm-siv}}, aes-gcm{{hi:aes-gcm}}, chacha20poly1305{{hi:chacha20poly1305}}.

## Related Topics {#related-topics .skip}

- [[encryption | Encryption]].
- [[hashing | Hashing]].
- [[hmac | HMAC]].
- [[password_hashing | Password hashing]].
- [[signature | Digital signatures]].
- [Cryptography category](index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
