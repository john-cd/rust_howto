# Hashing

{{#include hashing.incl.md}}

## Calculate the SHA-256 digest of a file

[![ring][c-ring-badge]][c-ring]  [![data_encoding][c-data_encoding-badge]][c-data_encoding]  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]

Writes some data to a file, then calculates the {{i:SHA-256}} {{hi:digest::Digest}}[`digest::Digest`][c-digest::Digest]⮳ of the file's contents using {{hi:digest::Context}}[`digest::Context`][c-digest::Context]⮳

```rust,editable
{{#include ../../../deps/tests/sha-digest.rs}}
```

## Sign and verify a message with HMAC digest

[![ring][c-ring-badge]][c-ring]  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]

Uses {{hi:ring::hmac}}[`ring::hmac`][c-ring::hmac]⮳ to creates a {{hi:Signature}}[`Signature`][c-ring::signature::Signature]⮳ of a string then verifies the {{i:signature}} is correct.

```rust,editable
{{#include ../../../deps/tests/hmac.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
