# Encrypt with AEAD

{{#include aead.incl.md}}

For more algorithms, see Rust Crypto AEADs: aes-gcm-siv{{hi:aes-gcm-siv}}, aes-gcm{{hi:aes-gcm}}, chacha20poly1305{{hi:chacha20poly1305}}

## `aes-gcm-siv` {#aes-gcm-siv}

[![aes-gcm-siv][c-aes_gcm_siv-badge]][c-aes_gcm_siv] [![aes-gcm-siv-crates.io][c-aes_gcm_siv-crates.io-badge]][c-aes_gcm_siv-crates.io] [![aes-gcm-siv-github][c-aes_gcm_siv-github-badge]][c-aes_gcm_siv-github] [![aes-gcm-siv-lib.rs][c-aes_gcm_siv-lib.rs-badge]][c-aes_gcm_siv-lib.rs]{{hi:aes-gcm-siv}}{{hi:Aes}}{{hi:Encryption}}{{hi:Aead}}{{hi:Siv}}{{hi:Aes-gcm}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the AES-GCM-SIV Misuse-Resistant Authenticated Encryption Cipher (RFC 8452) with optional architecture-specific hardware acceleration.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/aead/aes_gcm_siv.rs:example}}
```

## `aes-gcm` {#aes-gcm}

[![aes-gcm][c-aes_gcm-badge]][c-aes_gcm] [![aes-gcm-crates.io][c-aes_gcm-crates.io-badge]][c-aes_gcm-crates.io] [![aes-gcm-github][c-aes_gcm-github-badge]][c-aes_gcm-github] [![aes-gcm-lib.rs][c-aes_gcm-lib.rs-badge]][c-aes_gcm-lib.rs]{{hi:aes-gcm}}{{hi:Aes}}{{hi:Encryption}}{{hi:Gcm}}{{hi:Aead}}{{hi:Ghash}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the AES-GCM (Galois/Counter Mode) Authenticated Encryption with Associated Data (AEAD) Cipher with optional architecture-specific hardware acceleration.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/aead/aes_gcm.rs:example}}
```

## ChaCha20Poly1305 {#chacha20poly1305}

[![chacha20poly1305-website][c-chacha20poly1305-website-badge]][c-chacha20poly1305-website] [![chacha20poly1305][c-chacha20poly1305-badge]][c-chacha20poly1305] [![chacha20poly1305-crates.io][c-chacha20poly1305-crates.io-badge]][c-chacha20poly1305-crates.io] [![chacha20poly1305-github][c-chacha20poly1305-github-badge]][c-chacha20poly1305-github] [![chacha20poly1305-lib.rs][c-chacha20poly1305-lib.rs-badge]][c-chacha20poly1305-lib.rs]{{hi:chacha20poly1305}}{{hi:Chacha20}}{{hi:Poly1305}}{{hi:Aead}}{{hi:Xchacha20}}{{hi:Xchacha20poly1305}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the ChaCha20Poly1305 Authenticated Encryption with Additional Data Cipher (RFC 8439) with optional architecture-specific
hardware acceleration. Also contains implementations of the XChaCha20Poly1305 extended nonce variant of ChaCha20Poly1305, and the reduced-round
ChaCha8Poly1305 and ChaCha12Poly1305 lightweight variants.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/aead/chacha20poly1305.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 write
</div>
