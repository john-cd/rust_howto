# Examples

This folder contains examples that are embedded into the book.

Run `cargo add <crate>` in this folder or edit `Cargo.toml` in this folder to add dependencies that the Rust code embedded into the book markdown or the examples requires.

## Common commands

```bash
cargo fmt --all
cargo check --examples
cargo build --examples
cargo test --examples
cargo run --example <name>
```

To run all examples, use `just runall`.
