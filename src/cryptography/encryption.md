# Encryption

<a name="ex-pbkdf2"></a>

## Salt and hash a password with PBKDF2

[![ring-badge]][ring] [![data-encoding-badge]][data-encoding] [![cat-cryptography-badge]][cat-cryptography]

Uses [`ring::pbkdf2`] to hash a salted password using the PBKDF2 key derivation
function [`pbkdf2::derive`].  Verifies the hash is correct with
[`pbkdf2::verify`]. The salt is generated using
[`SecureRandom::fill`], which fills the salt byte array with
securely generated random numbers.

```rust,editable
{#include ../../deps/examples/pbkdf2.rs}
```

[`pbkdf2::derive`]: https://briansmith.org/rustdoc/ring/pbkdf2/fn.derive.html
[`pbkdf2::verify`]: https://briansmith.org/rustdoc/ring/pbkdf2/fn.verify.html
[`ring::pbkdf2`]: https://briansmith.org/rustdoc/ring/pbkdf2/index.html
[`SecureRandom::fill`]: https://briansmith.org/rustdoc/ring/rand/trait.SecureRandom.html#tymethod.fill
{{#include ../refs/link-refs.md}}
