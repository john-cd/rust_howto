# Hashing

{{#include hashing.incl.md}}

## Calculate the SHA-256 digest of a file {#sha256}

[![ring][c-ring-badge]][c-ring] [![ring-crates.io][c-ring-crates.io-badge]][c-ring-crates.io] [![ring-github][c-ring-github-badge]][c-ring-github] [![ring-lib.rs][c-ring-lib.rs-badge]][c-ring-lib.rs]{{hi:ring}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Rand}}{{hi:Rsa}}{{hi:Ecc}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[![data-encoding][c-data_encoding-badge]][c-data_encoding] [![data-encoding-crates.io][c-data_encoding-crates.io-badge]][c-data_encoding-crates.io] [![data-encoding-github][c-data_encoding-github-badge]][c-data_encoding-github] [![data-encoding-lib.rs][c-data_encoding-lib.rs-badge]][c-data_encoding-lib.rs]{{hi:data-encoding}}{{hi:Hex}}{{hi:No_std}}{{hi:Base64}}{{hi:Base32}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Writes some data to a file, then calculates the SHA-256{{hi:SHA-256}} [`digest::Digest`][c-digest::Digest]{{hi:digest::Digest}}⮳ of the file's contents using [`digest::Context`][c-digest::Context]{{hi:digest::Context}}⮳

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/sha_digest.rs:example}}
```

## Sign and verify a message with a HMAC digest {#hmac}

[![ring][c-ring-badge]][c-ring] [![ring-crates.io][c-ring-crates.io-badge]][c-ring-crates.io] [![ring-github][c-ring-github-badge]][c-ring-github] [![ring-lib.rs][c-ring-lib.rs-badge]][c-ring-lib.rs]{{hi:ring}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Rand}}{{hi:Rsa}}{{hi:Ecc}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Uses [`ring::hmac`][c-ring::hmac]{{hi:ring::hmac}}⮳ to creates a [`ring::signature::Signature`][c-ring::signature::Signature]{{hi:ring::signature::Signature}}⮳ of a string then verifies the signature{{hi:Signature}} is correct.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/hmac.rs:example}}
```

## Use general-purpose hashing algorithms {#general-purpose-hashing}

For more algorithms, see Rust Crypto Hashes: sha2{{hi:sha2}}, sha1{{hi:sha1}}, md-5{{hi:md-5}}

[![sha2][c-sha2-badge]][c-sha2] [![sha2-crates.io][c-sha2-crates.io-badge]][c-sha2-crates.io] [![sha2-github][c-sha2-github-badge]][c-sha2-github] [![sha2-lib.rs][c-sha2-lib.rs-badge]][c-sha2-lib.rs]{{hi:sha2}}{{hi:Hash}}{{hi:Crypto}}{{hi:sha2}}{{hi:Digest}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the SHA-2 hash function family, including SHA-224, SHA-256, SHA-384, and SHA-512.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/sha2.rs:example}}
```

[![sha1][c-sha1-badge]][c-sha1] [![sha1-crates.io][c-sha1-crates.io-badge]][c-sha1-crates.io] [![sha1-github][c-sha1-github-badge]][c-sha1-github] [![sha1-lib.rs][c-sha1-lib.rs-badge]][c-sha1-lib.rs]{{hi:sha1}}{{hi:Hash}}{{hi:sha1}}{{hi:Crypto}}{{hi:Digest}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

SHA-1 hash function

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/sha1.rs:example}}
```

