<!-- TODO Reviewed ??-??-2026. -->

# No Allocation

From `bk/crates`, run the host demonstration.

```console
cargo run --locked -p no_std_no_alloc --example no_alloc
cargo test --locked -p no_std_no_alloc
```

The example uses `core` and `heapless` with default features disabled.
