# Typecasts

{{#include typecasts.incl.md}}

## `bytemuck` {#bytemuck}

[![bytemuck][c-bytemuck-badge]][c-bytemuck]{{hi:bytemuck}}
[![bytemuck-crates.io][c-bytemuck-crates.io-badge]][c-bytemuck-crates.io]
[![bytemuck-github][c-bytemuck-github-badge]][c-bytemuck-github]
[![bytemuck-lib.rs][c-bytemuck-lib.rs-badge]][c-bytemuck-lib.rs]
[![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}
[![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

[`bytemuck`][c-bytemuck]⮳{{hi:bytemuck}}

```rust,editable
{{#include ../../../crates/cats/encoding/tests/typecasts/bytemuck.rs:example}}
```

## `zerocopy` {#zerocopy}

[![zerocopy][c-zerocopy-badge]][c-zerocopy]{{hi:zerocopy}}
[![zerocopy-crates.io][c-zerocopy-crates.io-badge]][c-zerocopy-crates.io]
[![zerocopy-github][c-zerocopy-github-badge]][c-zerocopy-github]
[![zerocopy-lib.rs][c-zerocopy-lib.rs-badge]][c-zerocopy-lib.rs]
[![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}
[![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing tools}}
[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}
[![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}}
[![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}}
[![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

[`zerocopy`][c-zerocopy]⮳{{hi:zerocopy}} makes zero-cost memory manipulation safe. It provides a set of traits and utilities to work with types that can be safely interpreted as byte slices.

- No data copying: Zero-copy avoids unnecessary data copying by directly interpreting the [memory][p-memory] of one data structure as another.
- [Performance][p-performance]: Eliminating data copying can significantly improve [performance][p-performance], especially in scenarios involving frequent data transfers between different [memory][p-memory] regions (e.g., network I/O, inter-process communication).
- Safety: The zerocopy crate provides mechanisms to ensure safe and correct zero-copy operations.

Zerocopy is often used in [network programming][p-network-programming], where high [performance][p-performance] and low [memory][p-memory] overhead are critical, or image handling.

```rust,editable
{{#include ../../../crates/cats/encoding/tests/typecasts/zerocopy.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

[typecasts: write (P1)](https://github.com/john-cd/rust_howto/issues/354)

</div>
