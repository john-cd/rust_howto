# Automatic trait derivation

The `derive` attribute generates code that will implement a trait with its own default implementation on the type you’ve annotated with the derive syntax.

[Derivable traits]( https://doc.rust-lang.org/book/appendix-03-derivable-traits.html )

{{#playground ../../deps/examples/derive.rs editable}}

You can use the `cargo-expand` utility to see the exact code that is generated for your specific type.

## Derive More

[Derive More]( https://crates.io/crates/derive_more ) derive lots of additional, commonly used traits and static methods for both structs and enums.

```rust,editable,ignore,noplayground
{{#include ../../deps/examples/derive_more.rs}}
```
