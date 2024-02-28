# Macros

[Rust reference - Macros][book-rust-reference-macros]⮳

[Rust by example - Macros][book-rust-by-example-macros]⮳

The [Little Book of Rust Macros][book-rust-macros]⮳

```rust,editable
{{#include ../../deps/tests/macros.rs}}
```

## Key crates

[![paste][paste-badge]][paste]  [![paste-github][paste-github-badge]][paste-github]

[`Paste`][paste] provides a flexible way to paste together identifiers in a macro, including using pasted identifiers to define new items.

[![proc-macro2][proc-macro2-badge]][proc-macro2]  [(github)][proc-macro2-github]  [(workshop)][proc-macro-workshop]

[`proc-macro2`][proc-macro2]⮳ [![proc-macro2-github][proc-macro2-github-badge]][proc-macro2-github] bring proc-macro-like functionality to other contexts like build.rs and main.rs and makes procedural macros unit testable.

[![syn][syn-badge]][syn]  [![syn-github][syn-github-badge]][syn-github]

[`Syn`][Syn] is a parsing library for parsing a stream of Rust tokens into a syntax tree of Rust source code.

[![quote][quote-badge]][quote]

[`Quote`][Quote] provides the `quote!` macro for turning Rust {{i:syntax tree}} data structures into {{i:tokens}} of source code.

## Tools

[Cargo expand][cargo-expand-crate]⮳ [(github)][cargo-expand-github]⮳

## See also

[![proc-macro-workshop][proc-macro-workshop-badge]][proc-macro-workshop]

[![watt][watt-badge]][watt]  [![watt-github][watt-github-badge]][watt-github]

{{#include ../refs/link-refs.md}}
