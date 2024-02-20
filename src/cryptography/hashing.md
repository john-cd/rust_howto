# Hashing

## Calculate the SHA-256 digest of a file

[![ring-badge]][ring] [![data-encoding-badge]][data-encoding] [![cat-cryptography-badge]][cat-cryptography]

Writes some data to a file, then calculates the SHA-256 [`digest::Digest`][digest::Digest] of the file's contents using [`digest::Context`][digest::Context]

```rust,editable
{{#include ../../deps/examples/sha-digest.rs}}
```

## Sign and verify a message with HMAC digest

[![ring-badge]][ring] [![cat-cryptography-badge]][cat-cryptography]

Uses [`ring::hmac`][ring::hmac] to creates a [`Signature`][ring::signature::Signature] of a string then verifies the signature is correct.

```rust,editable
{{#include ../../deps/examples/hmac.rs}}
```

{{#include ../refs/link-refs.md}}
