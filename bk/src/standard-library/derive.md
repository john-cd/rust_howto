# Automatic trait derivation

{{#include derive.incl.md}}

[![std][c-std-badge]][c-std]

The [`derive`][book-rust-reference-derive]{{hi:derive}}⮳ attribute{{hi:Attributes}} generates code that will implement a trait with its own default implementation{{hi:Default implementation}} on the type you've annotated with the derive syntax.

[Derivable traits][book-rust-derivable-traits]{{hi:Derivable traits}}⮳

```rust,editable,editable
{{#include ../../crates/standard_library/tests/other/derive.rs:example}}
```

You can use the [`cargo_expand`][c-cargo_expand]⮳{{hi:cargo_expand}} utility to see the exact code that is generated for your specific type.

See also:

- [Derive][book-rust-reference-derive]⮳

## Derive More {#derive-more}

[![derive_more][c-derive_more-badge]][c-derive_more]{{hi:derive_more}}

[Derive More (crates)][c-derive_more-crates.io]⮳ derive lots of additional, commonly used traits and static methods for both structs{{hi:Structs}} and enums{{hi:Enums}}.

```rust,editable,noplayground
{{#include ../../crates/standard_library/tests/other/derive_more.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
<div class="hidden">
[derive: review (P1)](https://github.com/john-cd/rust_howto/issues/621)

## See also

[[attributes | Attributes]]

[[rust-patterns | Rust Patterns]]
</div>
