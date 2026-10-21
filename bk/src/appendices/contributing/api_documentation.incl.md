<!-- TODO Reviewed ??-??-2026 -->
Rustdoc turns comments in Rust source into searchable, linked API documentation. Write docs beside the public items they describe, and keep examples and links checked as part of the crate workflow.

## Document the public contract

- Use `//!` for crate or module overviews and `///` for the item that follows. Start with a short summary, then explain the behavior and context a caller needs; the type signature already shows the input and output types.
- Document public types, functions, methods, traits, and variants. Explain observable behavior, important edge cases, side effects, and platform or feature requirements that are not clear from the signature.
- Describe when a function returns an error or can panic. For unsafe APIs, state the caller's safety requirements. Use `# Errors`, `# Panics`, and `# Safety` headings when those sections apply.
- Describe guarantees callers can rely on, not incidental implementation details.

See the [Rustdoc guidance on writing documentation][rustdoc~write-documentation] for further conventions.

## Include useful examples

Show a small, idiomatic example for non-obvious APIs. Rust code blocks in doc comments become doctests: `cargo test --doc` compiles and runs them with the crate. Keep examples self-contained and aligned with the documented behavior.

Use `no_run` when an example should compile but cannot be run in the test environment, `compile_fail` to demonstrate code that must not compile, and `ignore` only when rustdoc cannot compile the example. Explain any such limitation.

## Link related items

Use intra-doc links such as [`Type`] and [`Type::method`] to connect related public items. Rustdoc checks these links; see the [intra-doc link reference][rustdoc~intra-doc-links]. Prefer links users can follow in the published API, and resolve warnings for broken or ambiguous targets. The [`rustdoc::broken_intra_doc_links` lint][rustdoc~lints] warns about unresolved links.

## Generate and test the API docs

From `bk/`, run `just docs docall` to build and serve the workspace documentation for preview. To build public API pages without starting a server, run this command from `bk/crates/`:

```text
cargo doc --no-deps --locked --workspace
```

Run doctests for one changed library crate from `bk/crates/`:

```text
cargo test --doc --locked -p <crate>
```

Review rustdoc warnings and fix broken links or examples before submitting. If a doctest depends on a service, GUI, or platform-specific feature, document that limitation and keep the example's test behavior explicit.
