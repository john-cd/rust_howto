# Encryption

{{#include encryption.incl.md}}

<a name="ex-pbkdf2"></a>

## Salt and hash a password with PBKDF2

[![ring][c-ring-badge]][c-ring]  [![data_encoding][c-data_encoding-badge]][c-data_encoding]  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]

Uses {{hi:ring::pbkdf2}}[`ring::pbkdf2`][c-ring::pbkdf2]⮳ to hash a {{i:salted password}} using the {{i:PBKDF2}} key derivation function {{hi:pbkdf2::derive}}[`pbkdf2::derive`][c-ring::pbkdf2::derive]⮳
Verifies the {{i:hash}} is correct with {{hi:pbkdf2::verify}}[`pbkdf2::verify`][c-ring::pbkdf2::verify]⮳
The salt is generated using {{hi:SecureRandom::fill}}[`SecureRandom::fill`][c-ring::rand::SecureRandom::fill]⮳ which fills the salt byte array with securely generated random numbers.

```rust,editable
{{#include ../../../deps/tests/pbkdf2.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
