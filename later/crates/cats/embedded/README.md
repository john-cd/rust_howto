<!-- TODO Reviewed ??-??-2026 -->

# README

This package is part of the `later/crates` workspace. Its examples use a simulated pin, I2C bus, scheduler, and NOR flash. The registered Embassy example uses Embassy's standard-library executor and timer driver to test two async tasks exchanging messages through a bounded channel. The RP2040 firmware remains a non-compiled book snippet.

From `later/crates`, validate the examples with:

```powershell
cargo test -p embedded --locked --all-targets
cargo clippy -p embedded --locked --all-targets -- -D warnings
cargo +nightly-2026-10-22 fmt --package embedded --all -- --check
```

The standalone Embassy workspace demonstrates host task behavior; board-specific firmware uses a matching HAL and target configuration. See [the Embassy chapter](../../../src/categories/embedded/embassy.md) and [#751](https://github.com/john-cd/rust_howto/issues/751).
