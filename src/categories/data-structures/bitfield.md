# Custom

{{#include bitfield.incl.md}}

## Define and operate on a type represented as a bitfield {#bitfield}

[![bitflags][c-bitflags-badge]][c-bitflags] [![bitflags-crates.io][c-bitflags-crates.io-badge]][c-bitflags-crates.io] [![bitflags-github][c-bitflags-github-badge]][c-bitflags-github] [![bitflags-lib.rs][c-bitflags-lib.rs-badge]][c-bitflags-lib.rs]{{hi:bitflags}}{{hi:Bit}}{{hi:bitflags}}{{hi:Bitmask}}{{hi:Flags}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

`bitflags` offers a macro to generate structures which behave like bitflags. It creates type-safe bitfield{{hi:bitfield}} type `MyFlags` with help of [`bitflags::bitflags`][c-bitflags::bitflags]{{hi:bitflags::bitflags}}⮳ macro and implements elementary `clear` operation as well as [`std::fmt::Display`][c-std::fmt::Display]{{hi:std::fmt::Display}}⮳ trait for it. Subsequently, shows basic bitwise operations{{hi:Bitwise operations}} and formatting.

```rust,editable
{{#include ../../../deps/tests/categories/data_structures/bitfield.rs:example}}
```

## `flagset` {#flagset}

[![flagset][c-flagset-badge]][c-flagset]{{hi:flagset}}
[![flagset-crates.io][c-flagset-crates.io-badge]][c-flagset-crates.io]
[![flagset-github][c-flagset-github-badge]][c-flagset-github]
[![flagset-lib.rs][c-flagset-lib.rs-badge]][c-flagset-lib.rs]

FlagSet is a new, ergonomic approach to handling flags that combines the best of existing crates like `bitflags` and `enumflags` without their downsides.

```rust,editable,noplayground
{{#include ../../../deps/tests/categories/data_structures/flagset.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[bitfield: review (P1)](https://github.com/john-cd/rust_howto/issues/279)
TODO P0 The flagset crate is not available in the Rust Playground.
</div>
