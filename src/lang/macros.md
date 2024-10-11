# Macros

[Rust reference - Macros][book-rust-reference-macros]⮳

[Rust by example - Macros][book-rust-by-example-macros]⮳

The [Little Book of Rust Macros][book-rust-macros]⮳

```rust,editable
{{#include ../../deps/tests/macros.rs}}
```

## Key crates

[![paste][c-paste-badge]][c-paste]  [![paste-github][c-paste-github-badge]][paste-github]

[`{{i:Paste}}`][c-paste]⮳ provides a flexible way to paste together identifiers in a macro, including using pasted identifiers to define new items.

[![proc-macro2][c-proc-macro2-badge]][proc-macro2]  [![proc-macro2-github][c-proc-macro2-github-badge]][proc-macro2-github]  [![proc-macro-workshop][c-proc-macro-workshop-badge]][proc-macro-workshop]

[`{{i:proc-macro2}}`][proc-macro2]⮳ [![proc-macro2-github][c-proc-macro2-github-badge]][proc-macro2-github] bring proc-macro-like functionality to other contexts like build.rs and main.rs and makes procedural macros unit testable.

[![syn][c-syn-badge]][c-syn]  [![syn-github][c-syn-github-badge]][syn-github]

[`{{i:Syn}}`][c-Syn]⮳ is a parsing library for parsing a stream of Rust tokens into a syntax tree of Rust source code.

[![quote][c-quote-badge]][c-quote]

[`{{i:Quote}}`][c-Quote]⮳ provides the `quote!` macro for turning Rust {{i:syntax tree}} data structures into {{i:tokens}} of source code.

## Tools

[Cargo expand][c-cargo-expand-crates.io]⮳ [![cargo-expand-github][c-cargo-expand-github-badge]][c-cargo-expand-github]⮳

## See also

[![proc-macro-workshop][c-proc-macro-workshop-badge]][proc-macro-workshop]

[![watt][c-watt-badge]][c-watt]  [![watt-github][c-watt-github-badge]][watt-github]

{{#include ../refs/link-refs.md}}
