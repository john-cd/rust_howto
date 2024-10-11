# Hashing

{{#include hashing.incl.md}}

## Calculate the SHA-256 digest of a file

[![ring][c-ring-badge]][c-ring]  [![data_encoding][c-data_encoding-badge]][c-data_encoding]  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]

Writes some data to a file, then calculates the SHA-256{{hi:SHA-256}} [`digest::Digest`][c-digest::Digest]{{hi:digest::Digest}}⮳ of the file's contents using [`digest::Context`][c-digest::Context]{{hi:digest::Context}}⮳

```rust,editable
{{#include ../../../deps/tests/sha-digest.rs}}
```

## Sign and verify a message with HMAC digest

[![ring][c-ring-badge]][c-ring]  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]

Uses [`ring::hmac`][c-ring::hmac]{{hi:ring::hmac}}⮳ to creates a [`ring::signature::Signature`][c-ring::signature::Signature]{{hi:ring::signature::Signature}}⮳ of a string then verifies the signature{{hi:signature}} is correct.

```rust,editable
{{#include ../../../deps/tests/hmac.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
