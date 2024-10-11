# Simple Data Types

- Integers: {{hi:i8}}[`i8`][primitive-i8]⮳, {{hi:i16}}[`i16`][primitive-i16]⮳, {{hi:i32}}[`i32`][primitive-i32]⮳, {{hi:i64}}[`i64`][primitive-i64]⮳, {{hi:i128}}[`i128`][primitive-i128]⮳, {{hi:isize}}[`isize`][primitive-isize]⮳
- Unsigned: {{hi:u8}}[`u8`][primitive-u8]⮳, {{hi:u16}}[`u16`][primitive-u16]⮳, {{hi:u32}}[`u32`][primitive-u32]⮳, {{hi:u128}}[`u128`][primitive-u128]⮳, {{hi:usize}}[`usize`][primitive-usize]⮳
  - {{hi:usize}}[`usize`][primitive-usize]⮳ and {{hi:isize}}[`isize`][primitive-isize]⮳ are 32 or 64 bits, depending on the architecture of the computer.
- Floating point: {{hi:f32}}[`f32`][primitive-f32]⮳, {{hi:f64}}[`f64`][primitive-f64]⮳
- Boolean: {{hi:bool}}[`bool`][primitive-bool]⮳: `true`, `false`
- Char: `let z: char = 'ℤ';` Unicode
- Tuples: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
  - Access via `let five_hundred = x.0;`
  - Destructuring via `let (x, y, z) = tup;`
- Arrays: `let a: [i32; 5] = [1, 2, 3, 4, 5];` allocated on the stack. access via `let first = a[0];`
  - A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size
- Unit (aka void): `()`
- Type aliases: `type Kilometers = i32;`

## Overflow handling

- Wrap in all modes with the `wrapping_*` methods, such as {{hi:wrapping_add}}[`wrapping_add`][primitive-u32::wrapping_add]⮳.
- Return the {{hi:None}}[`None`][c-std::option::Option::None]⮳ value if there is overflow with the `checked_*` methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value’s minimum or maximum values with the `saturating_*` methods.

{{#include ../refs/link-refs.md}}
