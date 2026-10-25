<!-- TODO Reviewed ??-??-2026 -->

# Documentation

{{#include documentation.incl.md}}

Rust documentation serves both readers and tools: write API contracts in rustdoc comments, keep examples executable, and generate HTML with Cargo. This page focuses on Rust API documentation; see the separate mdBook chapters for publishing a book.

## Document Your API {#documenting-your-code}

Add `///` comments to public items to describe their purpose, inputs, outputs, errors, panics, and important constraints. Use `//!` at the start of a module or crate root for documentation about the containing module or crate. Use Markdown headings, lists, code spans, and intra-doc links to connect related APIs.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/documentation/rustdoc.rs:example}}
```

## Document Types and Modules {#module-or-crate-level-documentation}

The crate root's `//!` comment becomes the front page of the crate documentation. Item comments describe public types and methods; module comments explain how the pieces fit together. See the included examples for item documentation, `//!` comments, and a function example with an `Arguments` section.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/documentation/rustdoc2.rs:example}}
```

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/documentation/rustdoc3.rs:example}}
```

`#[doc(hidden)]` omits an item from generated documentation. It does not make a public item private, so do not use it as an API visibility or stability boundary.

## Test Documentation Examples {#documentation-tests}

Rust code fences in rustdoc comments are compiled as doctests by `cargo test`. Prefer examples that compile and demonstrate the documented behavior. Use rustdoc's fence options, such as `no_run` for examples that should compile but not execute and `ignore` only when a snippet cannot be made runnable. Keep longer examples in tested source files and include them in the prose when possible.

[`doc-comment`][c~doc-comment~docs]↗{{hi:doc-comment}} provides macros for attaching documentation text to Rust items and for running Markdown files such as a README as doctests. This crate already includes a compiled example of its `doc_comment!` macro:

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/documentation/doc_comment.rs:example}}
```

## Generate API Documentation {#generate-api-documentation}

Use `cargo doc --open` to build and open API documentation for the selected package and its dependencies. In a workspace, `cargo doc --workspace --no-deps` builds documentation for workspace packages without rebuilding each dependency's docs. Add `RUSTDOCFLAGS="-D warnings"` in a strict documentation job when all warnings should fail CI.

The `html_playground_url` rustdoc attribute can add a Run link to suitable code examples. The included example shows where to set it; use a real Playground URL for a published crate.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/rustdoc4.rs:example}}
```

## Render Mermaid Diagrams {#aquamarine}

[`aquamarine`][aquamarine~repo]↗ is a procedural macro that lets rustdoc render Mermaid diagrams from `mermaid` code fences in item documentation. It is optional presentation tooling; the crate's README documents the attribute and setup. Keep the surrounding API explanation understandable without relying on the diagram alone.

## Document Function Parameters {#function-parameters}

Rustdoc normally documents function parameters in the function's `///` block, for example with an `Arguments` section. [`roxygen`][c~roxygen~docs]↗{{hi:roxygen}} is a third-party procedural macro that can turn parameter-level comments into that section. Because this changes how comments are processed, consider whether a clearer signature, a parameter struct, or ordinary prose is a better fit before adopting it.

## References {#references .skip}

- [The rustdoc book][book~rustdoc]↗{{hi:rustdoc}}.
- [`docs.rs`][docs.rs~website]↗{{hi:docs.rs}}, a host for documentation built from published Rust crates.

## Related Topics {#related-topics .skip}

- [[mdbook | mdBook]].
- [[mdbook_utils | mdbook-utils]].
- [[templ | templ]].
- [[badges | Badges]].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
