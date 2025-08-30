# Bit Arrays

{{#include bitarrays.incl.md}}

A bit array (also known as bit map, bit set, bit string, or bit vector) is an data structure that allocates one or more adjacent bits for specific purposes, so that any single bit or group of bits within the structure can be set or inspected.

It is a way to represent a set of boolean flags or values with a limited range in a compact manner, using individual bits or small group of bits within an integer or an array of integers. Bit arrays save memory and potentially improve performance.

## Work with C-style Flags using `bitflags` {#bitflags}

[![bitflags][c~bitflags~docs~badge]][c~bitflags~docs] [![bitflags~crates.io][c~bitflags~crates.io~badge]][c~bitflags~crates.io] [![bitflags~repo][c~bitflags~repo~badge]][c~bitflags~repo] [![bitflags~lib.rs][c~bitflags~lib.rs~badge]][c~bitflags~lib.rs]{{hi:bitflags}}{{hi:Bit}}{{hi:bitflags}}{{hi:Bitmask}}{{hi:Flags}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`bitflags`][c~bitflags~docs]↗{{hi:bitflags}} generates ergonomic types for C-style flags. You can use `bitflags` to provide user-friendly bindings to C APIs, where flags may or may not be fully known in advance, with string parsing and formatting support.

A flag is a set of bits in a bits type (fixed-width unsigned integers) that may have a unique name. Bits are not required to be exclusive to a flag. Bits are not required to be contiguous.

The following example creates a type `MyFlags` with the help of the [`bitflags::bitflags`][c~bitflags::bitflags~docs]↗{{hi:bitflags::bitflags}} macro and subsequently shows basic bitwise operations{{hi:Bitwise operations}} and formatting:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/bitfield/bitflags.rs:example}}
```

## Work with C-style Flags using `flagset` {#flagset}

[![flagset][c~flagset~docs~badge]][c~flagset~docs] [![flagset~crates.io][c~flagset~crates.io~badge]][c~flagset~crates.io] [![flagset~repo][c~flagset~repo~badge]][c~flagset~repo] [![flagset~lib.rs][c~flagset~lib.rs~badge]][c~flagset~lib.rs]{{hi:flagset}}{{hi:Bitflags}}{{hi:Enum}}{{hi:Enumflags}}{{hi:Flags}}

[`flagset`][c~flagset~docs]↗{{hi:flagset}} is an alternative to crates like [`bitflags`][c~bitflags~docs]↗{{hi:bitflags}} and [`enumflags`][c~enumflags~docs]↗{{hi:enumflags}}. A `flagset` flag definition looks just like a regular enumeration, with the addition of the field-size type:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/bitfield/flagset.rs:example}}
```

## Create and Manipulate Bit Vectors with `bitvec` {#bitvec}

[![bitvec~website][c~bitvec~website~badge]][c~bitvec~website] [![bitvec][c~bitvec~docs~badge]][c~bitvec~docs] [![bitvec~crates.io][c~bitvec~crates.io~badge]][c~bitvec~crates.io] [![bitvec~repo][c~bitvec~repo~badge]][c~bitvec~repo] [![bitvec~lib.rs][c~bitvec~lib.rs~badge]][c~bitvec~lib.rs]{{hi:bitvec}}{{hi:Bitstream}}{{hi:Bitvector}}{{hi:Bitmap}}{{hi:Bitfields}}{{hi:bitvec}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`bitvec`][c~bitvec~docs]↗{{hi:bitvec}} provides efficient storage and manipulation of bit vectors. It addresses memory by bits, for packed collections and bitfields. This example demonstrates the usage of the `bitvec` crate:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/bitfield/bitvec.rs:example}}
```

## Store Large Sets of Integers with `roaring` {#roaring}

[![roaring][c~roaring~docs~badge]][c~roaring~docs] [![roaring~crates.io][c~roaring~crates.io~badge]][c~roaring~crates.io] [![roaring~repo][c~roaring~repo~badge]][c~roaring~repo] [![roaring~lib.rs][c~roaring~lib.rs~badge]][c~roaring~lib.rs]{{hi:roaring}}{{hi:Bitmap}}{{hi:Data-structure}}{{hi:roaring}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

Bitmaps are commonly used as fast data structures, for example as indices for databases, search engines, and big data processing systems.

Unfortunately, they can use too much memory. To compensate, [`roaring`][c~roaring~docs]↗{{hi:roaring}} implements compressed bitmap data structures. They efficiently store and manipulate large sets of integers:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/bitfield/roaring.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
