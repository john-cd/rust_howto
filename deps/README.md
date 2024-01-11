# Deps

The `examples` folder contains Rust code examples that are embedded into the book.

Run `cargo add <crate>` (or edit `Cargo.toml`) in this folder to add new dependencies that these examples may require.

## Common commands

```bash
cargo +nightly fmt --all
cargo clippy --examples
cargo check --examples
cargo build --examples
cargo test --tests --examples -- --show-output
cargo run --example <name>
```

To run all examples, use `just runall`.
