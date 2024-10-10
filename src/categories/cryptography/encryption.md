# Encryption

{{#include encryption.incl.md}}

<a name="ex-pbkdf2"></a>

## Salt and hash a password with PBKDF2

[![ring][ring-badge]][c-ring]  [![data-encoding][data-encoding-badge]][data-encoding]  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]

Uses [`{{i:ring::pbkdf2}}`][c-ring::pbkdf2]⮳ to hash a {{i:salted password}} using the {{i:PBKDF2}} key derivation function [`{{i:pbkdf2::derive}}`][c-ring::pbkdf2::derive]⮳
Verifies the {{i:hash}} is correct with [`{{i:pbkdf2::verify}}`][c-ring::pbkdf2::verify]⮳
The salt is generated using [`{{i:SecureRandom::fill}}`][c-ring::rand::SecureRandom::fill]⮳ which fills the salt byte array with securely generated random numbers.

```rust,editable
{{#include ../../../deps/tests/pbkdf2.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
