# Macros

[Rust reference - Macros][rust-reference-macros]⮳

[Rust by example - Macros][rust-by-example-macros]⮳

The [Little Book of Rust Macros][rust-macros]⮳

```rust,editable
{{#include ../../deps/examples/macros.rs}}
```

## Key crates

[![paste-badge]][paste] [(github)][paste-github]

[Paste][paste] provides a flexible way to paste together identifiers in a macro, including using pasted identifiers to define new items.

[![proc-macro2-badge]][proc-macro2] [(github)][proc-macro2-github] [(workshop)][proc-macro-workshop]

[proc-macro2][proc-macro2]⮳ bring proc-macro-like functionality to other contexts like build.rs and main.rs and makes procedural macros unit testable.

[![syn-badge]][syn]  [(github)][syn-github]

[Syn][syn] is a parsing library for parsing a stream of Rust tokens into a syntax tree of Rust source code.

[![quote-badge]][quote]

[Quote][quote] provides the `quote!` macro for turning Rust syntax tree data structures into tokens of source code.

## Tools

[Cargo expand][cargo-expand]⮳ [(github)][cargo-expand-github]⮳

## See also

[`proc macro` workshop][proc-macro-workshop]⮳

[![watt-badge]][watt] [(github)][watt-github]

{{#include ../refs/link-refs.md}}
