# Cryptographic Utilities

{{#include utilities.incl.md}}

## `zeroize` {#zeroize}

[![zeroize][c-zeroize-badge]][c-zeroize] [![zeroize-crates.io][c-zeroize-crates.io-badge]][c-zeroize-crates.io] [![zeroize-github][c-zeroize-github-badge]][c-zeroize-github] [![zeroize-lib.rs][c-zeroize-lib.rs-badge]][c-zeroize-lib.rs]{{hi:zeroize}}{{hi:Memory}}{{hi:Volatile}}{{hi:Secure}}{{hi:Memset}}{{hi:Zero}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}} [![cat-os][cat-os-badge]][cat-os]{{hi:Operating systems}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[`zeroize`][c-zeroize]⮳{{hi:zeroize}} securely clear secrets from memory. It guarantees that memory is zeroed, using an operation that will not be optimized away by the compiler. It is a portable pure-Rust implementation that works everywhere, even with WASM.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/utilities/zeroize.rs:example}}
```

## `subtle` {#subtle}

[![subtle-website][c-subtle-website-badge]][c-subtle-website] [![subtle][c-subtle-badge]][c-subtle] [![subtle-crates.io][c-subtle-crates.io-badge]][c-subtle-crates.io] [![subtle-github][c-subtle-github-badge]][c-subtle-github] [![subtle-lib.rs][c-subtle-lib.rs-badge]][c-subtle-lib.rs]{{hi:subtle}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Utilities}}{{hi:Constant-time}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[`subtle`][c-subtle]⮳{{hi:subtle}} provides pure-Rust traits and utilities for constant-time cryptographic implementations.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/utilities/subtle.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P2 write
</div>
