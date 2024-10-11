# Simple Data Types

- Integers: [`i8`][primitive-i8]{{hi:i8}}⮳, [`i16`][primitive-i16]{{hi:i16}}⮳, [`i32`][primitive-i32]{{hi:i32}}⮳, [`i64`][primitive-i64]{{hi:i64}}⮳, [`i128`][primitive-i128]{{hi:i128}}⮳, [`isize`][primitive-isize]{{hi:isize}}⮳
- Unsigned: [`u8`][primitive-u8]{{hi:u8}}⮳, [`u16`][primitive-u16]{{hi:u16}}⮳, [`u32`][primitive-u32]{{hi:u32}}⮳, [`u128`][primitive-u128]{{hi:u128}}⮳, [`usize`][primitive-usize]{{hi:usize}}⮳
  - [`usize`][primitive-usize]{{hi:usize}}⮳ and [`isize`][primitive-isize]{{hi:isize}}⮳ are 32 or 64 bits, depending on the architecture of the computer.
- Floating point: [`f32`][primitive-f32]{{hi:f32}}⮳, [`f64`][primitive-f64]{{hi:f64}}⮳
- Boolean: [`bool`][primitive-bool]{{hi:bool}}⮳: `true`, `false`
- Char: `let z: char = 'ℤ';` Unicode
- Tuples: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
  - Access via `let five_hundred = x.0;`
  - Destructuring via `let (x, y, z) = tup;`
- Arrays: `let a: [i32; 5] = [1, 2, 3, 4, 5];` allocated on the stack. access via `let first = a[0];`
  - A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size
- Unit (aka void): `()`
- Type aliases: `type Kilometers = i32;`

## Overflow handling

- Wrap in all modes with the `wrapping_*` methods, such as [`wrapping_add`][primitive-u32::wrapping_add]{{hi:wrapping_add}}⮳.
- Return the [`std::option::Option::None`][c-std::option::Option::None]{{hi:std::option::Option::None}}⮳ value if there is overflow with the `checked_*` methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value’s minimum or maximum values with the `saturating_*` methods.

{{#include ../refs/link-refs.md}}
