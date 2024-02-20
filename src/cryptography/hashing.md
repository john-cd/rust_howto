# Hashing

## Calculate the SHA-256 digest of a file

[![ring][ring-badge]][ring]  [![data-encoding][data-encoding-badge]][data-encoding]  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]

Writes some data to a file, then calculates the SHA-256 [`digest::Digest`][digest::Digest] of the file's contents using [`digest::Context`][digest::Context]

```rust,editable
{{#include ../../deps/tests/sha-digest.rs}}
```

## Sign and verify a message with HMAC digest

[![ring][ring-badge]][ring]  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]

Uses [`ring::hmac`][ring::hmac] to creates a [`Signature`][ring::signature::Signature] of a string then verifies the signature is correct.

```rust,editable
{{#include ../../deps/tests/hmac.rs}}
```

{{#include ../refs/link-refs.md}}
