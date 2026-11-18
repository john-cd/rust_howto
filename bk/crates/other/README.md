<!-- TODO Reviewed ??-??-2026. -->

# Other Book Examples

This package contains runnable examples that do not fit a crates.io category.
The Architecture example currently demonstrates repository, state-machine,
dependency-injection, layered, hexagonal, event-driven, pipeline, CQRS, and
event-sourcing patterns.

From `bk/crates`, run the example help or a specific demonstration:

```console
cargo run --locked -p other --example architecture -- --help
cargo run --locked -p other --example architecture -- repository
```

To validate the example and its tests, run:

```console
cargo test --locked -p other --all-targets --all-features
```
