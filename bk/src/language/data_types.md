# Data Types

{{#include data_types.incl.md}}

## Scalar Data Types {#scalar-data-types}

[![Rust by example - Primitives][book-rust-by-example-primitives-badge]][book-rust-by-example-primitives]{{hi:Primitives}}

Rust has several categories of primitive scalar types: integers, floating-point numbers, Booleans, and unicode characters.

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

## Compound Data Types: Tuples and Arrays {#compound-types}

[![Rust by example - tuples][book-rust-by-example-tuples-badge]][book-rust-by-example-tuples] [![Rust by example - array][book-rust-by-example-array-badge]][book-rust-by-example-array]

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

| Type | Examples |
|---|---|
| Tuples | `let tup: (i32, f64, u8) = (500, 6.4, 1);`. Access via `let five_hundred = x.0;`. Destructuring via `let (x, y, z) = tup;`. |
| Arrays | `let a: [i32; 5] = [1, 2, 3, 4, 5];`. Access via `let first = a[0];`. |

Both are fixed length. A [[vectors | Vector]] is a similar collection type provided by the [standard library][p-standard-library] that is allowed to grow or shrink in size.

The following provides examples of tuples and arrays:

```rust,editable
{{#include ../../crates/language/tests/data_types/compound_data_types.rs:example}}
```

## String Types {#string-types}

Primitive type `str` is a string slice that represents a view into a string, allowing you to access a portion of a string without owning it. It is immutable and typically used as a reference type, denoted as `&str`, which allows for efficient borrowing of string data. Its memory can be on the heap, stack, or static. String slices must always be valid UTF-8. `&'static str` is the type of string literals.

`String` is a growable, mutable, owned string allocated on the heap. It is not a primitive type, but is rather part of the standard library.

`&String` can be coerced to `&str`, which makes `&str` a candidate for function arguments, if mutability and ownership are not required. If mutation is needed, use `&mut String`.

```rust,editable
{{#include ../../crates/language/tests/data_types/string_data_types.rs:example}}
```

Strings are covered in much more details in the [[strings | Strings]] and [[text-processing | Text Processing]] chapters.

## Special Types {#special-types}

| Type Family | Types | Examples |
|---|---|---|
| Unit | [`unit`][primitive-unit]⮳. | The `()` type (aka 'void' in other languages) has exactly one value `()`, and is used when there is no other meaningful value that could be returned. |
| Never | [`never`][primitive-never]⮳. | `!` represents the type of computations which never resolve to any value at all. For example, the exit function `fn exit(code: i32) -> !` exits the process without ever returning, and so returns `!`. |

See also the [[functions | Functions]] chapter.

```rust,editable
{{#include ../../crates/language/tests/data_types/unit_never.rs:example}}
```

## Type Aliases {#type-aliases}

A type alias is a way to give a new name to an existing type, making code easier to read and write. It does not create a new type, meaning the original type's properties still apply.

Use the `type` keyword to declare type aliases:

```rust,editable
type Kilometers = u32;

type Point = (i32, i32);

// With generics:
type TypeAlias<T> = Bar<T> where T: Foo;
```

## Related Topics {#skip}

See also:

- [[enums | Enums]].
  - [[option | Option]].
  - [[result | Result]].
- [[generics | Generics]].
- [[slices | Slices]].
- [[strings | Strings]].
- [[structs | Structs]].
- [[vectors | Vectors]].
- [[data-structures | Data Structures]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
