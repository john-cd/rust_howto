# Simple Data Types

- Integers: [`{{i:i8}}`][c-i8]⮳, [`{{i:i16}}`][c-i16]⮳, [`{{i:i32}}`][c-i32]⮳, [`{{i:i64}}`][c-i64]⮳, [`{{i:i128}}`][c-i128]⮳, [`{{i:isize}}`][c-isize]⮳
- Unsigned: [`{{i:u8}}`][c-u8]⮳, [`{{i:u16}}`][c-u16]⮳, [`{{i:u32}}`][c-u32]⮳, [`{{i:u128}}`][c-u128]⮳, [`{{i:usize}}`][primitive-usize]⮳
  - [`{{i:usize}}`][primitive-usize]⮳ and [`{{i:isize}}`][c-isize]⮳ are 32 or 64 bits, depending on the architecture of the computer.
- Floating point: [`{{i:f32}}`][c-f32]⮳, [`{{i:f64}}`][c-f64]⮳
- Boolean: [`{{i:bool}}`][primitive-bool]⮳: `true`, `false`
- Char: `let z: char = 'ℤ';` Unicode
- Tuples: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
  - Access via `let five_hundred = x.0;`
  - Destructuring via `let (x, y, z) = tup;`
- Arrays: `let a: [i32; 5] = [1, 2, 3, 4, 5];` allocated on the stack. access via `let first = a[0];`
  - A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size
- Unit (aka void): `()`
- Type aliases: `type Kilometers = i32;`

## Overflow handling

- Wrap in all modes with the `wrapping_*` methods, such as [`{{i:wrapping_add}}`][primitive-u32::wrapping_add]⮳.
- Return the [`{{i:None}}`][c-std::option::Option::None]⮳ value if there is overflow with the `checked_*` methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value’s minimum or maximum values with the `saturating_*` methods.

{{#include ../refs/link-refs.md}}
