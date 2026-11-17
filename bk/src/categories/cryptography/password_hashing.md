<!-- TODO Reviewed 11-17-2026. -->

# Password Hashing

{{#include password_hashing.incl.md}}

Password hashing stores a deliberately expensive, salted verifier instead of a user's plaintext password. It does not make a database breach harmless: an attacker can still guess passwords offline, so use a password-hashing algorithm and parameters chosen to make each guess costly.

A password-hashing function is deliberately expensive to make each offline password guess costly. It uses a unique, non-secret salt for each password; the salt and algorithm parameters are normally stored with the resulting verifier. Password hashing does not encrypt passwords or prevent an attacker from making guesses.

For new applications, prefer Argon2id and tune its memory, iteration, and parallelism parameters for the deployment environment. Store the complete PHC string, which includes the algorithm parameters and salt, alongside the verifier. After a successful login, check whether the stored parameters need an upgrade and rehash when necessary. Use PBKDF2 or bcrypt when compatibility requires them; do not use a fast general-purpose hash such as SHA-256 for passwords.

## Hash a Password, then Verify a Password Against the Hash {#argon2}

{{#badges argon2 authentication=Authentication cryptography=Cryptography no-std="No standard library"}}

[`argon2`][c~argon2~docs]↗{{hi:argon2}} is a pure-Rust implementation of the ['Argon2'][argon2~wikipedia]↗ key derivation function, which is commonly used for secure password hashing.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/password_hashing/argon2.rs:example}}
```

## `scrypt` {#scrypt}

{{#badges scrypt authentication=Authentication cryptography=Cryptography no-std="No standard library"}}

The [`scrypt`][c~scrypt~docs]↗{{hi:scrypt}} key derivation function uses a memory-hard design intended to raise the cost of large-scale password guessing. Choose its parameters for the deployment environment and compare them with the alternatives you support; it is not universally more secure than [`PBKDF2`][c~ring::pbkdf2~docs]↗{{hi:PBKDF2}} or [bcrypt](#bcrypt).

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/password_hashing/scrypt.rs:example}}
```

## `bcrypt` {#bcrypt}

{{#badges bcrypt}}

[`bcrypt`][c~bcrypt~docs]↗{{hi:bcrypt}} is a salted password-hashing function with an adjustable work factor. It is useful when an existing system requires bcrypt; for a new application, prefer a memory-hard algorithm such as Argon2id. Bcrypt is for password verification, not deriving arbitrary-length encryption keys.

[`bcrypt`][c~bcrypt~docs]↗{{hi:bcrypt}}

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/password_hashing/bcrypt.rs:example}}
```

## Salt and Hash a Password with PBKDF2 {#pbkdf2}

{{#badges ring}} {{hi:Crypto}} {{hi:Cryptography}} {{hi:Rand}} {{hi:Rsa}} {{hi:ECC}} {{#badges data-encoding}} {{hi:Hex}} {{hi:No_std}} {{hi:Base64}} {{hi:Base32}} {{#categories cryptography=Cryptography,encoding=Encoding,no-std="No standard library"}}

Uses [`ring::pbkdf2`][c~ring::pbkdf2~docs]↗{{hi:ring::pbkdf2}} to hash a salted password{{hi:Salted passwords}} using the PBKDF2{{hi:PBKDF2}} key derivation function [`ring::pbkdf2::derive`][c~ring::pbkdf2::derive~docs]↗{{hi:ring::pbkdf2::derive}}.
Verifies the hash{{hi:Hashing}} is correct with [`ring::pbkdf2::verify`][c~ring::pbkdf2::verify~docs]↗{{hi:ring::pbkdf2::verify}}.
The salt is generated using [`ring::rand::SecureRandom::fill`][c~ring::rand::SecureRandom::fill~docs]↗{{hi:ring::rand::SecureRandom::fill}} which fills the salt byte array with securely generated random numbers.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/password_hashing/pbkdf2.rs:example}}
```

For more [algorithms][p~algorithms], see [Rust Crypto Password Hashes][rustcrypto-password-hashes~repo]↗.

## Related Topics {#related-topics .skip}

- [Cryptographic Hashing](../algorithms/hashing.md).
- [Authentication](../authentication/index.md).
- [Encryption](encryption.md).
- [AEAD](aead.md).
- [Secret Handling](secrets.md).
- [Cryptography category](index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
