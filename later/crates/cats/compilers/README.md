<!-- TODO Reviewed ??-??-2026. -->

# Compilers

Run `cargo run --locked -p compilers --example source_diagnostics` from
`later/crates` to turn a Logos lexer error into a labeled source diagnostic.

Examples focused on compiler design and incremental computation using Salsa and Comemo. It demonstrates integration with key ecosystem crates including `anyhow`, `clap`, `comemo`, `logos`, `salsa`.

## Examples Covered

- **Incremental Computation** (`examples/incremental_computation/`): Demonstrates concepts like `comemo`, `salsa`.
- **Tokenization** (`examples/tokenization/`): Demonstrates concepts like `tokenization`.

## Usage

To check or run examples in this crate:

```console
cargo check --package compilers --examples
cargo test --package compilers
```

Code examples correspond to chapters in `bk/src/categories/compilers` or related book sections.
