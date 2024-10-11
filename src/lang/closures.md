# Closures

```rust,editable
{{#include ../../deps/tests/closures.rs}}
```

## Closure with type annotations

```rust,editable
{{#include ../../deps/tests/closures_with_type_annotations.rs}}
```

Closures can capture variables{{hi:capture variables}}

- by reference: &T
- by mutable reference: &mut T
- by value: T

They preferentially capture variables by reference and only go lower when required.

To force a move:

```rust,editable
{{#include ../../deps/tests/closures_move.rs}}
```

## Closures as input parameters

```rust,editable
{{#include ../../deps/tests/closures_as_input_parameters.rs}}
```

- [`std::ops::Fn`][c-std::ops::Fn]{{hi:std::ops::Fn}}⮳: the closure uses the captured value by reference (`&T`)
- [`std::ops::FnMut`][c-std::ops::FnMut]{{hi:std::ops::FnMut}}⮳: the closure uses the captured value by mutable reference (`&mut T`)
- [`std::ops::FnOnce`][c-std::ops::FnOnce]{{hi:std::ops::FnOnce}}⮳: the closure uses the captured value by value (`T`)

Functions may also be used as arguments.

{{#include ../refs/link-refs.md}}
