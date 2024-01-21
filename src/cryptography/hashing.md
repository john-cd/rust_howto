# Hashing

## Calculate the SHA-256 digest of a file

[![ring-badge]][ring] [![data-encoding-badge]][data-encoding] [![cat-cryptography-badge]][cat-cryptography]

Writes some data to a file, then calculates the SHA-256 [`digest::Digest`][digest::Digest] of the file's contents using `[digest::Context]`

```rust,editable
{{#include ../../deps/examples/sha-digest.rs}}
```

[digest::Context]: https://briansmith.org/rustdoc/ring/digest/struct.Context.html
[digest::Digest]: https://briansmith.org/rustdoc/ring/digest/struct.Digest.html

## Sign and verify a message with HMAC digest

[![ring-badge]][ring] [![cat-cryptography-badge]][cat-cryptography]

Uses [`ring::hmac`][ring::hmac] to creates a [`hmac::Signature`][hmac::Signature] of a string then verifies the signature is correct.

```rust,editable
{{#include ../../deps/examples/hmac.rs}}
```

[hmac::Signature]: https://briansmith.org/rustdoc/ring/hmac/struct.Signature.html
[ring::hmac]: https://briansmith.org/rustdoc/ring/hmac/
{{#include ../refs/link-refs.md}}
