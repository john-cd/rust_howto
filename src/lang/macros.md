# Macros

[Rust reference - Macros][book-rust-reference-macros]⮳

[Rust by example - Macros][book-rust-by-example-macros]⮳

The [Little Book of Rust Macros][book-rust-macros]⮳

```rust,editable
{{#include ../../deps/tests/macros.rs}}
```

## Key crates

[![paste][c-paste-badge]][c-paste]  [![paste-github][c-paste-github-badge]][c-paste-github]

[`Paste`][c-paste]{{hi:Paste}}⮳ provides a flexible way to paste together identifiers in a macro, including using pasted identifiers to define new items.

[![proc_macro2][c-proc_macro2-badge]][c-proc_macro2]  [![proc_macro2-github][c-proc_macro2-github-badge]][c-proc_macro2-github]  [![proc-macro-workshop][proc-macro-workshop-badge]][proc-macro-workshop]

[`proc_macro2`][c-proc_macro2]{{hi:proc_macro2}}⮳ [![proc_macro2-github][c-proc_macro2-github-badge]][c-proc_macro2-github] bring proc-macro-like functionality to other contexts like build.rs and main.rs and makes procedural macros unit testable.

[![syn][c-syn-badge]][c-syn]  [![syn-github][c-syn-github-badge]][c-syn-github]

[`Syn`][c-Syn]{{hi:Syn}}⮳ is a parsing library for parsing a stream of Rust tokens into a syntax tree of Rust source code.

[![quote][c-quote-badge]][c-quote]

[`Quote`][c-Quote]{{hi:Quote}}⮳ provides the `quote!` macro for turning Rust syntax tree{{hi:syntax tree}} data structures into tokens{{hi:tokens}} of source code.

## Tools

[Cargo expand][c-cargo_expand-crates.io]⮳ [![cargo_expand-github][c-cargo_expand-github-badge]][c-cargo_expand-github]⮳

## See also

[![proc-macro-workshop][proc-macro-workshop-badge]][proc-macro-workshop]

[![watt][c-watt-badge]][c-watt]  [![watt-github][c-watt-github-badge]][c-watt-github]

{{#include ../refs/link-refs.md}}
