# Bitfields

{{#include bitfield.incl.md}}

A bitfield is a data structure that efficiently stores a sequence of bits. It is a way to represent a set of boolean flags or options in a compact manner, using individual bits within an integer or an array of integers. Bitfields save memory and potentially improve performance.

## Define and Operate on a Type Represented as a Bitfield {#bitfield}

[![bitflags][c~bitflags~docs~badge]][c~bitflags~docs] [![bitflags~crates.io][c~bitflags~crates.io~badge]][c~bitflags~crates.io] [![bitflags~repo][c~bitflags~repo~badge]][c~bitflags~repo] [![bitflags~lib.rs][c~bitflags~lib.rs~badge]][c~bitflags~lib.rs]{{hi:bitflags}}{{hi:Bit}}{{hi:bitflags}}{{hi:Bitmask}}{{hi:Flags}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`bitflags`][c~bitflags~docs]↗{{hi:bitflags}} offers a macro to generate structures which behave like bitflags. It creates type-safe bitfield{{hi:Bitfield}} type `MyFlags` with help of [`bitflags::bitflags`][c~bitflags::bitflags~docs]↗{{hi:bitflags::bitflags}} macro and implements elementary [`clear`][c~clear~docs]↗{{hi:clear}}{{hi:clear}} operation as well as [`std::fmt::Display`][c~std::fmt::Display~docs]↗{{hi:std::fmt::Display}} trait for it. Subsequently, shows basic bitwise operations{{hi:Bitwise operations}} and formatting.

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/bitfield/bitfield.rs:example}}
```

## `flagset` {#flagset}

[![flagset][c~flagset~docs~badge]][c~flagset~docs]{{hi:flagset}}
[![flagset~crates.io][c~flagset~crates.io~badge]][c~flagset~crates.io]
[![flagset~repo][c~flagset~repo~badge]][c~flagset~repo]
[![flagset~lib.rs][c~flagset~lib.rs~badge]][c~flagset~lib.rs]

A [`flagset`][c~flagset~docs]↗{{hi:flagset}} refers to a bitfield used to represent a set of boolean flags or options, where each bit has a specific meaning.

[`flagset`][c~flagset~docs]↗{{hi:flagset}} is a ergonomic approach to handling flags that combines the best of existing crates like [`bitflags`][c~bitflags~docs]↗{{hi:bitflags}} and [`enumflags`][c~enumflags~docs]↗{{hi:enumflags}} without their downsides.

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/bitfield/flagset.rs:example}}
```

## Create Bit Vectors with `bitvec` {#bitvec}

[![bitvec~website][c~bitvec~website~badge]][c~bitvec~website] [![bitvec][c~bitvec~docs~badge]][c~bitvec~docs] [![bitvec~crates.io][c~bitvec~crates.io~badge]][c~bitvec~crates.io] [![bitvec~repo][c~bitvec~repo~badge]][c~bitvec~repo] [![bitvec~lib.rs][c~bitvec~lib.rs~badge]][c~bitvec~lib.rs]{{hi:bitvec}}{{hi:Bitstream}}{{hi:Bitvector}}{{hi:Bitmap}}{{hi:Bitfield}}{{hi:bitvec}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`bitvec`][c~bitvec~docs]↗{{hi:bitvec}} provides efficient storage and manipulation of bit vectors. It addresses memory by bits, for packed collections and bitfields.

This example demonstrates the usage of the `bitvec` crate for bit manipulation in Rust:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/bitfield/bitvec.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[bitfield: edit / review](https://github.com/john-cd/rust_howto/issues/279)

- [roaring][c~roaring~crates.io]↗.

</div>
