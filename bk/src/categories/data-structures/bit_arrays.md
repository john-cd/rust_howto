<!-- TODO Reviewed ??-??-2026 -->

# Bit Arrays

{{#include bit_arrays.incl.md}}

A bit array (also known as a bitmap, bit set, bit string, or bit vector) is a data structure that stores adjacent bits for specific purposes, so that any single bit or group of bits within the structure can be set or inspected.

It is a way to represent a set of boolean flags or values with a limited range in a compact manner, using individual bits or small group of bits within an integer or an array of integers. Bit arrays save memory and potentially improve performance.

## Work with C-style Flags using `bitflags` {#bitflags}

{{#badges bitflags no-std="No standard library"}}{{hi:Bit}}{{hi:bitflags}}{{hi:Bitmask}}{{hi:Flags}}

[`bitflags`][c~bitflags~docs]↗{{hi:bitflags}} generates ergonomic types for C-style flags. You can use `bitflags` to provide user-friendly bindings to C APIs, where flags may or may not be fully known in advance, with string parsing and formatting support.

A flag is a set of bits in a bits type (fixed-width unsigned integers) that may have a unique name. Bits are not required to be exclusive to a flag. Bits are not required to be contiguous.

The following example creates a type `MyFlags` with the help of the [`bitflags::bitflags`][c~bitflags::bitflags~docs]↗{{hi:bitflags::bitflags}} macro and subsequently shows basic bitwise operations{{hi:Bitwise operations}} and formatting:

```rust,editable
{{#include ../../../crates/cats/data_structures/examples/bitfield/bitflags.rs:example}}
```

## Work with C-style Flags using `flagset` {#flagset}

{{#badges flagset}}{{hi:Bitflags}}{{hi:Enum}}{{hi:Enumflags}}{{hi:Flags}}

[`flagset`][c~flagset~docs]↗{{hi:flagset}} is an alternative to crates like [`bitflags`][c~bitflags~docs]↗{{hi:bitflags}} and [`enumflags`][c~enumflags~docs]↗{{hi:enumflags}}. A `flagset` flag definition looks just like a regular enumeration, with the addition of the field-size type:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/bitfield/flagset.rs:example}}
```

## Create and Manipulate Bit Vectors with `bitvec` {#bitvec}

[![bitvec~website][c~bitvec~website~badge]][c~bitvec~website] [![bitvec][c~bitvec~docs~badge]][c~bitvec~docs] [![bitvec~crates.io][c~bitvec~crates.io~badge]][c~bitvec~crates.io] [![bitvec~repo][c~bitvec~repo~badge]][c~bitvec~repo] [![bitvec~lib.rs][c~bitvec~lib.rs~badge]][c~bitvec~lib.rs]{{hi:bitvec}}{{hi:Bitstream}}{{hi:Bitvector}}{{hi:Bitmap}}{{hi:Bitfields}}{{hi:bitvec}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`bitvec`][c~bitvec~docs]↗{{hi:bitvec}} provides efficient storage and manipulation of bit vectors. It addresses memory by bits, for packed collections and bitfields. This example demonstrates the usage of the `bitvec` crate:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/bitfield/bitvec.rs:example}}
```

## Track Visited Nodes with `fixedbitset` {#fixedbitset}

{{#badges fixedbitset data-structures="Data structures"}}{{hi:Bit set}}{{hi:Visited set}}{{hi:Graph}}

[`fixedbitset`][c~fixedbitset~docs]↗{{hi:fixedbitset}} is a bit set with a size fixed at creation. It uses one bit per element, where a `Vec<bool>` uses a byte, and offers fast set operations such as `union_with`. The example uses it as the visited set of a graph walk. Choose [`bitvec`][c~bitvec~docs]↗{{hi:bitvec}} instead for arbitrary bit-level views or a growable vector.

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/bitfield/fixedbitset.rs:example}}
```

## Store Large Sets of Integers with `roaring` {#roaring}

{{#badges roaring data-structures="Data structures"}}{{hi:Bitmap}}{{hi:Data-structure}}{{hi:roaring}}

Bitmaps are commonly used as fast [data structures][p~category-data-structures]{{hi:data-structures}}, for example as indices for databases, search engines, and big data processing systems.

Unfortunately, they can use too much memory. To compensate, [`roaring`][c~roaring~docs]↗{{hi:roaring}} implements compressed bitmap data structures. They efficiently store and manipulate large sets of integers:

```rust,editable,noplayground
{{#include ../../../crates/cats/data_structures/examples/bitfield/roaring.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[vectors | Vectors]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  accuracy review: data structures
</div>
