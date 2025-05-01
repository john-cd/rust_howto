# Variables and Constants

{{#include variables_and_constants.incl.md}}

## Variables {#variables}

[![Rust by example - Variable bindings][book-rust-by-example-variable_bindings-badge]][book-rust-by-example-variable_bindings]{{hi:Variables}} [![Rust by example - constants][book-rust-by-example-constants-badge]][book-rust-by-example-constants]{{hi:const}}

Variables are immutable by default. Use the `mut` keyword to make a variable mutable.

Starting the name of a variable{{hi:Variables}} with an underscore silences unused variable{{hi:Unused variable}} warnings.

Constants must be explicitly typed and their values must be known at compile time.
Constants are declared using the `const` keyword.

```rust,editable
{{#include ../../crates/language/tests/variables_and_constants/vars_and_consts.rs:example}}
```

## Shadowing {#shadowing}

Variables can be redeclared with the same name, "shadowing" the previous variable. The new variable effectively hides the previous one within its scope.{{hi:Shadowing}}

```rust,editable
{{#include ../../crates/language/tests/variables_and_constants/shadowing.rs:example}}
```

## Destructuring {#destructuring}

Destructuring allows you to unpack values from compound types like tuples, arrays, structs, or enums into individual variables.{{hi:Destructuring}}

```rust,editable
{{#include ../../crates/language/tests/variables_and_constants/destructuring.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[variables_and_constants: add text NOW](https://github.com/john-cd/rust_howto/issues/563)
</div>
