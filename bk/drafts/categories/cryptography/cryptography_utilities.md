# Cryptographic Utilities

{{#include cryptography_utilities.incl.md}}

## `zeroize` {#zeroize}

[![zeroize][c~zeroize~docs~badge]][c~zeroize~docs] [![zeroize~crates.io][c~zeroize~crates.io~badge]][c~zeroize~crates.io] [![zeroize~github][c~zeroize~github~badge]][c~zeroize~github] [![zeroize~lib.rs][c~zeroize~lib.rs~badge]][c~zeroize~lib.rs]{{hi:zeroize}}{{hi:Memory}}{{hi:Volatile}}{{hi:Secure}}{{hi:Memset}}{{hi:Zero}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~os][cat~os~badge]][cat~os]{{hi:Operating systems}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

"Zeroize" refers to the process of securely erasing sensitive data, such as cryptographic keys, passwords, or any other confidential information, from memory or storage. The goal is to ensure that the data cannot be recovered or reconstructed, preventing unauthorized access.

The [`zeroize`][c~zeroize~docs]⮳{{hi:zeroize}} crate securely clear secrets from memory. It guarantees that memory is zeroed, using an operation that will not be optimized away by the compiler. It is a portable pure-Rust implementation that works everywhere, even with WASM.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/utilities/zeroize.rs:example}}
```

## `subtle` {#subtle}

[![subtle~website][c~subtle~website~badge]][c~subtle~website] [![subtle][c~subtle~docs~badge]][c~subtle~docs] [![subtle~crates.io][c~subtle~crates.io~badge]][c~subtle~crates.io] [![subtle~github][c~subtle~github~badge]][c~subtle~github] [![subtle~lib.rs][c~subtle~lib.rs~badge]][c~subtle~lib.rs]{{hi:subtle}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Utilities}}{{hi:Constant-time}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`subtle`][c~subtle~docs]⮳{{hi:subtle}} provides pure-Rust traits and utilities for writing constant-time cryptographic code. In cryptography, timing attacks exploit variations in execution time to infer secret information. Constant-time code aims to eliminate these variations, ensuring that the execution time of an operation is independent of the secret data.

It consists of a `Choice` type, and a collection of [traits][p~traits] using `Choice` instead of bool, which are intended to execute in constant-time. The Choice type is a wrapper around a u8 that holds a 0 or 1.

The [`subtle`][c~subtle~docs]⮳{{hi:subtle}} crate is a low-level library and doesn't implement cryptographic algorithms itself. Instead, it provides the building blocks for developers to implement cryptographic algorithms in a way that minimizes the risk of timing attacks. It's typically used in low-level cryptographic libraries or when implementing custom cryptographic protocols.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/utilities/subtle.rs:example}}
```

## Related Topics {#related-topics}

- [[memory-management | Memory Management]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1180)
</div>
