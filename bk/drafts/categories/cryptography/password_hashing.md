# Password Hashing

{{#include password_hashing.incl.md}}

Password [hashing][p~hashing] is a crucial security practice that protects user passwords by one-way transforming them into a string of characters, called a "hash", which reverse-engineering into the password is computationally infeasible. This hash is then stored instead of the actual password. Password [hashing][p~hashing] protects against data breaches: If a [database][p~database] is compromised, attackers won't be able to see the actual passwords, only the hashed versions.

A [key derivation function][key_derivation_function]↗ (KDF) is a cryptographic algorithm that derives one or more secret keys from a secret value, such as a master key, a password, or a passphrase, using a pseudorandom function (typically a cryptographic hash function or block cipher).

The original use for a KDF is key derivation, the generation of multiple keys from secret passwords or passphrases.

Despite their original use for key derivation, KDFs are possibly better known for their use in password [hashing][p~hashing] (password verification by hash comparison), as used by the [`passwd`][wikipedia~passwd]↗{{hi:passwd}} file or `shadow` password file. Password hash [functions][p~functions] should be relatively expensive to calculate in case of brute-force attacks, and the key stretching of KDFs happen to provide this characteristic.
Despite their original use for key derivation, KDFs are possibly better known for their use in password [hashing][p~hashing] (password verification by hash comparison), as used by the [`passwd`][wikipedia~passwd]↗{{hi:passwd}} file or `shadow` password file. Password hash [functions][p~functions] should be relatively expensive to calculate in case of brute-force attacks, and the key stretching of KDFs happen to provide this characteristic.

In that role, key derivation [functions][p~functions] take a password, a salt, (and sometimes a cost factor) as inputs, then generate a password hash - deliberately slowly. Their purpose is to make each password guessing trial by an attacker who has obtained a password hash file expensive and therefore the cost of a guessing attack high or prohibitive. In cryptography, "salt" refers to non-secret, random data added to input data before hashing it.

Popular password hashing algorithms include [`bcrypt`][c~bcrypt~docs]↗{{hi:bcrypt}}, [`Argon2`][c~argon2~docs]↗{{hi:Argon2}} and [`scrypt`][c~scrypt~docs]↗{{hi:scrypt}}.

## Hash a Password, then Verify a Password Against the Hash {#argon2}

[![argon2][c~argon2~docs~badge]][c~argon2~docs]{{hi:argon2}}
[![argon2~crates.io][c~argon2~crates.io~badge]][c~argon2~crates.io]
[![argon2~repo][c~argon2~repo~badge]][c~argon2~repo]
[![argon2~lib.rs][c~argon2~lib.rs~badge]][c~argon2~lib.rs]
[![cat~authentication][cat~authentication~badge]][cat~authentication]{{hi:Authentication}}
[![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`argon2`][c~argon2~docs]↗{{hi:argon2}} is a pure-Rust implementation of the ['Argon2'][argon2~wikipedia]↗ key derivation function, which is commonly used for secure password hashing.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/password_hashing/argon2.rs:example}}
```

## `scrypt` {#scrypt}

[![scrypt][c~scrypt~docs~badge]][c~scrypt~docs]{{hi:scrypt}}
[![scrypt~crates.io][c~scrypt~crates.io~badge]][c~scrypt~crates.io]
[![scrypt~repo][c~scrypt~repo~badge]][c~scrypt~repo]
[![scrypt~lib.rs][c~scrypt~lib.rs~badge]][c~scrypt~lib.rs]
[![cat~authentication][cat~authentication~badge]][cat~authentication]{{hi:Authentication}}
[![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

The [`scrypt`][c~scrypt~docs]↗{{hi:scrypt}} key derivation function is designed to be far more secure against hardware brute-force attacks than alternative functions such as [`PBKDF2`][c~ring::pbkdf2~docs]↗{{hi:PBKDF2}} or ['bcrypt'](#bcrypt).

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/password_hashing/scrypt.rs:example}}
```

## `bcrypt` {#bcrypt}

[![bcrypt][c~bcrypt~docs~badge]][c~bcrypt~docs]{{hi:bcrypt}}
[![bcrypt~crates.io][c~bcrypt~crates.io~badge]][c~bcrypt~crates.io]
[![bcrypt~repo][c~bcrypt~repo~badge]][c~bcrypt~repo]
[![bcrypt~lib.rs][c~bcrypt~lib.rs~badge]][c~bcrypt~lib.rs]

['bcrypt'][bcrypt~wikipedia]↗ is a password-hashing function. Besides incorporating a salt to protect against rainbow table attacks, ['bcrypt'][bcrypt~wikipedia]↗ is an adaptive function: over time, the iteration count can be increased to make it slower, so it remains resistant to brute-force [search][p~search] attacks even with increasing computation power. ['bcrypt'][bcrypt~wikipedia]↗ is not a key derivation function (KDF). For example, [`bcrypt`][c~bcrypt~docs]↗{{hi:bcrypt}} cannot be used to [derive][p~derive] a 512-bit key from a password.

[`bcrypt`][c~bcrypt~docs]↗{{hi:bcrypt}}

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/password_hashing/bcrypt.rs:example}}
```

## Salt and Hash a Password with PBKDF2 {#pbkdf2}

[![ring][c~ring~docs~badge]][c~ring~docs] [![ring~crates.io][c~ring~crates.io~badge]][c~ring~crates.io] [![ring~repo][c~ring~repo~badge]][c~ring~repo] [![ring~lib.rs][c~ring~lib.rs~badge]][c~ring~lib.rs]{{hi:ring}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Rand}}{{hi:Rsa}}{{hi:ECC}} [![data-encoding][c~data-encoding~docs~badge]][c~data-encoding~docs] [![data-encoding~crates.io][c~data-encoding~crates.io~badge]][c~data-encoding~crates.io] [![data-encoding~repo][c~data-encoding~repo~badge]][c~data-encoding~repo] [![data-encoding~lib.rs][c~data-encoding~lib.rs~badge]][c~data-encoding~lib.rs]{{hi:data-encoding}}{{hi:Hex}}{{hi:No_std}}{{hi:Base64}}{{hi:Base32}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

Uses [`ring::pbkdf2`][c~ring::pbkdf2~docs]↗{{hi:ring::pbkdf2}} to hash a salted password{{hi:Salted passwords}} using the PBKDF2{{hi:PBKDF2}} key derivation function [`ring::pbkdf2::derive`][c~ring::pbkdf2::derive~docs]↗{{hi:ring::pbkdf2::derive}}.
Verifies the hash{{hi:Hashing}} is correct with [`ring::pbkdf2::verify`][c~ring::pbkdf2::verify~docs]↗{{hi:ring::pbkdf2::verify}}.
The salt is generated using [`ring::rand::SecureRandom::fill`][c~ring::rand::SecureRandom::fill~docs]↗{{hi:ring::rand::SecureRandom::fill}} which fills the salt byte array with securely generated random numbers.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/password_hashing/pbkdf2.rs:example}}
```

For more [algorithms][p~algorithms], see [Rust Crypto Password Hashes][rustcrypto-password-hashes~repo]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[password_hashing: write](https://github.com/john-cd/rust_howto/issues/275)
</div>