[![md-5][c-md_5-badge]][c-md_5] [![md-5-crates.io][c-md_5-crates.io-badge]][c-md_5-crates.io] [![md-5-github][c-md_5-github-badge]][c-md_5-github] [![md-5-lib.rs][c-md_5-lib.rs-badge]][c-md_5-lib.rs]{{hi:md-5}}{{hi:Hash}}{{hi:Md5}}{{hi:Crypto}}{{hi:Digest}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

MD5 hash function

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/md_5.rs:example}}
```

## Encrypt with AEAD {#aead-encryption}

For more algorithms, see Rust Crypto AEADs: aes-gcm-siv{{hi:aes-gcm-siv}}, aes-gcm{{hi:aes-gcm}}, chacha20poly1305{{hi:chacha20poly1305}}

[![aes-gcm-siv][c-aes_gcm_siv-badge]][c-aes_gcm_siv] [![aes-gcm-siv-crates.io][c-aes_gcm_siv-crates.io-badge]][c-aes_gcm_siv-crates.io] [![aes-gcm-siv-github][c-aes_gcm_siv-github-badge]][c-aes_gcm_siv-github] [![aes-gcm-siv-lib.rs][c-aes_gcm_siv-lib.rs-badge]][c-aes_gcm_siv-lib.rs]{{hi:aes-gcm-siv}}{{hi:Aes}}{{hi:Encryption}}{{hi:Aead}}{{hi:Siv}}{{hi:Aes-gcm}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the AES-GCM-SIV Misuse-Resistant Authenticated Encryption Cipher (RFC 8452) with optional architecture-specific hardware acceleration.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/aes_gcm_siv.rs:example}}
```

[![aes-gcm][c-aes_gcm-badge]][c-aes_gcm] [![aes-gcm-crates.io][c-aes_gcm-crates.io-badge]][c-aes_gcm-crates.io] [![aes-gcm-github][c-aes_gcm-github-badge]][c-aes_gcm-github] [![aes-gcm-lib.rs][c-aes_gcm-lib.rs-badge]][c-aes_gcm-lib.rs]{{hi:aes-gcm}}{{hi:Aes}}{{hi:Encryption}}{{hi:Gcm}}{{hi:Aead}}{{hi:Ghash}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the AES-GCM (Galois/Counter Mode) Authenticated Encryption with Associated Data (AEAD) Cipher with optional architecture-specific hardware acceleration.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/aes_gcm.rs:example}}
```

## Use the RSA algorithm {#rsa}

[![rsa][c-rsa-badge]][c-rsa] [![rsa-crates.io][c-rsa-crates.io-badge]][c-rsa-crates.io] [![rsa-github][c-rsa-github-badge]][c-rsa-github] [![rsa-lib.rs][c-rsa-lib.rs-badge]][c-rsa-lib.rs]{{hi:rsa}}{{hi:Crypto}}{{hi:Security}}{{hi:Encryption}}{{hi:rsa}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}

Pure Rust RSA implementation.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/rsa.rs:example}}
```

## Compute digital signatures {#digital-signatures}

For more algorithms, see Rust Crypto Signatures:

- ed25519{{hi:ed25519}}. Use in conjunction with the `ed25519-dalek` crate.
- ecdsa{{hi:ecdsa}}
- dsa{{hi:dsa}}

