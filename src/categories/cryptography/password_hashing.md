# Password Hashing

{{#include password_hashing.incl.md}}

A [key derivation function][key_derivation_function]⮳ (KDF) is a cryptographic algorithm that derives one or more secret keys from a secret value, such as a master key, a password, or a passphrase, using a pseudorandom function (typically a cryptographic hash function or block cipher).

The original use for a KDF is key derivation, the generation of multiple keys from secret passwords or passphrases.

Despite their original use for key derivation, KDFs are possibly better known for their use in password hashing (password verification by hash comparison), as used by the `passwd` file or `shadow` password file. Password hash functions should be relatively expensive to calculate in case of brute-force attacks, and the key stretching of KDFs happen to provide this characteristic. The non-secret parameters are called "salt" in this context.

In that role, key derivation functions take a password, a salt, (and sometimes a cost factor) as inputs, then generate a password hash - deliberately slowly. Their purpose is to make each password guessing trial by an attacker who has obtained a password hash file expensive and therefore the cost of a guessing attack high or prohibitive.

## Hash a password, then verify a password against the hash {#argon2}

[![argon2][c-argon2-badge]][c-argon2]{{hi:argon2}}
[![argon2-crates.io][c-argon2-crates.io-badge]][c-argon2-crates.io]
[![argon2-github][c-argon2-github-badge]][c-argon2-github]
[![argon2-lib.rs][c-argon2-lib.rs-badge]][c-argon2-lib.rs]
[![cat-authentication][cat-authentication-badge]][cat-authentication]{{hi:Authentication}}
[![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

`argon2` is a pure-Rust implementation of the ['Argon2'][argon2-wikipedia]⮳ key derivation function, which is commonly used for secure password hashing.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/argon2.rs:example}}
```

## `scrypt` {#scrypt}

[![scrypt][c-scrypt-badge]][c-scrypt]{{hi:scrypt}}
[![scrypt-crates.io][c-scrypt-crates.io-badge]][c-scrypt-crates.io]
[![scrypt-github][c-scrypt-github-badge]][c-scrypt-github]
[![scrypt-lib.rs][c-scrypt-lib.rs-badge]][c-scrypt-lib.rs]
[![cat-authentication][cat-authentication-badge]][cat-authentication]{{hi:Authentication}}
[![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

The 'scrypt' key derivation function is designed to be far more secure against hardware brute-force attacks than alternative functions such as 'PBKDF2' or ['bcrypt'](#bcrypt).

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/scrypt.rs:example}}
```

## `bcrypt` {#bcrypt}

[![bcrypt][c-bcrypt-badge]][c-bcrypt]{{hi:bcrypt}}
[![bcrypt-crates.io][c-bcrypt-crates.io-badge]][c-bcrypt-crates.io]
[![bcrypt-github][c-bcrypt-github-badge]][c-bcrypt-github]
[![bcrypt-lib.rs][c-bcrypt-lib.rs-badge]][c-bcrypt-lib.rs]

['bcrypt'][bcrypt-wikipedia]⮳ is a password-hashing function. Besides incorporating a salt to protect against rainbow table attacks, ['bcrypt'][bcrypt-wikipedia]⮳ is an adaptive function: over time, the iteration count can be increased to make it slower, so it remains resistant to brute-force search attacks even with increasing computation power. ['bcrypt'][bcrypt-wikipedia]⮳ is not a key derivation function (KDF). For example, bcrypt cannot be used to derive a 512-bit key from a password.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/cryptography/bcrypt.rs:example}}
```

For more algorithms, see [Rust Crypto Password Hashes][rustcrypto-password-hashes-github].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[password_hashing: write (P1)](https://github.com/john-cd/rust_howto/issues/275)
</div>
