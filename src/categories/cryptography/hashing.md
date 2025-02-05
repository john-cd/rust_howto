# Hashing

{{#include hashing.incl.md}}

## Calculate the SHA-256 digest of a file {#sha256}

[![ring][c-ring-badge]][c-ring] [![ring-crates.io][c-ring-crates.io-badge]][c-ring-crates.io] [![ring-github][c-ring-github-badge]][c-ring-github] [![ring-lib.rs][c-ring-lib.rs-badge]][c-ring-lib.rs]{{hi:ring}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Rand}}{{hi:Rsa}}{{hi:Ecc}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[![data-encoding][c-data_encoding-badge]][c-data_encoding] [![data-encoding-crates.io][c-data_encoding-crates.io-badge]][c-data_encoding-crates.io] [![data-encoding-github][c-data_encoding-github-badge]][c-data_encoding-github] [![data-encoding-lib.rs][c-data_encoding-lib.rs-badge]][c-data_encoding-lib.rs]{{hi:data-encoding}}{{hi:Hex}}{{hi:No_std}}{{hi:Base64}}{{hi:Base32}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Writes some data to a file, then calculates the SHA-256{{hi:SHA-256}} [`digest::Digest`][c-digest::Digest]{{hi:digest::Digest}}⮳ of the file's contents using [`digest::Context`][c-digest::Context]{{hi:digest::Context}}⮳

```rust,editable
{{#include ../../../crates/ex/cats/cryptography/tests/sha_digest.rs:example}}
```

## Use general-purpose hashing algorithms

For more algorithms, see Rust Crypto Hashes: sha2{{hi:sha2}}, sha1{{hi:sha1}}, md-5{{hi:md-5}}

### `blake3` {#blake3}

[![blake3][c-blake3-badge]][c-blake3] [![blake3-crates.io][c-blake3-crates.io-badge]][c-blake3-crates.io] [![blake3-github][c-blake3-github-badge]][c-blake3-github] [![blake3-lib.rs][c-blake3-lib.rs-badge]][c-blake3-lib.rs]{{hi:blake3}}

`blake3` implements the BLAKE3 hash function. BLAKE3 is a cryptographic hash function that is faster than MD5, SHA-1, SHA-2, and SHA-3, yet is at least as secure as the latest standard SHA-3.

```rust,editable
{{#include ../../../crates/ex/cats/cryptography/tests/blake3.rs:example}}
```

### `sha2` {#sha2}

[![sha2][c-sha2-badge]][c-sha2] [![sha2-crates.io][c-sha2-crates.io-badge]][c-sha2-crates.io] [![sha2-github][c-sha2-github-badge]][c-sha2-github] [![sha2-lib.rs][c-sha2-lib.rs-badge]][c-sha2-lib.rs]{{hi:sha2}}{{hi:Hash}}{{hi:Crypto}}{{hi:sha2}}{{hi:Digest}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the SHA-2 hash function family, including SHA-224, SHA-256, SHA-384, and SHA-512.

```rust,editable
{{#include ../../../crates/ex/cats/cryptography/tests/sha2.rs:example}}
```

### `sha1` {#sha1}

[![sha1][c-sha1-badge]][c-sha1] [![sha1-crates.io][c-sha1-crates.io-badge]][c-sha1-crates.io] [![sha1-github][c-sha1-github-badge]][c-sha1-github] [![sha1-lib.rs][c-sha1-lib.rs-badge]][c-sha1-lib.rs]{{hi:sha1}}{{hi:Hash}}{{hi:sha1}}{{hi:Crypto}}{{hi:Digest}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

SHA-1 hash function

```rust,editable
{{#include ../../../crates/ex/cats/cryptography/tests/sha1.rs:example}}
```

### `md-5` {#md-5}

[![md-5][c-md_5-badge]][c-md_5] [![md-5-crates.io][c-md_5-crates.io-badge]][c-md_5-crates.io] [![md-5-github][c-md_5-github-badge]][c-md_5-github] [![md-5-lib.rs][c-md_5-lib.rs-badge]][c-md_5-lib.rs]{{hi:md-5}}{{hi:Hash}}{{hi:Md5}}{{hi:Crypto}}{{hi:Digest}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

MD5 hash function

```rust,editable
{{#include ../../../crates/ex/cats/cryptography/tests/md_5.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

[hashing: improve (P1)](https://github.com/john-cd/rust_howto/issues/273)  review [blessed.rs][blessed-rs-website]

</div>
