# Safe Arithmetic in Rust

{{#include overflow.incl.md}}

## Handle Overflows {#overflow-handling}

- Wrap in all modes with the `wrapping_*`{{hi:wrapping_*}} methods, such as [`wrapping_add`][primitive~u32::wrapping_add]↗{{hi:wrapping_add}}.
- Return the [`std::option::Option::None`][c~std::option::Option::None~docs]↗{{hi:std::option::Option::None}} value if there is overflow{{hi:Overflow}} with the `checked_*`{{hi:checked_*}} methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*`{{hi:overflowing_*}} methods.
- Saturate at the value's minimum or maximum values with the `saturating_*`{{hi:saturating_*}} methods.

- [`half`][c~half~lib.rs] - data structures in Rust.

## Summary of arithmetic behaviors

| Method Class | Behavior on Overflow | Example |
|---|---|---|
| `wrapping_` | Performs two's complement wrapping | `255u8.wrapping_add(1) == 0` |
| `checked_` | Returns `None` | `255u8.checked_add(1) == None` |
| `overflowing_` | Returns `(result, overflowed: bool)` | `255u8.overflowing_add(1) == (0, true)` |
| `saturating_` | Returns the max/min of the type | `255u8.saturating_add(1) == 255` |

## Panics in Debug vs. Wrapping in Release

By default, Rust panics on integer overflow in **debug** builds. However, in **release** builds, it performs wrapping arithmetic (equivalent to `wrapping_add`).

If you need a specific behavior regardless of the build mode, you should use the explicit methods listed above.

## Related Topics {#related-topics .skip}

- [[mathematics | Mathematics]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
