# Hashing

{{#include hashing.incl.md}}

## Calculate the SHA-256 digest of a file

[![ring][ring-badge]][ring]  [![data-encoding][data-encoding-badge]][data-encoding]  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]

Writes some data to a file, then calculates the {{i:SHA-256}} [`{{i:digest::Digest}}`][digest::Digest]⮳ of the file's contents using [`{{i:digest::Context}}`][digest::Context]⮳

```rust,editable
{{#include ../../../deps/tests/sha-digest.rs}}
```

## Sign and verify a message with HMAC digest

[![ring][ring-badge]][ring]  [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]

Uses [`{{i:ring::hmac}}`][ring::hmac]⮳ to creates a [`{{i:Signature}}`][ring::signature::Signature]⮳ of a string then verifies the {{i:signature}} is correct.

```rust,editable
{{#include ../../../deps/tests/hmac.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
