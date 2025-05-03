# `use` Keyword

{{#include use_keyword.incl.md}}

## Avoid Writing Full Paths with the `use` Keyword {#use-keyword}

[![book-rust-by-example-use][book-rust-by-example-use-badge]][book-rust-by-example-use]

The [`use`][book-rust-reference-use]{{hi:use}}⮳ keyword creates a shortcut for a path. The shorter name can be used everywhere else in the scope.

```rust,editable
{{#include ../../crates/code_structure/tests/use/use1.rs:example}}
```

## Bring a Function in Scope {#bring-function-in-scope}

It is idiomatic to bring the function's parent module into scope, not the function itself:

```rust,editable
{{#include ../../crates/code_structure/tests/use/use2.rs:example}}
```

## Bring a Struct or Enum in Scope {#bring-struct-or-enum-in-scope}

On the other hand, when bringing in [structs][p-structs], [enums][p-enums], and other items with `use`, it is idiomatic to specify the full path.

```rust,editable
{{#include ../../crates/code_structure/tests/use/use3.rs:example}}
```

## Bring an Item from an External Crate into Scope {#bring-item-from-external-crate-into-scope}

```rust,editable
{{#include ../../crates/code_structure/tests/use/use4.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO review
</div>
