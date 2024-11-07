# Automatic trait derivation

{{#include derive.incl.md }}

[![std][c-std-badge]][c-std]

The [`derive`][book-rust-reference-derive]{{hi:derive}}⮳ attribute{{hi:Attributes}} generates code that will implement a trait with its own default implementation{{hi:Default implementation}} on the type you’ve annotated with the derive syntax.

[Derivable traits][book-rust-derivable-traits]{{hi:Derivable traits}}⮳

```rust,editable
{{#include ../../deps/tests/std/derive.rs:example}}
```

You can use the `cargo_expand`{{hi:cargo-expand}} utility to see the exact code that is generated for your specific type.

See also:

- [Derive][book-rust-reference-derive]⮳

## Derive More

[![derive_more][c-derive_more-badge]][c-derive_more]{{hi:derive_more}}

[Derive More (crates)][c-derive_more-crates.io]⮳ derive lots of additional, commonly used traits and static methods for both structs{{hi:Structs}} and enums{{hi:Enums}}.

```rust,noplayground
{{#include ../../deps/tests/std/derive_more.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
<div class="hidden">
TODO: review
</div>
