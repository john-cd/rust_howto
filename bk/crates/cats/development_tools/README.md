<!-- TODO Reviewed ??-??-2026. -->

# Development Tools

This crate contains code examples for the **Development Tools** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `cargo_metadata`, `clap`, `crates_io_api`, `doc-comment`, `gix`.

## Examples Covered

- **Automation** (`examples/automation/`): Demonstrates concepts like `git_churn`, `workspace_lints`.
- **Crate Indices** (`examples/crate_indices/`): Demonstrates concepts like `alphabetical_page`, `category_page`, `dependencies`, `fixtures`, `published_recipes`.
- **Crate Registries** (`examples/crate_registries/`): Demonstrates concepts like `crates_io`.
- **Documentation** (`examples/documentation/`): Demonstrates concepts like `doc_comment`, `rustdoc`, `rustdoc2`, `rustdoc3`.
- **Formal verification** (`examples/creusot_contract/`): An independent workspace pinned to Creusot 0.14.0 and `nightly-2026-08-03`.
- **Rust Source** (`examples/rust_source/`): Demonstrates concepts like `cargo_metadata`, `find_unwraps`, `quote_prettyplease`, `syn_file`, `wasm_roundtrip`.
- **Semver** (`examples/semver/`): Demonstrates concepts like `semver_command`, `semver_complex`, `semver_latest`, `semver_parse`, `semver_prerelease`.
- **Standalone Examples**: `rustdoc4`.

## Usage

To check or run examples in this crate:

```console
cargo check --package development_tools --examples
cargo test --package development_tools
```

Code examples correspond to chapters in `bk/src/categories/development-tools` and related book sections.
