# Data Types

{{#include data_types.incl.md}}

## Scalar Data Types {#scalar-data-types}

[![Rust by example - Primitives][book-rust-by-example-primitives-badge]][book-rust-by-example-primitives]{{hi:Primitives}}

Rust has several categories of scalar types: integers, floating-point numbers, Booleans, and unicode characters.

| Type Family | Types | Examples |
|---|---|---|
| Signed Integers | [`i8`][primitive-i8]{{hi:i8}}⮳, [`i16`][primitive-i16]{{hi:i16}}⮳, [`i32`][primitive-i32]{{hi:i32}}⮳, [`i64`][primitive-i64]{{hi:i64}}⮳, [`i128`][primitive-i128]{{hi:i128}}⮳, [`isize`][primitive-isize]{{hi:isize}}⮳. | `-8i8`, `-32i32`. |
| Unsigned Integers | [`u8`][primitive-u8]{{hi:u8}}⮳, [`u16`][primitive-u16]{{hi:u16}}⮳, [`u32`][primitive-u32]{{hi:u32}}⮳, [`u64`][primitive-u64]{{hi:u64}}⮳, [`u128`][primitive-u128]{{hi:u128}}⮳, [`usize`][primitive-usize]{{hi:usize}}⮳. | `6u8`. |
| Floating point | [`f32`][primitive-f32]{{hi:f32}}⮳, [`f64`][primitive-f64]{{hi:f64}}⮳. | `0.15`. |
| Boolean | [`bool`][primitive-bool]{{hi:bool}}⮳. | `true`, `false`. |
| Unicode Character | [`char`][primitive-char]⮳ | `let z: char = 'ℤ';` |

[`usize`][primitive-usize]{{hi:usize}}⮳ and [`isize`][primitive-isize]{{hi:isize}}⮳ are 32 or 64 bits, depending on the architecture of the computer.

The following illustrates the various scalar data types:

```rust,editable
{{#include ../../crates/language/tests/data_types/scalar_data_types.rs:example}}
```

See also 'Unit' and 'Never' below.

## Compound Data Types: Tuples and Arrays {#compound-types}

[![Rust by example - tuples][book-rust-by-example-tuples-badge]][book-rust-by-example-tuples] [![Rust by example - array][book-rust-by-example-array-badge]][book-rust-by-example-array]

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

| Type | Examples |
|---|---|
| Tuples | `let tup: (i32, f64, u8) = (500, 6.4, 1);`. Access via `let five_hundred = x.0;`. Destructuring via `let (x, y, z) = tup;`. |
| Arrays | `let a: [i32; 5] = [1, 2, 3, 4, 5];` allocated on the stack. Access via `let first = a[0];`. |

Both are fixed length. A [[vectors | Vector]] is a similar collection type provided by the [standard library][p-standard-library] that is allowed to grow or shrink in size.

The following provides examples of tuples and arrays:

```rust,editable
{{#include ../../crates/language/tests/data_types/compound_data_types.rs:example}}
```

## String Types {#string-types}

```rust,editable
{{#include ../../crates/language/tests/data_types/string_data_types.rs:example}}
```

## Special Types {#special-types}

| Type Family | Types | Examples |
|---|---|---|
| Unit | [`unit`][primitive-unit]⮳. | The `()` type (aka 'void' in other languages) has exactly one value `()`, and is used when there is no other meaningful value that could be returned. |
| Never | [`never`][primitive-never]⮳. | `!` represents the type of computations which never resolve to any value at all. For example, the exit function `fn exit(code: i32) -> !` exits the process without ever returning, and so returns `!`. |

See also [[functions | Functions]].

## Type Aliases {#type-aliases}

Use the `type` keyword to declare type aliases: `type Kilometers = i32;`.

## Related Topics {#skip}

See also:

- [[enums | Enums]].
  - [[option | Option]].
  - [[result | Result]].
- [[slices | Slices]].
- [[strings | Strings]].
- [[structs | Structs]].
- [[vectors | Vectors]].
- [[data-structures | Data Structures]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO string types / unit never examples
TODO find spot for overflows

| [Handle Overflows][ex-language-overflow-handling] |

### Handle Overflows {#overflow-handling}

- Wrap in all modes with the `wrapping_*` methods, such as [`wrapping_add`][primitive-u32::wrapping_add]{{hi:wrapping_add}}⮳.
- Return the [`std::option::Option::None`][c-std::option::Option::None]{{hi:std::option::Option::None}}⮳ value if there is overflow{{hi:Overflow}} with the `checked_*` methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value's minimum or maximum values with the `saturating_*` methods.

- [half — data structures in Rust](https://lib.rs/crates/half)

</div>
