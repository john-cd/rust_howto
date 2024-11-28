# Encryption

{{#include encryption.incl.md}}

## Salt and hash a password with PBKDF2 {#pbkdf2}

[![ring][c-ring-badge]][c-ring] [![ring-crates.io][c-ring-crates.io-badge]][c-ring-crates.io] [![ring-github][c-ring-github-badge]][c-ring-github] [![ring-lib.rs][c-ring-lib.rs-badge]][c-ring-lib.rs]{{hi:ring}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Rand}}{{hi:Rsa}}{{hi:Ecc}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[![data-encoding][c-data_encoding-badge]][c-data_encoding] [![data-encoding-crates.io][c-data_encoding-crates.io-badge]][c-data_encoding-crates.io] [![data-encoding-github][c-data_encoding-github-badge]][c-data_encoding-github] [![data-encoding-lib.rs][c-data_encoding-lib.rs-badge]][c-data_encoding-lib.rs]{{hi:data-encoding}}{{hi:Hex}}{{hi:No_std}}{{hi:Base64}}{{hi:Base32}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Uses [`ring::pbkdf2`][c-ring::pbkdf2]{{hi:ring::pbkdf2}}⮳ to hash a salted password{{hi:Salted passwords}} using the PBKDF2{{hi:PBKDF2}} key derivation function [`ring::pbkdf2::derive`][c-ring::pbkdf2::derive]{{hi:ring::pbkdf2::derive}}⮳
Verifies the hash{{hi:Hashing}} is correct with [`ring::pbkdf2::verify`][c-ring::pbkdf2::verify]{{hi:ring::pbkdf2::verify}}⮳
The salt is generated using [`ring::rand::SecureRandom::fill`][c-ring::rand::SecureRandom::fill]{{hi:ring::rand::SecureRandom::fill}}⮳ which fills the salt byte array with securely generated random numbers.

```rust,editable
{{#include ../../../deps/tests/cats/cryptography/pbkdf2.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P2 expand
</div>
