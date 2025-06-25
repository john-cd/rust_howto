# Variables

{{#include variables.incl.md}}

[![Rust by example - Variable bindings][book-rust-by-example-variable_bindings-badge]][book-rust-by-example-variable_bindings]{{hi:Variables}} [![Rust by example - constants][book-rust-by-example-constants-badge]][book-rust-by-example-constants]{{hi:const}}

Variables are declared with `let`, following by the variable's name and an (often optional) type annotation. Rust can often infer the type of a variable, but you can provide explicit type annotations.

```rust,editable
{{#include ../../crates/language/examples/variables/vars.rs:example}}
```

## Immutability and Mutability {#immutability}

Variables are immutable by default. Use the `mut` keyword to make a variable mutable:

```rust,editable
{{#include ../../crates/language/examples/variables/vars2.rs:example}}
```

## Shadowing {#shadowing}

Variables can be redeclared with the same name, "shadowing" the previous variable. The new variable effectively hides the previous one within its scope.{{hi:Shadowing}}

Shadowing is useful when you need to perform a series of transformations on a value, but you no longer need the original value. Instead of creating new variable names for each intermediate step, you can simply reuse the same name.

```rust,editable
{{#include ../../crates/language/examples/variables/shadowing.rs:example}}
```

## Destructuring {#destructuring}

Destructuring allows you to unpack values from compound types like tuples, arrays, structs, or enums into individual variables.{{hi:Destructuring}}

```rust,editable
{{#include ../../crates/language/examples/variables/destructuring.rs:example}}
```

## Related Topics {#skip}

- [[constants_and_statics | Constants and Statics]].
- [[match | Match and Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
