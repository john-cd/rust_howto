# Password Hashing

{{#include password_hashing.incl.md}}

## argon2 {#argon2}

[![argon2][c-argon2-badge]][c-argon2]{{hi:argon2}}
[![argon2-crates.io][c-argon2-crates.io-badge]][c-argon2-crates.io]
[![argon2-github][c-argon2-github-badge]][c-argon2-github]
[![argon2-lib.rs][c-argon2-lib.rs-badge]][c-argon2-lib.rs]
[![cat-authentication][cat-authentication-badge]][cat-authentication]{{hi:Authentication}}
[![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pure Rust implementation of the Argon2 password hashing function with support for the Argon2d, Argon2i, and Argon2id algorithmic variants

## scrypt {#scrypt}

[![scrypt][c-scrypt-badge]][c-scrypt]{{hi:scrypt}}
[![scrypt-crates.io][c-scrypt-crates.io-badge]][c-scrypt-crates.io]
[![scrypt-github][c-scrypt-github-badge]][c-scrypt-github]
[![scrypt-lib.rs][c-scrypt-lib.rs-badge]][c-scrypt-lib.rs]
[![cat-authentication][cat-authentication-badge]][cat-authentication]{{hi:Authentication}}
[![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

The scrypt key derivation function is designed to be far more secure against hardware brute-force attacks than alternative functions such as PBKDF2 or bcrypt.

## bcrypt {#bcrypt}

[![bcrypt][c-bcrypt-badge]][c-bcrypt]{{hi:bcrypt}}
[![bcrypt-crates.io][c-bcrypt-crates.io-badge]][c-bcrypt-crates.io]
[![bcrypt-github][c-bcrypt-github-badge]][c-bcrypt-github]
[![bcrypt-lib.rs][c-bcrypt-lib.rs-badge]][c-bcrypt-lib.rs]

Hash and verify passwords.

For more algorithms, see Rust Crypto Password Hashes: https://github.com/RustCrypto/password-hashes#rustcrypto-password-hashes

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write

https://en.wikipedia.org/wiki/Key_derivation_function
</div>
