# Encryption

{{#include encryption.incl.md}}

## Salt and hash a password with PBKDF2 {#pbkdf2}

[![ring][c-ring-badge]][c-ring]{{hi:ring}} [![data-encoding][c-data_encoding-badge]][c-data_encoding]{{hi:data-encoding}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}

Uses [`ring::pbkdf2`][c-ring::pbkdf2]{{hi:ring::pbkdf2}}⮳ to hash a salted password{{hi:Salted passwords}} using the PBKDF2{{hi:PBKDF2}} key derivation function [`ring::pbkdf2::derive`][c-ring::pbkdf2::derive]{{hi:ring::pbkdf2::derive}}⮳
Verifies the hash{{hi:Hashing}} is correct with [`ring::pbkdf2::verify`][c-ring::pbkdf2::verify]{{hi:ring::pbkdf2::verify}}⮳
The salt is generated using [`ring::rand::SecureRandom::fill`][c-ring::rand::SecureRandom::fill]{{hi:ring::rand::SecureRandom::fill}}⮳ which fills the salt byte array with securely generated random numbers.

```rust,editable
{{#include ../../../deps/tests/cats/cryptography/pbkdf2.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
