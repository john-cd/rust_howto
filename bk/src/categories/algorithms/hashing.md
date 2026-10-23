<!-- TODO Reviewed ??-??-2026 -->
# Hashing

{{#include hashing.incl.md}}

Hash functions map input of any length to a fixed-size output, often called a _hash_, _hash value_, or _digest_. A given function is deterministic: the same input produces the same output. Since the set of outputs is finite, different inputs can collide. Cryptographic hash functions are designed to make finding collisions or recovering an input infeasible; ordinary hash functions may only prioritize speed.

Hashes are used for several different purposes:

- Hash tables compute a hash of each key to find a place to store or look it up. This hash is an implementation detail, not a stable identifier to save or send.
- A checksum or cryptographic digest can detect changes when it is compared with a trusted value. A plain digest does not authenticate who supplied the data.
- Password storage requires a dedicated, deliberately expensive password-hashing function. Do not store passwords with SHA-256, BLAKE3, or another fast general-purpose hash.
- Digital-signature schemes commonly hash data as part of signing and verification; use a complete signature scheme rather than treating a digest as a signature.

Choose an implementation based on the required properties:

**Cryptographic digests:**
- [`sha2`][c~sha2~docs]↗ for the SHA-2 family.
- [`blake3`][c~blake3~docs]↗ for a fast, parallelizable cryptographic hash.

**Non-cryptographic hashers:**
- `rustc-hash`, `foldhash`, `murmur3`, and `fnv` can suit trusted inputs when speed matters. They do not provide cryptographic collision resistance.

For attacker-controlled keys, keep the standard library's randomized `HashMap` hasher unless you have measured a need to change it and reviewed the security trade-off. `hashbrown` is a hash-map implementation, not a hash function.

**Checksums:** `crc` or `crc32fast` can detect accidental corruption. CRCs are not cryptographic and do not protect against deliberate changes.

[RustCrypto Hashes](https://github.com/RustCrypto/hashes#rustcrypto-hashes)

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

Use a cryptographic hash when a digest must resist deliberate attacks. For passwords, use a dedicated password-hashing function instead.

### Hash with `blake3` {#blake3}

[![blake3][c~blake3~docs~badge]][c~blake3~docs] [![blake3~crates.io][c~blake3~crates.io~badge]][c~blake3~crates.io] [![blake3~repo][c~blake3~repo~badge]][c~blake3~repo] [![blake3~lib.rs][c~blake3~lib.rs~badge]][c~blake3~lib.rs]{{hi:blake3}}

[`blake3`][c~blake3~docs]↗{{hi:blake3}} implements a cryptographic hash with a tree structure that can use parallel processing. It supports incremental hashing and output of arbitrary length. BLAKE3 also provides keyed hashing and key-derivation modes, but it is not a password-hashing function; use a dedicated function such as Argon2id for passwords.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/hashing/blake3.rs:example}}
```

### Hash with `sha2` {#sha2}

[![sha2][c~sha2~docs~badge]][c~sha2~docs] [![sha2~crates.io][c~sha2~crates.io~badge]][c~sha2~crates.io] [![sha2~repo][c~sha2~repo~badge]][c~sha2~repo] [![sha2~lib.rs][c~sha2~lib.rs~badge]][c~sha2~lib.rs]{{hi:sha2}}{{hi:Hash}}{{hi:Crypto}}{{hi:sha2}}{{hi:Digest}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

SHA-2 (Secure Hash Algorithm 2) is a family of cryptographic hash functions designed by the National Security Agency (NSA) and standardized by NIST.

[`sha2`][c~sha2~docs]↗{{hi:sha2}} is a pure Rust implementation of the SHA-2 hash function family, including SHA-224, SHA-256, SHA-384, and SHA-512. SHA-256 is the most commonly used variant.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/hashing/sha2.rs:example}}
```

### `foldhash` {#foldhash}

[![foldhash][c~foldhash~docs~badge]][c~foldhash~docs] [![foldhash~crates.io][c~foldhash~crates.io~badge]][c~foldhash~crates.io] [![foldhash~repo][c~foldhash~repo~badge]][c~foldhash~repo] [![foldhash~lib.rs][c~foldhash~lib.rs~badge]][c~foldhash~lib.rs]{{hi:foldhash}}{{hi:Hash}}{{hi:Hasher}}{{hi:No-std}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}

A fast, non-cryptographic hash function. Do not use it as a cryptographic digest or assume it protects attacker-controlled keys from collision attacks.

## Legacy Hashing Algorithms {#legacy-hashing-algorithms}

For legacy applications, you may consider using the following hashing algorithms. Note that these algorithms are considered weak by modern cryptographic standards and should not be used for security-sensitive applications.

### Hash with `sha1` {#sha1}

[![sha1][c~sha1~docs~badge]][c~sha1~docs] [![sha1~crates.io][c~sha1~crates.io~badge]][c~sha1~crates.io] [![sha1~repo][c~sha1~repo~badge]][c~sha1~repo] [![sha1~lib.rs][c~sha1~lib.rs~badge]][c~sha1~lib.rs]{{hi:sha1}}{{hi:Hash}}{{hi:sha1}}{{hi:Crypto}}{{hi:Digest}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`sha1`][c~sha1~docs]↗{{hi:sha1}} implements the SHA-1 hash function.

```rust,editable,noplayground
{{#include ../../../crates/cats/algorithms/examples/hashing/sha1.rs:example}}
```

### Hash with `md-5` {#md-5}

[![md-5][c~md-5~docs~badge]][c~md-5~docs] [![md-5~crates.io][c~md-5~crates.io~badge]][c~md-5~crates.io] [![md-5~repo][c~md-5~repo~badge]][c~md-5~repo] [![md-5~lib.rs][c~md-5~lib.rs~badge]][c~md-5~lib.rs]{{hi:md-5}}{{hi:Hash}}{{hi:Md5}}{{hi:Crypto}}{{hi:Digest}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`md-5`][c~md-5~docs]↗{{hi:md-5}} implements the MD5 hash function.

```rust,editable
{{#include ../../../crates/cats/algorithms/examples/hashing/md5.rs:example}}
```

## Cryptographic Algorithms {#cryptographic-algorithms}

This page covers hashes. Use the dedicated topics for other cryptographic operations:

- [[password_hashing | Password Hashing]] for password storage with Argon2id, scrypt, or bcrypt.
- [[encryption | Encryption]] for authenticated encryption and message authentication.
- [[cryptography | Cryptography]] for public-key algorithms, signatures, and certificates.

## Related Topics {#related-topics .skip}

- [[algorithms | Algorithms]].
- [[cryptography | Cryptography]].
- [[data-structures | Data Structures]].
- [[hashmaps | Hash Maps]].
- [[encryption | Encryption]].
- [[password_hashing | Password Hashing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