[![ed25519-website][c-ed25519-website-badge]][c-ed25519-website] [![ed25519][c-ed25519-badge]][c-ed25519] [![ed25519-crates.io][c-ed25519-crates.io-badge]][c-ed25519-crates.io] [![ed25519-github][c-ed25519-github-badge]][c-ed25519-github] [![ed25519-lib.rs][c-ed25519-lib.rs-badge]][c-ed25519-lib.rs]{{hi:ed25519}}{{hi:Crypto}}{{hi:Signature}}{{hi:Ecc}}{{hi:Curve25519}}{{hi:Signing}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Edwards Digital Signature Algorithm (EdDSA) over Curve25519 (as specified in RFC 8032) support library providing signature type definitions and PKCS#8 private key decoding/encoding support.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/ed25519.rs:example}}
```

[![ed25519-dalek-website][c-ed25519_dalek-website-badge]][c-ed25519_dalek-website] [![ed25519-dalek][c-ed25519_dalek-badge]][c-ed25519_dalek] [![ed25519-dalek-crates.io][c-ed25519_dalek-crates.io-badge]][c-ed25519_dalek-crates.io] [![ed25519-dalek-github][c-ed25519_dalek-github-badge]][c-ed25519_dalek-github] [![ed25519-dalek-lib.rs][c-ed25519_dalek-lib.rs-badge]][c-ed25519_dalek-lib.rs]{{hi:ed25519-dalek}}{{hi:Cryptography}}{{hi:Ed25519}}{{hi:Signature}}{{hi:Ecc}}{{hi:Curve25519}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Fast and efficient ed25519 EdDSA key generations, signing, and verification in pure Rust.

[![ecdsa-website][c-ecdsa-website-badge]][c-ecdsa-website] [![ecdsa][c-ecdsa-badge]][c-ecdsa] [![ecdsa-crates.io][c-ecdsa-crates.io-badge]][c-ecdsa-crates.io] [![ecdsa-github][c-ecdsa-github-badge]][c-ecdsa-github] [![ecdsa-lib.rs][c-ecdsa-lib.rs-badge]][c-ecdsa-lib.rs]{{hi:ecdsa}}{{hi:Crypto}}{{hi:Secp256k1}}{{hi:Signature}}{{hi:Ecc}}{{hi:Nist}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the Elliptic Curve Digital Signature Algorithm (ECDSA) as specified in FIPS 186-4 (Digital Signature Standard), providing RFC6979 deterministic signatures as well as support for added entropy.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/ecdsa.rs:example}}
```

[![dsa-website][c-dsa-website-badge]][c-dsa-website] [![dsa][c-dsa-badge]][c-dsa] [![dsa-crates.io][c-dsa-crates.io-badge]][c-dsa-crates.io] [![dsa-github][c-dsa-github-badge]][c-dsa-github] [![dsa-lib.rs][c-dsa-lib.rs-badge]][c-dsa-lib.rs]{{hi:dsa}}{{hi:Crypto}}{{hi:Signature}}{{hi:Nist}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the Digital Signature Algorithm (DSA) as specified in FIPS 186-4 (Digital Signature Standard), providing RFC6979 deterministic
signatures as well as support for added entropy.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/dsa.rs:example}}
```

## Create certificates {#certificates}

For more formats, see Rust Crypto Formats.

- der{{hi:der}}
- pem-rfc7468{{hi:pem-rfc7468}}
- pkcs8{{hi:pkcs8}}
- x509-cert{{hi:x509-cert}}

[![der-website][c-der-website-badge]][c-der-website] [![der][c-der-badge]][c-der] [![der-crates.io][c-der-crates.io-badge]][c-der-crates.io] [![der-github][c-der-github-badge]][c-der-github] [![der-lib.rs][c-der-lib.rs-badge]][c-der-lib.rs]{{hi:der}}{{hi:Crypto}}{{hi:Asn1}}{{hi:Pkcs}}{{hi:Itu}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust embedded-friendly implementation of the Distinguished Encoding Rules (DER) for Abstract Syntax Notation One (ASN.1) as described in ITU X.690 with full support for heapless `no_std` targets.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/der.rs:example}}
```

[![pem-rfc7468-website][c-pem_rfc7468-website-badge]][c-pem_rfc7468-website] [![pem-rfc7468][c-pem_rfc7468-badge]][c-pem_rfc7468] [![pem-rfc7468-crates.io][c-pem_rfc7468-crates.io-badge]][c-pem_rfc7468-crates.io] [![pem-rfc7468-github][c-pem_rfc7468-github-badge]][c-pem_rfc7468-github] [![pem-rfc7468-lib.rs][c-pem_rfc7468-lib.rs-badge]][c-pem_rfc7468-lib.rs]{{hi:pem-rfc7468}}{{hi:Crypto}}{{hi:Key}}{{hi:Rsa}}{{hi:Pem}}{{hi:Pkcs}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

PEM Encoding (RFC 7468) for PKIX, PKCS, and CMS Structures, implementing a strict subset of the original Privacy-Enhanced Mail encoding intended
specifically for use with cryptographic keys, certificates, and other messages. Provides a no_std-friendly, constant-time implementation suitable for use with cryptographic private keys.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/pem_rfc7468.rs:example}}
```

[![pkcs8-website][c-pkcs8-website-badge]][c-pkcs8-website] [![pkcs8][c-pkcs8-badge]][c-pkcs8] [![pkcs8-crates.io][c-pkcs8-crates.io-badge]][c-pkcs8-crates.io] [![pkcs8-github][c-pkcs8-github-badge]][c-pkcs8-github] [![pkcs8-lib.rs][c-pkcs8-lib.rs-badge]][c-pkcs8-lib.rs]{{hi:pkcs8}}{{hi:Crypto}}{{hi:Key}}{{hi:Pkcs}}{{hi:Private}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of Public-Key Cryptography Standards (PKCS) #8: Private-Key Information Syntax Specification (RFC 5208), with additional
support for PKCS#8v2 asymmetric key packages (RFC 5958).

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/pkcs8.rs:example}}
```

