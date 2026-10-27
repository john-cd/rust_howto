<!-- TODO Reviewed ??-??-2026 -->

# README

This package is part of the `later/crates` workspace. Its host examples use a simulated pin, I2C bus, scheduler, and NOR flash.

From `later/crates`, validate the examples with:

```powershell
cargo test -p embedded --locked --all-targets
cargo clippy -p embedded --locked --all-targets -- -D warnings
cargo +nightly-2026-10-22 fmt --package embedded --all -- --check
```

The Embassy sketch is not registered as a Cargo target; it remains a board-specific book snippet until a supported board and executor setup is available (tracked by [#751](https://github.com/john-cd/rust_howto/issues/751)).
