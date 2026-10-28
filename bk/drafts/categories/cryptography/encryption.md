<!-- TODO Reviewed ??-??-2026 -->

# Encryption

{{#include encryption.incl.md}}

Use symmetric authenticated encryption to protect application data. The shared key encrypts and authenticates a message; the nonce must follow the algorithm's requirements and must not be reused with the same key for AES-GCM or ChaCha20-Poly1305. Public-key algorithms are generally used to establish or wrap keys rather than encrypt large messages directly. Authentication failure must be treated as an error, and unauthenticated plaintext must not be used.

The AES-GCM and ChaCha20-Poly1305 examples use pure Rust implementations, so
compiling, testing, and linting them requires no additional native crypto
library. Running the key-generation examples requires the operating system's
secure random source; they do not need an external service.

## AES-256-GCM {#aes-256-gcm}

[`aes-gcm`][c~aes-gcm~docs]↗ provides AES-GCM authenticated encryption. The example generates a key and nonce from the operating system's random source, encrypts and decrypts a message, and tests that modified ciphertext is rejected.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/encryption/aes_gcm.rs:example}}
```

## ChaCha20-Poly1305 {#chacha20-poly1305}

[`chacha20poly1305`][c~chacha20poly1305~docs]↗ provides ChaCha20-Poly1305 authenticated encryption. The example verifies a round trip and confirms that modified tags and associated data fail authentication.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/encryption/chacha20poly1305.rs:example}}
```

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

- [[aead | Authenticated encryption and associated data]].
- [[password_hashing | Password hashing]].
- [[signature | Digital signatures]].
- [Cryptography category](index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
