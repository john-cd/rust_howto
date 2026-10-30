<!-- TODO Reviewed ??-??-2026 -->

# Tools for Macro Development

{{#include macro_tools.incl.md}}

## `cargo expand` {#cargo-expand}

[![cargo-expand][c~cargo-expand~docs~badge]][c~cargo-expand~docs]{{hi:cargo-expand}}
[![cargo-expand~crates.io][c~cargo-expand~crates.io~badge]][c~cargo-expand~crates.io]
[![cargo-expand~repo][c~cargo-expand~repo~badge]][c~cargo-expand~repo]
[![cargo-expand~lib.rs][c~cargo-expand~lib.rs~badge]][c~cargo-expand~lib.rs]
[![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}
[![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}

`cargo expand` prints the Rust code produced after macro expansion, which helps inspect generated code and debug macros. It uses rustc's unstable expansion interface, so use a nightly toolchain when required. Install nightly `rustfmt` for formatted output; `--ugly` skips formatting.

```bash
cargo install cargo-expand
rustup component add rustfmt --toolchain nightly

# With nightly selected for the project:
cargo expand --lib path::to::module

# Or select nightly for this command only:
cargo +nightly expand --lib path::to::module
```

Expanded output is for inspection and debugging. Treat it as a view of generated code, not a replacement for the macro's source or a stable API.

## Test Macro Diagnostics with `trybuild` {#trybuild}

[`trybuild`][c~trybuild~docs]↗ runs compile-pass and compile-fail UI tests through `cargo test`. Add it as a development dependency, keep input cases under `tests/ui/`, and review expected `.stderr` diagnostics when a test changes. This is useful for checking that invalid macro input produces clear compiler errors.

```bash
cargo add --dev trybuild
cargo test
```

## Snapshot Macro Expansions with `macrotest` {#macrotest}

[`macrotest`][macrotest~docs]↗ compares expanded test cases with checked-in `*.expanded.rs` snapshots. Install `cargo-expand`, add `macrotest` as a development dependency, and call `macrotest::expand("tests/expand/*.rs")` from a test. Review snapshot changes like source changes; use `macrotest::expand_without_refresh` in CI when snapshots must not be rewritten.

```bash
cargo add --dev macrotest
cargo test
```

## Snapshot Generated Code in a Unit Test {#snapshot-generated-code}

`macrotest` needs the compiler's expansion interface. When a macro's logic lives in an ordinary function from `proc_macro2::TokenStream` to `TokenStream`, a plain unit test can do the same job: format the output with [`prettyplease`][c~prettyplease~docs]↗{{hi:prettyplease}} and compare it with the expected source.

```rust,editable
{{#include ../../../crates/cats/development_tools_procedural_macro_helpers/examples/macro_tools/main.rs:example}}
```

## Expand Macros in Rust Analyzer {#rust-analyzer-expand}

In Rust Analyzer, place the caret on a macro invocation and run **Expand macro recursively** from the command palette or editor context menu. The editor shows the expansion without changing the source file; see the [Rust Analyzer feature documentation][rust-analyzer~macro-expansion~docs].

## Related Topics {#related-topics .skip}

- [Procedural macros][p~macros] and code generation.
- Compile-time tooling.
- Derive and attribute macros.
- Development productivity tools.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
