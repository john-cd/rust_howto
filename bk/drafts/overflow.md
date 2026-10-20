# Safe Arithmetic in Rust

{{#include overflow.incl.md}}

Rust provides explicit arithmetic operations so you can choose the overflow behavior that best matches your intent. Good overflow handling is especially important for numeric code, low-level algorithms, and any code that depends on correctness in edge cases.

## Handle Overflows {#overflow-handling}

- Wrap in all modes with the `wrapping_*`{{hi:wrapping_*}} methods, such as [`wrapping_add`][primitive~u32::wrapping_add]↗{{hi:wrapping_add}}.
- Return the [`std::option::Option::None`][c~std::option::Option::None~docs]↗{{hi:std::option::Option::None}} value if there is overflow with the `checked_*`{{hi:checked_*}} methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*`{{hi:overflowing_*}} methods.
- Saturate at the value's minimum or maximum values with the `saturating_*`{{hi:saturating_*}} methods.

```rust,noplayground
fn main() {
    let x = 255u8;

    assert_eq!(x.wrapping_add(1), 0);
    assert_eq!(x.checked_add(1), None);
    assert_eq!(x.overflowing_add(1), (0, true));
    assert_eq!(x.saturating_add(1), 255);
}
```

## Summary of Arithmetic Behaviors

| Method Class | Behavior on Overflow | Example |
|---|---|---|
| `wrapping_` | Performs two's complement wrapping | `255u8.wrapping_add(1) == 0` |
| `checked_` | Returns `None` | `255u8.checked_add(1) == None` |
| `overflowing_` | Returns `(result, overflowed: bool)` | `255u8.overflowing_add(1) == (0, true)` |
| `saturating_` | Returns the max/min of the type | `255u8.saturating_add(1) == 255` |

For repeated wrapping arithmetic, the `std::num::Wrapping` type can also be useful:

```rust,noplayground
use std::num::Wrapping;

let x = Wrapping(255u8);
let y = x + Wrapping(1);
assert_eq!(y.0, 0);
```

## Panics in Debug vs. Wrapping in Release

By default, Rust panics on integer overflow in **debug** builds. In **release** builds, integer overflow on primitive types is handled as wrapping arithmetic.

Because the behavior changes between build profiles, you should use explicit overflow methods if your code depends on a specific outcome.

## When to Choose Each Method

- Use `checked_*` when overflow is an error condition and you want to handle it explicitly.
- Use `overflowing_*` when you need both the result and a flag that indicates overflow.
- Use `wrapping_*` for low-level algorithms where wraparound is intentional.
- Use `saturating_*` for values that should stay within a bounded range.

## Related Topics {#related-topics .skip}

- [[mathematics | Mathematics]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
