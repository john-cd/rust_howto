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

{{#badges zeroize cryptography=Cryptography memory-management="Memory management" os="Operating systems" no-std="No standard library"}}{{hi:Memory}}{{hi:Volatile}}{{hi:Secure}}{{hi:Memset}}{{hi:Zero}}

The [`zeroize`][c~zeroize~docs]↗{{hi:zeroize}} crate clears the value's memory using writes intended not to be optimized away. It cannot erase copies made elsewhere, values in registers, or data already written to logs, swap, or storage. Minimize copies and avoid logging secrets; consider the crate's zeroize-on-drop wrappers for owned secret values.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/utilities/zeroize.rs:example}}
```

## `subtle` {#subtle}

[![subtle~website][c~subtle~website~badge]][c~subtle~website] {{#badges subtle}} {{hi:Crypto}} {{hi:Cryptography}} {{hi:Utilities}} {{hi:Constant-time}} {{#categories cryptography=Cryptography,no-std="No standard library"}}

[`subtle`][c~subtle~docs]↗{{hi:subtle}} provides pure-Rust traits and utilities for writing constant-time cryptographic code. In cryptography, timing attacks exploit variations in execution time to infer secret information. Constant-time code aims to eliminate these variations, ensuring that the execution time of an operation is independent of the secret data.

It consists of a [`Choice`][c~subtle::Choice~docs]↗{{hi:subtle::Choice}} type, and a collection of [traits][p~traits] using `Choice` instead of bool, which are intended to execute in constant-time. The Choice type is a wrapper around a u8 that holds a 0 or 1.

The [`subtle`][c~subtle~docs]↗{{hi:subtle}} crate is a low-level library and doesn't implement cryptographic algorithms itself. Its operations alone do not make a whole program constant-time: surrounding branches, memory accesses, and the target platform also matter. Prefer reviewed cryptographic implementations to building custom protocols.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/utilities/subtle.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[memory-management | Memory Management]].
- [[cryptography | Cryptography]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
