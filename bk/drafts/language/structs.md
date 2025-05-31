# Structs

{{#include structs.incl.md}}

## Struct Syntax {#struct}

[![Rust by example - Structs][book-rust-by-example-structs-badge]][book-rust-by-example-structs]{{hi:Structs}}

Structs are custom data types that allow you to group related data together.

The following example demonstrates how to define a `struct` and create a instance of it:

```rust,editable
{{#include ../../crates/language/tests/structs/structs.rs:example}}
```

Struct fields{{hi:Fields}} follow the general rule of everything being private by default{{hi:Private by default}} unless annotated with [`pub`][book-rust-reference-visibility-and-privacy]{{hi:Visibility}}⮳.

It is common to define a function or an associated function to initialize the struct:

```rust,editable
{{#include ../../crates/language/tests/structs/structs2.rs:example}}
```

You may update a `struct` by using the `..previous_instance` to fill in the rest.

You can define tuple struct and Unit-like structs (without fields):

```rust,editable
{{#include ../../crates/language/tests/structs/structs3.rs:example}}
```

Implementation block, associated function, methods TODO

```rust,editable
{{#include ../../crates/language/tests/structs/structs4.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[structs: add description NOW](https://github.com/john-cd/rust_howto/issues/559)
rename examples
</div>