[![x509-cert-website][c-x509_cert-website-badge]][c-x509_cert-website] [![x509-cert][c-x509_cert-badge]][c-x509_cert] [![x509-cert-crates.io][c-x509_cert-crates.io-badge]][c-x509_cert-crates.io] [![x509-cert-github][c-x509_cert-github-badge]][c-x509_cert-github] [![x509-cert-lib.rs][c-x509_cert-lib.rs-badge]][c-x509_cert-lib.rs]{{hi:x509-cert}}{{hi:Crypto}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the X.509 Public Key Infrastructure Certificate format as described in RFC 5280.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/x509_cert.rs:example}}
```

## Use TLS / SSL {#tls-ssl}

[![rustls][c-rustls-badge]][c-rustls] [![rustls-crates.io][c-rustls-crates.io-badge]][c-rustls-crates.io] [![rustls-github][c-rustls-github-badge]][c-rustls-github] [![rustls-lib.rs][c-rustls-lib.rs-badge]][c-rustls-lib.rs]{{hi:rustls}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}

Rustls is a portable pure-rust high-level implementation of TLS. Implements TLS 1.2 and higher.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/rustls.rs:example}}
```

[![native-tls][c-native_tls-badge]][c-native_tls] [![native-tls-crates.io][c-native_tls-crates.io-badge]][c-native_tls-crates.io] [![native-tls-github][c-native_tls-github-badge]][c-native_tls-github] [![native-tls-lib.rs][c-native_tls-lib.rs-badge]][c-native_tls-lib.rs]{{hi:native-tls}}

A wrapper over a platform's native TLS implementation. Delegates to the system TLS implementations on windows and macOS, and uses OpenSSL on Linux.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/native_tls.rs:example}}
```

## Utilities {#utilities}

### `subtle` {#subtle}

[![subtle-website][c-subtle-website-badge]][c-subtle-website] [![subtle][c-subtle-badge]][c-subtle] [![subtle-crates.io][c-subtle-crates.io-badge]][c-subtle-crates.io] [![subtle-github][c-subtle-github-badge]][c-subtle-github] [![subtle-lib.rs][c-subtle-lib.rs-badge]][c-subtle-lib.rs]{{hi:subtle}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Utilities}}{{hi:Constant-time}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure-Rust traits and utilities for constant-time cryptographic implementations.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/subtle.rs:example}}
```

### `zeroize` {#zeroize}

[![zeroize][c-zeroize-badge]][c-zeroize] [![zeroize-crates.io][c-zeroize-crates.io-badge]][c-zeroize-crates.io] [![zeroize-github][c-zeroize-github-badge]][c-zeroize-github] [![zeroize-lib.rs][c-zeroize-lib.rs-badge]][c-zeroize-lib.rs]{{hi:zeroize}}{{hi:Memory}}{{hi:Volatile}}{{hi:Secure}}{{hi:Memset}}{{hi:Zero}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}} [![cat-os][cat-os-badge]][cat-os]{{hi:Operating systems}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Securely clear secrets from memory with a simple trait built on stable Rust primitives which guarantee memory is zeroed using an operation that will not bee optimized away by the compiler. Uses a portable pure Rust implementation that works everywhere, even WASM.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/zeroize.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[hashing: improve (P1)](https://github.com/john-cd/rust_howto/issues/273)  review [blessed.rs][blessed-rs-website]
</div>
