# Hashing

{{#include hashing.incl.md}}

Hashing is a process that converts input data of any size into a fixed-size string of characters, typically a sequence of numbers and letters. The output, known as a hash value or hash code, is generated by a hash function. Hash functions are designed to be fast, deterministic (the same input will always produce the same output), and produce a unique hash for distinct inputs. One key feature of hashing is that it's a one-way function; you can't reverse-engineer the original input from the hash value.

Hashes are widely used in various applications such as:

- Data Integrity: Ensuring that data has not been altered by comparing the hash value of the original data with the hash value of the received data.
- Password Storage: Storing hashed versions of passwords rather than plain text to enhance security.
- Digital Signatures: Verifying the authenticity and integrity of messages or documents.

## Calculate the SHA-256 Digest of a File {#sha256}

[![ring][c~ring~docs~badge]][c~ring~docs] [![ring~crates.io][c~ring~crates.io~badge]][c~ring~crates.io] [![ring~github][c~ring~github~badge]][c~ring~github] [![ring~lib.rs][c~ring~lib.rs~badge]][c~ring~lib.rs]{{hi:ring}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Rand}}{{hi:Rsa}}{{hi:ECC}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[![data-encoding][c~data_encoding~docs~badge]][c~data_encoding~docs] [![data-encoding~crates.io][c~data_encoding~crates.io~badge]][c~data_encoding~crates.io] [![data-encoding~github][c~data_encoding~github~badge]][c~data_encoding~github] [![data-encoding~lib.rs][c~data_encoding~lib.rs~badge]][c~data_encoding~lib.rs]{{hi:data-encoding}}{{hi:Hex}}{{hi:No_std}}{{hi:Base64}}{{hi:Base32}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

SHA-256 (Secure Hash Algorithm 256-bit) is part of the SHA-2 family of cryptographic hash functions. It produces a fixed-size 256-bit hash value (64 characters) from input data of any size. SHA-256 is widely used in applications such as digital signatures, certificate generation, and data integrity verification.

Writes some data to a file, then calculates the SHA-256{{hi:SHA-256}} [`digest::Digest`][c~digest::Digest~docs]{{hi:digest::Digest}}⮳ of the file's contents using [`digest::Context`][c~digest::Context~docs]{{hi:digest::Context}}⮳.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/hashing/sha_digest.rs:example}}
```

## Use General-purpose Hashing Algorithms {#skip1}

For more [algorithms][p~algorithms], see Rust Crypto Hashes: sha2{{hi:sha2}}, sha1{{hi:sha1}}, md-5{{hi:md-5}}

### `blake3` {#blake3}

[![blake3][c~blake3~docs~badge]][c~blake3~docs] [![blake3~crates.io][c~blake3~crates.io~badge]][c~blake3~crates.io] [![blake3~github][c~blake3~github~badge]][c~blake3~github] [![blake3~lib.rs][c~blake3~lib.rs~badge]][c~blake3~lib.rs]{{hi:blake3}}

[`blake3`][c~blake3~docs]⮳{{hi:blake3}} implements the BLAKE3 hash function. BLAKE3 is a cryptographic hash function that is faster than MD5, SHA-1, SHA-2, and SHA-3, yet is at least as secure as the latest standard SHA-3. It's designed to take advantage of parallel processing capabilities. BLAKE3 can produce hashes of arbitrary length, from short digests to longer ones. This is useful for various applications, including key derivation and password hashing. BLAKE3 allows for incremental hashing, where you can update the hash state with new data without recomputing the entire hash. This is useful for streaming data or situations where the input is received in chunks.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/hashing/blake3.rs:example}}
```

### `sha2` {#sha2}

[![sha2][c~sha2~docs~badge]][c~sha2~docs] [![sha2~crates.io][c~sha2~crates.io~badge]][c~sha2~crates.io] [![sha2~github][c~sha2~github~badge]][c~sha2~github] [![sha2~lib.rs][c~sha2~lib.rs~badge]][c~sha2~lib.rs]{{hi:sha2}}{{hi:Hash}}{{hi:Crypto}}{{hi:sha2}}{{hi:Digest}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

SHA-2 (Secure Hash Algorithm 2) is a family of cryptographic hash functions designed by the National Security Agency (NSA) and standardized by NIST.

[`sha2`][c~sha2~docs]⮳{{hi:sha2}} is a pure Rust implementation of the SHA-2 hash function family, including SHA-224, SHA-256, SHA-384, and SHA-512. SHA-256 is the most commonly used variant.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/hashing/sha2.rs:example}}
```

### `sha1` {#sha1}

[![sha1][c~sha1~docs~badge]][c~sha1~docs] [![sha1~crates.io][c~sha1~crates.io~badge]][c~sha1~crates.io] [![sha1~github][c~sha1~github~badge]][c~sha1~github] [![sha1~lib.rs][c~sha1~lib.rs~badge]][c~sha1~lib.rs]{{hi:sha1}}{{hi:Hash}}{{hi:sha1}}{{hi:Crypto}}{{hi:Digest}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`sha1`][c~sha1~docs]⮳{{hi:sha1}} implements the SHA-1 hash function.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/hashing/sha1.rs:example}}
```

### `md-5` {#md-5}

[![md-5][c~md_5~docs~badge]][c~md_5~docs] [![md-5~crates.io][c~md_5~crates.io~badge]][c~md_5~crates.io] [![md-5~github][c~md_5~github~badge]][c~md_5~github] [![md-5~lib.rs][c~md_5~lib.rs~badge]][c~md_5~lib.rs]{{hi:md-5}}{{hi:Hash}}{{hi:Md5}}{{hi:Crypto}}{{hi:Digest}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`md-5`][c~md_5~docs]⮳{{hi:md-5}} implements the MD5 hash function.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/hashing/md_5.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[hashing: improve](https://github.com/john-cd/rust_howto/issues/273) review [blessed.rs][blessed-rs~website]
</div>
