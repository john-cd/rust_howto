# Deps

The `tests` and `examples` folders contain Rust code that is embedded into the book.

Run `cargo add <crate>` (or edit `Cargo.toml`) in this folder to add new dependencies that these examples may require.

## Common commands

Type `just` to browse common commands, or manually enter the following:

```bash
cargo +nightly fmt --all

mdbook build
# then
cargo check --examples
# or
cargo build --examples
# or
cargo clippy --examples
# or
cargo test --tests --examples -- --show-output
# or
cargo run --example <name>
```

To run all examples (but not the tests), use `just runall`.
