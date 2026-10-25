<!-- TODO Reviewed ??-??-2026 -->

# README

This package is a standalone workspace, kept outside the broad `later/crates` workspace so its host examples have independent package metadata and a lockfile.

The default examples run on the host and use a simulated pin, I2C bus, scheduler, and NOR flash. From the repository root, validate them with:

```powershell
cargo test --manifest-path later/crates/cats/embedded/Cargo.toml --locked --all-targets
cargo clippy --manifest-path later/crates/cats/embedded/Cargo.toml --locked --all-targets -- -D warnings
cargo +nightly-2026-10-22 fmt --manifest-path later/crates/cats/embedded/Cargo.toml --all -- --check
```

The Embassy sketch is not registered as a Cargo target; it remains a board-specific book snippet until a supported board and executor setup is available (tracked by [#751](https://github.com/john-cd/rust_howto/issues/751)).
