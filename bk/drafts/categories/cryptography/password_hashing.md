<!-- TODO Reviewed ??-??-2026 -->

# Password Hashing

{{#include password_hashing.incl.md}}

Password hashing stores a deliberately expensive, salted verifier instead of a user's plaintext password. It does not make a database breach harmless: an attacker can still guess passwords offline, so use a password-hashing algorithm and parameters chosen to make each guess costly.

A [key derivation function][key_derivation_function]↗ (KDF) is a cryptographic algorithm that derives one or more secret keys from a secret value, such as a master key, a password, or a passphrase, using a pseudorandom function (typically a cryptographic hash function or block cipher).

The original use for a KDF is key derivation, the generation of multiple keys from secret passwords or passphrases.

Despite their original use for key derivation, KDFs are possibly better known for their use in password [hashing][p~hashing] (password verification by hash comparison), as used by the [`passwd`][wikipedia~passwd]↗{{hi:passwd}} file or `shadow` password file. Password hash [functions][p~functions] should be relatively expensive to calculate in case of brute-force attacks, and the key stretching of KDFs happen to provide this characteristic.

In that role, key derivation [functions][p~functions] take a password, a salt, (and sometimes a cost factor) as inputs, then generate a password hash - deliberately slowly. Their purpose is to make each password guessing trial by an attacker who has obtained a password hash file expensive and therefore the cost of a guessing attack high or prohibitive. In cryptography, "salt" refers to non-secret, random data added to input data before hashing it.

Popular password hashing algorithms include [`bcrypt`][c~bcrypt~docs]↗{{hi:bcrypt}}, [`Argon2`][c~argon2~docs]↗{{hi:Argon2}} and [`scrypt`][c~scrypt~docs]↗{{hi:scrypt}}.

For new applications, prefer Argon2id and tune its memory, iteration, and parallelism parameters for the deployment environment. Store the complete PHC string, which includes the algorithm parameters and salt, alongside the verifier. After a successful login, check whether the stored parameters need an upgrade and rehash when necessary. Use PBKDF2 or bcrypt when compatibility requires them; do not use a fast general-purpose hash such as SHA-256 for passwords.

## Hash a Password, then Verify a Password Against the Hash {#argon2}

{{#badges argon2 authentication=Authentication cryptography=Cryptography no-std="No standard library"}}

[`argon2`][c~argon2~docs]↗{{hi:argon2}} is a pure-Rust implementation of the ['Argon2'][argon2~wikipedia]↗ key derivation function, which is commonly used for secure password hashing.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/password_hashing/argon2.rs:example}}
```

## `scrypt` {#scrypt}

{{#badges scrypt authentication=Authentication cryptography=Cryptography no-std="No standard library"}}

The [`scrypt`][c~scrypt~docs]↗{{hi:scrypt}} key derivation function is designed to be far more secure against hardware brute-force attacks than alternative functions such as [`PBKDF2`][c~ring::pbkdf2~docs]↗{{hi:PBKDF2}} or ['bcrypt'](#bcrypt).

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/password_hashing/scrypt.rs:example}}
```

## `bcrypt` {#bcrypt}

{{#badges bcrypt}}

['bcrypt'][bcrypt~wikipedia]↗ is a password-hashing function. Besides incorporating a salt to protect against rainbow table attacks, ['bcrypt'][bcrypt~wikipedia]↗ is an adaptive function: over time, the iteration count can be increased to make it slower, so it remains resistant to brute-force [search][p~search] attacks even with increasing computation power. ['bcrypt'][bcrypt~wikipedia]↗ is not a key derivation function (KDF). For example, [`bcrypt`][c~bcrypt~docs]↗{{hi:bcrypt}} cannot be used to [derive][p~derive] a 512-bit key from a password.

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

- [[hashing | Cryptographic hashing]].
- [Authentication](../authentication/index.md).
- [[encryption | Encryption]].
- [[aead | Authenticated encryption]].
- [[secrets | Secret handling]].
- [Cryptography category](index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
