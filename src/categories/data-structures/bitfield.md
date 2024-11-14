# Custom

{{#include bitfield.incl.md}}

## Define and operate on a type represented as a bitfield {#bitfield}

[![bitflags][c-bitflags-badge]][c-bitflags]{{hi:bitflags}}  [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No std}}  [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

[![bitflags][c-bitflags-badge]][c-bitflags]
[![bitflags-crates.io][c-bitflags-crates.io-badge]][c-bitflags-crates.io]
[![bitflags-github][c-bitflags-github-badge]][c-bitflags-github]
[![bitflags-lib.rs][c-bitflags-lib.rs-badge]][c-bitflags-lib.rs]

Creates type-safe bitfield{{hi:bitfield}} type `MyFlags` with help of [`bitflags::bitflags`][c-bitflags::bitflags]{{hi:bitflags::bitflags}}⮳ macro and implements elementary `clear` operation as well as [`std::fmt::Display`][c-std::fmt::Display]{{hi:std::fmt::Display}}⮳ trait for it. Subsequently, shows basic bitwise operations{{hi:Bitwise operations}} and formatting.

```rust
{{#include ../../../deps/tests/cats/data_structures/bitfield.rs:example}}
```

## Flagset {#flagset}

[![flagset][c-flagset-badge]][c-flagset]{{hi:flagset}}
[![flagset-crates.io][c-flagset-crates.io-badge]][c-flagset-crates.io]
[![flagset-github][c-flagset-github-badge]][c-flagset-github]
[![flagset-lib.rs][c-flagset-lib.rs-badge]][c-flagset-lib.rs]

FlagSet is a new, ergonomic approach to handling flags that combines the best of existing crates like `bitflags` and `enumflags` without their downsides.

```rust
{{#include ../../../deps/tests/cats/data_structures/flagset.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO review
</div>
