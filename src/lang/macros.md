# Macros

[Rust reference - Macros][book-rust-reference-macros]⮳

[Rust by example - Macros][book-rust-by-example-macros]⮳  [![Rust by example - macros][book-rust-by-example-macros-badge]][book-rust-by-example-macros]{{hi:Macros}}

The [Little Book of Rust Macros][book-rust-macros]⮳

```rust
{{#include ../../deps/tests/macros.rs}}
```

## Key crates

[![paste][c-paste-badge]][c-paste]{{hi:paste}}  [![paste-github][c-paste-github-badge]][c-paste-github]

[![paste][c-paste-badge]][c-paste]
[![paste-crates.io][c-paste-crates.io-badge]][c-paste-crates.io]
[![paste-github][c-paste-github-badge]][c-paste-github]
[![paste-lib.rs][c-paste-lib.rs-badge]][c-paste-lib.rs]

{{hi:Macros}}

[`paste`][c-paste]⮳ provides a flexible way to paste together identifiers in a macro, including using pasted identifiers to define new items.

[![proc_macro2][c-proc_macro2-badge]][c-proc_macro2]{{hi:proc-macro2}}  [![proc_macro2-github][c-proc_macro2-github-badge]][c-proc_macro2-github]  [![proc-macro-workshop][proc-macro-workshop-badge]][proc-macro-workshop]

[`proc_macro2`][c-proc_macro2]{{hi:proc-macro2}}⮳ [![proc_macro2-github][c-proc_macro2-github-badge]][c-proc_macro2-github] bring proc-macro-like functionality to other contexts like build.rs and main.rs and makes procedural macros unit testable.

[![syn][c-syn-badge]][c-syn]{{hi:syn}}
[![syn-crates.io][c-syn-crates.io-badge]][c-syn-crates.io]
[![syn-github][c-syn-github-badge]][c-syn-github]
[![syn-lib.rs][c-syn-lib.rs-badge]][c-syn-lib.rs]

[`syn`][c-syn]{{hi:syn}}⮳ is a parsing library for parsing a stream of Rust tokens into a syntax tree of Rust source code.

[![quote][c-quote-badge]][c-quote]{{hi:quote}}

[![quote][c-quote-badge]][c-quote]{{hi:quote}}
[![quote-crates.io][c-quote-crates.io-badge]][c-quote-crates.io]
[![quote-github][c-quote-github-badge]][c-quote-github]
[![quote-lib.rs][c-quote-lib.rs-badge]][c-quote-lib.rs]

[`quote`][c-quote]{{hi:quote}}⮳ provides the `quote!` macro for turning Rust syntax tree{{hi:Syntax tree}} data structures into tokens{{hi:Tokens}} of source code.

### darling

[![darling][c-darling-badge]][c-darling]{{hi:darling}}
[![darling-crates.io][c-darling-crates.io-badge]][c-darling-crates.io]
[![darling-github][c-darling-github-badge]][c-darling-github]
[![darling-lib.rs][c-darling-lib.rs-badge]][c-darling-lib.rs]

Derive macro to easily parse derive macro inputs

## Tools

[![cargo-expand-crates.io][c-cargo_expand-crates.io-badge]][c-cargo_expand-crates.io]{{hi:cargo-expand}}
[![cargo-expand-github][c-cargo_expand-github-badge]][c-cargo_expand-github]
[![cargo-expand-lib.rs][c-cargo_expand-lib.rs-badge]][c-cargo_expand-lib.rs]

[Cargo expand][c-cargo_expand-crates.io]⮳ [![cargo_expand-github][c-cargo_expand-github-badge]][c-cargo_expand-github]⮳

Allows you to inspect the code that macros expand to. Rust Analyzer also allows you to expand macros directly in your editor.

## See also

[![proc-macro-workshop][proc-macro-workshop-badge]][proc-macro-workshop]

[![watt][c-watt-badge]][c-watt]{{hi:watt}}
[![watt-crates.io][c-watt-crates.io-badge]][c-watt-crates.io]
[![watt-github][c-watt-github-badge]][c-watt-github]
[![watt-lib.rs][c-watt-lib.rs-badge]][c-watt-lib.rs]

{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO: add more / edit
</div>
