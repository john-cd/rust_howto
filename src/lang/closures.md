# Closures

```rust,editable
{{#include ../../deps/examples/closures.rs}}
```

## Closure with type annotations

```rust,editable
{{#include ../../deps/examples/closures_with_type_annotations.rs}}
```

Closures can capture variables

- by reference: &T
- by mutable reference: &mut T
- by value: T

They preferentially capture variables by reference and only go lower when required.

To force a move:

```rust,editable
{{#include ../../deps/examples/closures_move.rs}}
```

## Closures as input parameters

```rust,editable
{{#include ../../deps/examples/closures_as_input_parameters.rs}}
```

- `Fn`: the closure uses the captured value by reference (`&T`)
- `FnMut`: the closure uses the captured value by mutable reference (`&mut T`)
- `FnOnce`: the closure uses the captured value by value (`T`)

Functions may also be used as arguments.

{{#include ../links.md}}
