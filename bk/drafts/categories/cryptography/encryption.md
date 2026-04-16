# Encryption

{{#include encryption.incl.md}}

Encryption transforms readable data (plaintext) into an unreadable format (ciphertext) to protect its confidentiality. Decryption reverses this process, restoring the original plaintext from the ciphertext using a secret key.

## Use the RSA Algorithm {#rsa}

[![rsa][c~rsa~docs~badge]][c~rsa~docs] [![rsa~crates.io][c~rsa~crates.io~badge]][c~rsa~crates.io] [![rsa~repo][c~rsa~repo~badge]][c~rsa~repo] [![rsa~lib.rs][c~rsa~lib.rs~badge]][c~rsa~lib.rs]{{hi:rsa}}{{hi:Crypto}}{{hi:Security}}{{hi:Encryption}}{{hi:rsa}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}}

RSA (Rivest-Shamir-Adleman) is one of the most widely used public-key cryptosystems for secure data transmission. It relies on the mathematical properties of prime numbers to generate a pair of keys: a public key and a private key. The public key is used to encrypt data, which can only be decrypted by the corresponding private key, ensuring that only the intended recipient can access the information.

[`rsa`][c~rsa~docs]↗{{hi:rsa}} is a pure Rust RSA implementation. It generates RSA key pairs (public and private keys), encrypts data with the public key, and decrypts data with the corresponding private key. The crate also supports digital signatures, enables signing data with the private key and verifying the [signature][p~signature] with the public key. [`rsa`][c~rsa~docs]↗{{hi:rsa}} offers various functionalities for working with RSA keys, including loading and saving keys in different formats (like PEM), and provides implementations of different padding schemes (like PKCS#1 v1.5 padding and PSS padding).

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/rsa.rs:example}}
```

## `sodiumoxide` {#sodiumoxide}

[![sodiumoxide][c~sodiumoxide~docs~badge]][c~sodiumoxide~docs] [![sodiumoxide~crates.io][c~sodiumoxide~crates.io~badge]][c~sodiumoxide~crates.io] [![sodiumoxide~repo][c~sodiumoxide~repo~badge]][c~sodiumoxide~repo] [![sodiumoxide~lib.rs][c~sodiumoxide~lib.rs~badge]][c~sodiumoxide~lib.rs]{{hi:sodiumoxide}}{{hi:NaCl}}{{hi:Libsodium}}{{hi:Crypto}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}}

[`sodiumoxide`][c~sodiumoxide~docs]↗{{hi:sodiumoxide}} is a fast cryptographic library for Rust (bindings to [`libsodium`][libsodium]↗{{hi:libsodium}}). It implements encryption, decryption, signatures, password hashing, and more.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[encryption: expand](https://github.com/john-cd/rust_howto/issues/272)
</div>
