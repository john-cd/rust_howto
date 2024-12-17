# Password Hashing

{{#include _password_hashing.incl.md}}

## `argon2` {#argon2}

[![argon2][c-argon2-badge]][c-argon2]{{hi:argon2}}
[![argon2-crates.io][c-argon2-crates.io-badge]][c-argon2-crates.io]
[![argon2-github][c-argon2-github-badge]][c-argon2-github]
[![argon2-lib.rs][c-argon2-lib.rs-badge]][c-argon2-lib.rs]
[![cat-authentication][cat-authentication-badge]][cat-authentication]{{hi:Authentication}}
[![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the Argon2 password hashing function with support for the Argon2d, Argon2i, and Argon2id algorithmic variants

```rust,editable
{{#include ../../../deps/tests/categories/cryptography/argon2.rs:example}}
```

## `scrypt` {#scrypt}

[![scrypt][c-scrypt-badge]][c-scrypt]{{hi:scrypt}}
[![scrypt-crates.io][c-scrypt-crates.io-badge]][c-scrypt-crates.io]
[![scrypt-github][c-scrypt-github-badge]][c-scrypt-github]
[![scrypt-lib.rs][c-scrypt-lib.rs-badge]][c-scrypt-lib.rs]
[![cat-authentication][cat-authentication-badge]][cat-authentication]{{hi:Authentication}}
[![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

The scrypt key derivation function is designed to be far more secure against hardware brute-force attacks than alternative functions such as PBKDF2 or bcrypt.

```rust,editable
{{#include ../../../deps/tests/categories/cryptography/scrypt.rs:example}}
```

## `bcrypt` {#bcrypt}

[![bcrypt][c-bcrypt-badge]][c-bcrypt]{{hi:bcrypt}}
[![bcrypt-crates.io][c-bcrypt-crates.io-badge]][c-bcrypt-crates.io]
[![bcrypt-github][c-bcrypt-github-badge]][c-bcrypt-github]
[![bcrypt-lib.rs][c-bcrypt-lib.rs-badge]][c-bcrypt-lib.rs]

Hash and verify passwords.

```rust,editable
{{#include ../../../deps/tests/categories/cryptography/bcrypt.rs:example}}
```

For more algorithms, see [Rust Crypto Password Hashes][rustcrypto-password-hashes-github]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[_password_hashing: write (P1)](https://github.com/john-cd/rust_howto/issues/275)

[Key derivation function][key_derivation_function]
[key_derivation_function]: https://en.wikipedia.org/wiki/Key_derivation_function
</div>
