# Lifetimes {#lifetimes}

{{#include lifetimes.incl.md}}

[![Rust by example - Lifetimes][book-rust-by-example-lifetimes-badge]][book-rust-by-example-lifetimes]{{hi:Lifetimes}}

Prevent dangling references{{hi:Dangling references}}.

`&i32`        a reference
`&'a i32`     a reference with an explicit lifetime
`&'a mut i32` a mutable reference with an explicit lifetime

```rust,editable
{{#include ../../crates/language/tests/feat/static_lifetime.rs:example}}
```

The generic lifetime{{hi:Lifetimes}} `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`:

```rust,editable
{{#include ../../crates/language/tests/feat/generic_lifetime.rs:example}}
```

## Lifetime Annotations in Struct Definitions and methods {#lifetime-annotations}

```rust,editable
{{#include ../../crates/language/tests/feat/lifetime.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[lifetimes: review (P1)](https://github.com/john-cd/rust_howto/issues/547)

## See also

[[ownership_borrowing | Ownership Borrowing]]

[[cow | COW]]

[[memory-management | Memory Management]]

[[rust-patterns | Rust Patterns]]

[[typecasts | Typecasts]]

</div>
