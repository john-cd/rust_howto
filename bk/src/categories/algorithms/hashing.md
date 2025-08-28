# Hashing

{{#include hashing.incl.md}}

Hashing is a process that converts input data of any size into a "digest", a fixed-size string of characters, typically a sequence of numbers and letters, also known as a hash value or hash code. Hash functions are designed to be fast, deterministic (the same input will always produce the same output), and produce a unique hash for distinct inputs (lack of collisions). One key feature of hashing is that it is a _one-way_ function; you can't reverse-engineer the original input from the hash value.

Hashes are widely used in various applications such as:

- Hash Maps: Efficient data retrieval.
- Data Integrity: Ensuring that data has not been altered by comparing the hash value of the original data with the hash value of the received data.
- Password Storage: Storing hashed versions of passwords rather than plain text to enhance security.
- Digital Signatures: Verifying the authenticity and integrity of messages or documents.

Use the following crates for general-purpose hashing and cryptographic hashing:
- `blake3` and `sha2` for general-purpose hashing
- `ahash` for use in in-memory hashmaps.
- `rustc-hash` for fast, non-cryptographic hashing.
- `murmur3` is a non-cryptographic hash function suitable for general hash-based lookup.
- `fnv` is the Fowler–Noll–Vo hash function that is more efficient for smaller hash keys.
- `hashbrown` is a Rust port of Google's high-performance SwissTable hash map, adapted to make it a drop-in replacement for Rust's standard `HashMap` and `HashSet` types.

You may also use `crc` or `crc32fast` for CRC checksums.

TODO distinguish between general-purpose / fast / OOS resistant / crypto hashing.

[](https://github.com/RustCrypto/hashes#rustcrypto-hashes)

[Cryptographic hash function](https://en.wikipedia.org/wiki/Cryptographic_hash_function)

## Calculate the SHA-256 Digest of a File {#sha256}

[![ring][c~ring~docs~badge]][c~ring~docs] [![ring~crates.io][c~ring~crates.io~badge]][c~ring~crates.io] [![ring~repo][c~ring~repo~badge]][c~ring~repo] [![ring~lib.rs][c~ring~lib.rs~badge]][c~ring~lib.rs]{{hi:ring}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Rand}}{{hi:Rsa}}{{hi:ECC}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[![data-encoding][c~data-encoding~docs~badge]][c~data-encoding~docs] [![data-encoding~crates.io][c~data-encoding~crates.io~badge]][c~data-encoding~crates.io] [![data-encoding~repo][c~data-encoding~repo~badge]][c~data-encoding~repo] [![data-encoding~lib.rs][c~data-encoding~lib.rs~badge]][c~data-encoding~lib.rs]{{hi:data-encoding}}{{hi:Hex}}{{hi:No_std}}{{hi:Base64}}{{hi:Base32}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

SHA-256 (Secure Hash Algorithm 256-bit) is part of the SHA-2 family of cryptographic hash functions. It produces a fixed-size 256-bit hash value (64 characters) from input data of any size. SHA-256 is widely used in applications such as digital signatures, certificate generation, and data integrity verification.

Writes some data to a file, then calculates the SHA-256{{hi:SHA-256}} [`digest::Digest`][c~digest::Digest~docs]↗{{hi:digest::Digest}} of the file's contents using [`digest::Context`][c~digest::Context~docs]↗{{hi:digest::Context}}.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/hashing/sha_digest.rs:example}}
```

## Use General-purpose Hashing Algorithms {#general-purpose-hashing-algorithms}

For more [[algorithms | algorithms]], see Rust Crypto Hashes: sha2{{hi:sha2}}, sha1{{hi:sha1}}, md-5{{hi:md-5}}

### Hash with `blake3` {#blake3}

[![blake3][c~blake3~docs~badge]][c~blake3~docs] [![blake3~crates.io][c~blake3~crates.io~badge]][c~blake3~crates.io] [![blake3~repo][c~blake3~repo~badge]][c~blake3~repo] [![blake3~lib.rs][c~blake3~lib.rs~badge]][c~blake3~lib.rs]{{hi:blake3}}

[`blake3`][c~blake3~docs]↗{{hi:blake3}} implements the BLAKE3 hash function. BLAKE3 is a cryptographic hash function that is faster than MD5, SHA-1, SHA-2, and SHA-3, yet is at least as secure as the latest standard SHA-3. It is designed to take advantage of parallel processing capabilities. BLAKE3 can produce hashes of arbitrary length, from short digests to longer ones. This is useful for various applications, including key derivation and password hashing. BLAKE3 allows for incremental hashing, i.e. updating the hash state with new data without recomputing the entire hash. This is useful for streaming data or situations where the input is received in chunks.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/hashing/blake3.rs:example}}
```

### Hash with `sha2` {#sha2}

[![sha2][c~sha2~docs~badge]][c~sha2~docs] [![sha2~crates.io][c~sha2~crates.io~badge]][c~sha2~crates.io] [![sha2~repo][c~sha2~repo~badge]][c~sha2~repo] [![sha2~lib.rs][c~sha2~lib.rs~badge]][c~sha2~lib.rs]{{hi:sha2}}{{hi:Hash}}{{hi:Crypto}}{{hi:sha2}}{{hi:Digest}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

SHA-2 (Secure Hash Algorithm 2) is a family of cryptographic hash functions designed by the National Security Agency (NSA) and standardized by NIST.

[`sha2`][c~sha2~docs]↗{{hi:sha2}} is a pure Rust implementation of the SHA-2 hash function family, including SHA-224, SHA-256, SHA-384, and SHA-512. SHA-256 is the most commonly used variant.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/hashing/sha2.rs:example}}
```

### `foldhash` {#foldhash}

[![foldhash][c~foldhash~docs~badge]][c~foldhash~docs] [![foldhash~crates.io][c~foldhash~crates.io~badge]][c~foldhash~crates.io] [![foldhash~repo][c~foldhash~repo~badge]][c~foldhash~repo] [![foldhash~lib.rs][c~foldhash~lib.rs~badge]][c~foldhash~lib.rs]{{hi:foldhash}}{{hi:Hash}}{{hi:Hasher}}{{hi:No-std}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

A fast, non-cryptographic, minimally DoS-resistant hashing algorithm.

## Legacy Hashing Algorithms {#legacy-hashing-algorithms}

For legacy applications, you may consider using the following hashing algorithms. Note that these algorithms are considered weak by modern cryptographic standards and should not be used for security-sensitive applications.

### Hash with `sha1` {#sha1}

[![sha1][c~sha1~docs~badge]][c~sha1~docs] [![sha1~crates.io][c~sha1~crates.io~badge]][c~sha1~crates.io] [![sha1~repo][c~sha1~repo~badge]][c~sha1~repo] [![sha1~lib.rs][c~sha1~lib.rs~badge]][c~sha1~lib.rs]{{hi:sha1}}{{hi:Hash}}{{hi:sha1}}{{hi:Crypto}}{{hi:Digest}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`sha1`][c~sha1~docs]↗{{hi:sha1}} implements the SHA-1 hash function.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/hashing/sha1.rs:example}}
```

### Hash with `md-5` {#md-5}

[![md-5][c~md-5~docs~badge]][c~md-5~docs] [![md-5~crates.io][c~md-5~crates.io~badge]][c~md-5~crates.io] [![md-5~repo][c~md-5~repo~badge]][c~md-5~repo] [![md-5~lib.rs][c~md-5~lib.rs~badge]][c~md-5~lib.rs]{{hi:md-5}}{{hi:Hash}}{{hi:Md5}}{{hi:Crypto}}{{hi:Digest}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`md-5`][c~md-5~docs]↗{{hi:md-5}} implements the MD5 hash function.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/hashing/md5.rs:example}}
```

## Cryptograhic Algorithms {#cryptographic-algorithms}

Use [`ring`][c~ring~docs]↗{{hi:ring}}, [`rust-crypto`][c~rust-crypto~docs]↗{{hi:rust-crypto}}, [`sha2`][c~sha2~docs]↗{{hi:sha2}}. Choose carefully based on security needs and audit history.

- `argon2`, `scrypt`, `bcrypt` for password hashing,
- `aes-gcm-siv`, `aes-gcm`, and `chacha20poly1305` for AEAD Encryption,
- `rsa` for RSA,
- `ed25519`, `ecdsa`, `dsa` for digital signatures,
- `der`, `pem-rfc7468`, `pkcs8`, `x509-cert` for certificates,

## Related Topics {#related-topics .skip}

- [[algorithms | Algorithms]].
- [[data-structures | Data Structures]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[hashing: improve](https://github.com/john-cd/rust_howto/issues/273)

- [[cryptography | Cryptography]],
- [[encryption | Encryption]],
- [[password_hashing | Password Hashing]].

</div>
