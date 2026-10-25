<!-- TODO Reviewed ??-??-2026 -->

# Analyze and Generate Rust Code

{{#include rust_source.incl.md}}

Tools that read or write Rust source need a parser, a code builder and a way to find the files of a project. The recipes below use `syn`, `quote`, `prettyplease` and `cargo_metadata` outside of a procedural macro.

## Parse Rust Source with `syn` {#syn-parse-file}

[![syn][c~syn~docs~badge]][c~syn~docs] [![syn~crates.io][c~syn~crates.io~badge]][c~syn~crates.io]{{hi:syn}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Rust source}}

[`syn`][c~syn~docs]↗{{hi:syn}} is best known for procedural macros, but `syn::parse_file` also parses ordinary source text in any program. The resulting syntax tree can be walked with a `syn::visit::Visit` implementation: override the `visit_*` methods you care about and call the matching `syn::visit::visit_*` function to keep descending. Enable the `full` feature to parse function bodies and statements, and `visit` for the visitor traits. Parse failures are returned as `syn::Error` values.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/rust_source/syn_file.rs:example}}
```

## Generate and Format Rust Code with `quote` and `prettyplease` {#quote-prettyplease}

[![quote][c~quote~docs~badge]][c~quote~docs] [![quote~crates.io][c~quote~crates.io~badge]][c~quote~crates.io] [![prettyplease][c~prettyplease~docs~badge]][c~prettyplease~docs] [![prettyplease~crates.io][c~prettyplease~crates.io~badge]][c~prettyplease~crates.io]{{hi:quote}}{{hi:prettyplease}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Code generation}}

[`quote`][c~quote~docs]↗{{hi:quote}} builds a `TokenStream` from a Rust-like template; `#name` splices in a value. A `TokenStream` prints on a single line, so parse it into a `syn::File` and pass it to [`prettyplease::unparse`][c~prettyplease~docs]↗{{hi:prettyplease}} to get readable, rustfmt-like output. This is the usual last step of a build script or code generator that writes `.rs` files.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/rust_source/quote_prettyplease.rs:example}}
```

## Query a Cargo Workspace with `cargo_metadata` {#cargo-metadata}

[![cargo_metadata][c~cargo_metadata~docs~badge]][c~cargo_metadata~docs] [![cargo_metadata~crates.io][c~cargo_metadata~crates.io~badge]][c~cargo_metadata~crates.io]{{hi:cargo_metadata}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Cargo}}

[`cargo_metadata`][c~cargo_metadata~docs]↗{{hi:cargo_metadata}} runs `cargo metadata` and deserializes its JSON output into typed structures: packages, targets, features and (without `no_deps`) the resolved dependency graph. Tools that inspect a project use it instead of parsing `Cargo.toml` by hand. `no_deps` makes the call faster and lets it work offline.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/rust_source/cargo_metadata.rs:example}}
```

## Write a Project Check that Finds `unwrap()` Calls {#find-unwraps}

[![syn][c~syn~docs~badge]][c~syn~docs] [![syn~crates.io][c~syn~crates.io~badge]][c~syn~crates.io] [![proc-macro2][c~proc-macro2~docs~badge]][c~proc-macro2~docs] [![proc-macro2~crates.io][c~proc-macro2~crates.io~badge]][c~proc-macro2~crates.io]{{hi:syn}}{{hi:proc-macro2}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Linting}}

Project-specific rules that `clippy` does not cover can be written in Rust, for example as a `cargo xtask`-style binary that CI runs: a small workspace member, started through a Cargo alias such as `xtask = "run -p xtask --"` in `.cargo/config.toml`, that exits with a non-zero status when it finds a violation. This check parses source text with [`syn`][c~syn~docs]↗{{hi:syn}} and reports each `unwrap` and `expect` call with its line. Line numbers need the `span-locations` feature of [`proc-macro2`][c~proc-macro2~docs]↗{{hi:proc-macro2}}. Combine it with [`cargo_metadata`][c~cargo_metadata~docs]↗{{hi:cargo_metadata}} to find the source files of every workspace member.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/rust_source/find_unwraps.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[parsing | Parsing Utilities]].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
