# Simple Data Types

- Integers: [`{{i:i8}}`][i8]⮳, [`{{i:i16}}`][i16]⮳, [`{{i:i32}}`][i32]⮳, [`{{i:i64}}`][i64]⮳, [`{{i:i128}}`][i128]⮳, [`{{i:isize}}`][isize]⮳
- Unsigned: [`{{i:u8}}`][u8]⮳, [`{{i:u16}}`][u16]⮳, [`{{i:u32}}`][u32]⮳, [`{{i:u128}}`][u128]⮳, [`{{i:usize}}`][usize]⮳
  - [`{{i:usize}}`][usize]⮳ and [`{{i:isize}}`][isize]⮳ are 32 or 64 bits, depending on the architecture of the computer.
- Floating point: [`{{i:f32}}`][f32]⮳, [`{{i:f64}}`][f64]⮳
- Boolean: [`{{i:bool}}`][bool]⮳: `true`, `false`
- Char: `let z: char = 'ℤ';` Unicode
- Tuples: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
  - Access via `let five_hundred = x.0;`
  - Destructuring via `let (x, y, z) = tup;`
- Arrays: `let a: [i32; 5] = [1, 2, 3, 4, 5];` allocated on the stack. access via `let first = a[0];`
  - A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size
- Unit (aka void): `()`
- Type aliases: `type Kilometers = i32;`

## Overflow handling

- Wrap in all modes with the `wrapping_*` methods, such as [`{{i:wrapping_add}}`][u32::wrapping_add]⮳.
- Return the [`{{i:None}}`][std::option::Option::None]⮳ value if there is overflow with the `checked_*` methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value’s minimum or maximum values with the `saturating_*` methods.

{{#include ../refs/link-refs.md}}
