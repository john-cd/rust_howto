## Sign and verify a message with HMAC digest

[![ring-badge]][ring] [![cat-cryptography-badge]][cat-cryptography]

Uses [`ring::hmac`] to creates a [`hmac::Signature`] of a string then verifies the signature is correct.

```rust,editable
{#include ../../../deps/examples/hmac.rs}
```

[`hmac::Signature`]: https://briansmith.org/rustdoc/ring/hmac/struct.Signature.html
[`ring::hmac`]: https://briansmith.org/rustdoc/ring/hmac/
