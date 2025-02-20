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

- No data copying: Zero-copy avoids unnecessary data copying by directly interpreting the memory of one data structure as another.
- [Performance][p-performance]: Eliminating data copying can significantly improve [performance][p-performance], especially in scenarios involving frequent data transfers between different memory regions (e.g., network I/O, inter-process communication).
- Safety: The zerocopy crate provides mechanisms to ensure safe and correct zero-copy operations.

Zerocopy is often used in [network programming][p-network-programming], where high [performance][p-performance] and low memory overhead are critical, or image handling.

```rust,editable
{{#include ../../../crates/cats/encoding/tests/typecasts/zerocopy.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

[typecasts: write (P1)](https://github.com/john-cd/rust_howto/issues/354)

Rust is a statically typed language, so "type casting" in the traditional C/C++ sense is less common.  Instead, Rust emphasizes explicit type conversions and traits. Here's a breakdown:

## Type Conversion (using as)

The `as` keyword is used for basic type conversions, but it's important to be aware of potential issues like truncation or overflow. This is the closest equivalent to C-style casting, but it should be used with caution.

## Traits for Conversions

`From` and `Into`: These traits are used for conversions that should always succeed. `From` allows you to define how to convert from a type, and `Into` provides a blanket implementation for converting into a type if From is implemented. This is the preferred way to do type conversions in most cases.

`TryFrom` and `TryInto`: These traits are used for conversions that might fail. They return a `Result` to indicate success or failure. Use these when there's a possibility of the conversion not working (e.g., parsing a string to a number).

## Parsing Strings

The `FromStr` trait is used for parsing strings into other types. Many standard types implement `FromStr`. Example:

```rust,editable
let num: i32 = "123".parse().unwrap();
```

## Casting Between Numeric Types

Use the `as` keyword, but be very cautious about potential loss of data or unexpected behavior due to truncation or overflow.

## Casting Between Pointers

Requires unsafe blocks and careful consideration of memory management.

## Casting Between Traits (Dynamic Dispatch)

Use trait objects (dyn Trait) for dynamic dispatch, but this is more about polymorphism than traditional type casting.

## Coercions (Implicit Conversions)

Rust performs some implicit coercions, such as dereferencing and unsizing. These are not type casts in the traditional sense, but they do involve implicit changes in type.

No Implicit Casting:  Rust does not do implicit type casting like C/C++.  This is a deliberate design choice to improve safety and avoid unexpected behavior.

## Key Differences from C/C++

Rust is much more explicit about type conversions. This helps to avoid bugs and makes the code more readable.
Rust encourages the use of traits like `From`, `Into`, `TryFrom`, and `TryInto` for conversions. This makes the code more generic and reusable.
Rust avoids implicit type casting, which can lead to unexpected behavior in C/C++.

In summary, instead of "casting," you'll usually implement or use traits like `From`, `Into`, `TryFrom`, `TryInto`, or FromStr for type conversions in Rust.  The `as` keyword exists, but it should be used carefully and only when other options aren't suitable, as it can lead to data loss or unexpected behavior if not handled properly.

</div>
