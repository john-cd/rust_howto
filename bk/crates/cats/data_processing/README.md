<!-- TODO Reviewed ??-??-2026. -->

# Data Processing

From `bk/crates`, run the examples using inline data:

```console
cargo run --locked -p data_processing --example dataframes
cargo run --locked -p data_processing --features arrow --example data_processing -- ipc
cargo run --locked -p data_processing --features datafusion --example datafusion
cargo test --locked -p data_processing --all-features --examples
```

Omit `ipc` to run all Arrow recipes, or use `--help` to list them.
