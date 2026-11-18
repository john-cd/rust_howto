<!-- TODO Reviewed ??-??-2026. -->

# Other Book Examples

This package contains runnable examples for architecture, AWS service clients, Crux, scheduling, Rhai, and task automation. The AWS example is enabled by the `aws-sdk` feature.

From `bk/crates`, run the example help or a specific demonstration:

```console
cargo run --locked -p other --example architecture -- --help
cargo run --locked -p other --example architecture -- repository
cargo run --locked -p other --features aws-sdk --example aws_sdk -- aws_message_bus
```

To validate all the examples, including the offline AWS tests, run:

```console
cargo test --locked -p other --all-targets --all-features
```
