# Deps

The `tests` folder contains the Rust code that is embedded into the book.

Run `cargo add <crate>` (or edit `Cargo.toml`) in this folder to add new dependencies that the code may require.

## Common commands

Type `just` to browse common commands, or manually enter the following:

```bash
cargo +nightly fmt --all
# then
cargo check --tests
# or
cargo build --tests
# or
cargo clippy --tests
# or
cargo test -- --show-output
```

## Folders

`tests` contains the current book examples, written as tests.
`drafts` contains WIP book examples.
`src` contains a stub `lib.rs` file
`temp` contains the outputs of the tests.
