<!-- TODO Reviewed ??-??-2026 -->

# Cryptographic Utilities

{{#include cryptography_utilities.incl.md}}

These helpers support cryptographic applications but do not replace a reviewed cryptographic protocol or algorithm. Use operating-system randomness for secret or nonce bytes, and treat Base64 and hexadecimal only as text encodings; neither provides encryption.

## Obtain Random Bytes from the Operating System with `getrandom` {#getrandom}

[![getrandom][c~getrandom~docs~badge]][c~getrandom~docs] [![getrandom~crates.io][c~getrandom~crates.io~badge]][c~getrandom~crates.io] [![getrandom~repo][c~getrandom~repo~badge]][c~getrandom~repo]{{hi:getrandom}}{{hi:randomness}}{{hi:operating system}}

The [`getrandom`][c~getrandom~docs]↗ crate fills byte buffers from the operating system's random source. Use it for cryptographic random bytes; the [`rand`][c~rand~docs]↗ crate is a general-purpose random-number library and is not primarily a cryptographic library.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/utilities/getrandom_example.rs:example}}
```

## Encode Bytes as Base64 {#base64}

[![base64][c~base64~docs~badge]][c~base64~docs] [![base64~crates.io][c~base64~crates.io~badge]][c~base64~crates.io] [![base64~repo][c~base64~repo~badge]][c~base64~repo]{{hi:Base64}}{{hi:text encoding}}

The [`base64`][c~base64~docs]↗ crate converts arbitrary bytes into Base64 text and decodes that text back into bytes. This is a transport encoding, not encryption, and it does not protect confidential data.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/utilities/base64_example.rs:example}}
```

## Encode Bytes as Hexadecimal {#hex}

[![hex][c~hex~docs~badge]][c~hex~docs] [![hex~crates.io][c~hex~crates.io~badge]][c~hex~crates.io] [![hex~repo][c~hex~repo~badge]][c~hex~repo]{{hi:hexadecimal}}{{hi:text encoding}}

The [`hex`][c~hex~docs]↗ crate encodes bytes as pairs of hexadecimal digits and decodes them back into bytes. Hexadecimal is useful for readable diagnostics and wire formats, but it is larger than the original binary data and is not encryption.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/utilities/hex_example.rs:example}}
```

## `zeroize` {#zeroize}

[![zeroize][c~zeroize~docs~badge]][c~zeroize~docs] [![zeroize~crates.io][c~zeroize~crates.io~badge]][c~zeroize~crates.io] [![zeroize~repo][c~zeroize~repo~badge]][c~zeroize~repo] [![zeroize~lib.rs][c~zeroize~lib.rs~badge]][c~zeroize~lib.rs]{{hi:zeroize}}{{hi:Memory}}{{hi:Volatile}}{{hi:Secure}}{{hi:Memset}}{{hi:Zero}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~os][cat~os~badge]][cat~os]{{hi:Operating systems}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

"Zeroize" refers to the process of securely erasing sensitive data, such as cryptographic keys, passwords, or any other confidential information, from memory or storage. The goal is to ensure that the data cannot be recovered or reconstructed, preventing unauthorized access.

The [`zeroize`][c~zeroize~docs]↗{{hi:zeroize}} crate securely clear secrets from memory. It guarantees that memory is zeroed, using an operation that will not be optimized away by the compiler. It is a portable pure-Rust implementation that works everywhere, even with WASM.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/utilities/zeroize.rs:example}}
```

## `subtle` {#subtle}

[![subtle~website][c~subtle~website~badge]][c~subtle~website] [![subtle][c~subtle~docs~badge]][c~subtle~docs] [![subtle~crates.io][c~subtle~crates.io~badge]][c~subtle~crates.io] [![subtle~repo][c~subtle~repo~badge]][c~subtle~repo] [![subtle~lib.rs][c~subtle~lib.rs~badge]][c~subtle~lib.rs]{{hi:subtle}}{{hi:Crypto}}{{hi:Cryptography}}{{hi:Utilities}}{{hi:Constant-time}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`subtle`][c~subtle~docs]↗{{hi:subtle}} provides pure-Rust traits and utilities for writing constant-time cryptographic code. In cryptography, timing attacks exploit variations in execution time to infer secret information. Constant-time code aims to eliminate these variations, ensuring that the execution time of an operation is independent of the secret data.

It consists of a [`Choice`][c~subtle::Choice~docs]↗{{hi:subtle::Choice}} type, and a collection of [traits][p~traits] using `Choice` instead of bool, which are intended to execute in constant-time. The Choice type is a wrapper around a u8 that holds a 0 or 1.

The [`subtle`][c~subtle~docs]↗{{hi:subtle}} crate is a low-level library and doesn't implement cryptographic algorithms itself. Instead, it provides the building blocks for developers to implement cryptographic algorithms in a way that minimizes the risk of timing attacks. It's typically used in low-level cryptographic libraries or when implementing custom cryptographic protocols.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/utilities/subtle.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[memory-management | Memory Management]].
- [[cryptography | Cryptography]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
