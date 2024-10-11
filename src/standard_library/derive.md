# Automatic trait derivation

{{#include derive.incl.md }}

The {{hi:derive}}[`derive`][book-rust-reference-derive]⮳ {{i:attribute}} generates code that will implement a trait with its own {{i:default implementation}} on the type you’ve annotated with the derive syntax.

[Derivable traits][book-rust-derivable-traits]⮳

{{#playground ../../deps/tests/derive.rs editable}}

You can use the `cargo_expand`{{hi:cargo_expand}} utility to see the exact code that is generated for your specific type.

## Derive More

[![derive_more][c-derive_more-badge]][c-derive_more]

[Derive More (crates)][c-derive_more-crates.io]⮳ derive lots of additional, commonly used traits and static methods for both {{i:structs}} and {{i:enums}}.

```rust,editable,noplayground
{{#include ../../deps/tests/derive_more.rs}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
