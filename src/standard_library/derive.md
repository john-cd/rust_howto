# Automatic trait derivation

{{#include derive.incl.md }}

The [`{{i:derive}}`][book-rust-reference-derive]⮳ {{i:attribute}} generates code that will implement a trait with its own {{i:default implementation}} on the type you’ve annotated with the derive syntax.

[Derivable traits][book-rust-derivable-traits]⮳

{{#playground ../../deps/tests/derive.rs editable}}

You can use the {{i:`cargo-expand`}} utility to see the exact code that is generated for your specific type.

## Derive More

[![derive-more][derive-more-badge]][derive-more]

[Derive More (crates)][derive-more-crate]⮳ derive lots of additional, commonly used traits and static methods for both {{i:structs}} and {{i:enums}}.

```rust,editable,noplayground
{{#include ../../deps/tests/derive_more.rs}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
