# Closures

{{#include closures.incl.md}}

## Closures {#closures}

[![Rust by example - Closures][book-rust-by-example-closures-badge]][book-rust-by-example-closures]

```rust,editable
{{#include ../../deps/tests/lang/closures.rs:example}}
```

## Closure with type annotations {#closure-with-type-annotations}

```rust,editable
{{#include ../../deps/tests/lang/closures_with_type_annotations.rs:example}}
```

{{i:Closures}} can capture variables{{hi:Capture of variables}}

- by reference: &T
- by mutable reference: &mut T
- by value: T

They preferentially capture variables by reference and only go lower when required.

To force a move:

```rust,editable
{{#include ../../deps/tests/lang/closures_move.rs:example}}
```

## Closures as input parameters {#closures-as-input-parameters}

```rust,editable
{{#include ../../deps/tests/lang/closures_as_input_parameters.rs:example}}
```

- [`std::ops::Fn`][c-std::ops::Fn]{{hi:std::ops::Fn}}⮳: the closure uses the captured value by reference (`&T`)
- [`std::ops::FnMut`][c-std::ops::FnMut]{{hi:std::ops::FnMut}}⮳: the closure uses the captured value by mutable reference (`&mut T`)
- [`std::ops::FnOnce`][c-std::ops::FnOnce]{{hi:std::ops::FnOnce}}⮳: the closure uses the captured value by value (`T`)

Functions may also be used as arguments.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO P1: edit
</div>
