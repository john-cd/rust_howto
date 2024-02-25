# Automatic trait derivation

The `derive` attribute generates code that will implement a trait with its own default implementation on the type you’ve annotated with the derive syntax.

[Derivable traits][book-rust-derivable-traits]⮳

{{#playground ../../deps/tests/derive.rs editable}}

You can use the `cargo-expand` utility to see the exact code that is generated for your specific type.

## Derive More

[![derive-more][derive-more-badge]][derive-more]

[Derive More (crates)][derive-more-crate]⮳ derive lots of additional, commonly used traits and static methods for both structs and enums.

```rust,editable,noplayground
{{#include ../../deps/tests/derive_more.rs}}
```

{{#include ../refs/link-refs.md}}