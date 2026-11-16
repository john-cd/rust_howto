<!-- TODO Reviewed ??-??-2026. -->

# No Standard Library

From `bk/crates`, check the checksum library without its default `std` feature
and run the host example.

```console
cargo check --locked -p no_std --lib --no-default-features
cargo run --locked -p no_std --example no_std
```
