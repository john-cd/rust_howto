<!-- TODO Reviewed ??-??-2026. -->

# Integer Overflow

{{#include overflow.incl.md}}

Rust provides explicit integer operations so code can choose what should happen when a result is outside the type's range. Make that choice visible whenever wraparound, saturation, or failure handling is part of the algorithm.

## Choose an Explicit Overflow Policy {#explicit-overflow-methods}

- `wrapping_*` methods always wrap.
- `checked_*` methods return `None` on overflow.
- `overflowing_*` methods return the result and a boolean overflow flag.
- `saturating_*` methods clamp the result to the type's minimum or maximum.

```rust,noplayground
{{#include ../../../crates/standard_library/examples/overflow/overflow_methods.rs:example}}
```

| Method family | Behavior if the operation overflows | Example with `u8` |
| --- | --- | --- |
| `wrapping_*` | Wraps around | `255_u8.wrapping_add(1) == 0` |
| `checked_*` | Returns `None` | `255_u8.checked_add(1) == None` |
| `overflowing_*` | Returns `(result, true)` | `255_u8.overflowing_add(1) == (0, true)` |
| `saturating_*` | Clamps to the maximum or minimum | `255_u8.saturating_add(1) == 255` |

## Use `std::num::Wrapping` {#wrapping-type}

`std::num::Wrapping` makes ordinary arithmetic operators wrap for a wrapped integer value.

```rust,noplayground
{{#include ../../../crates/standard_library/examples/overflow/wrapping_type.rs:example}}
```

## Observe Default Debug and Release Behavior {#debug-and-release-overflow}

With Cargo's default profiles, overflow checks are enabled in debug builds and disabled in release builds. Ordinary integer arithmetic therefore panics on overflow in a debug build and wraps in a release build. A project can change this behavior with the `overflow-checks` profile setting, so avoid relying on the profile default when the result is part of the program's logic.

Run the example from `bk/crates` to see the two default behaviors. The debug command exits with an overflow panic; the release command prints `255 + 1 = 0`.

```sh
cargo run --locked -p standard_library --example overflow -- profile-behavior
cargo run --locked --release -p standard_library --example overflow -- profile-behavior
```

## Related Topics {#related-topics .skip}

- [Operators](../../standard-library/operators.md).
- [Recoverable Error Handling](../../standard-library/recoverable_error_handling.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
