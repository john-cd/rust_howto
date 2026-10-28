<!-- TODO Reviewed ??-??-2026 -->

# Embedded

This crate contains code examples for the **Embedded** category on crates.io.

## Examples Covered

- **Embassy** (`examples/embassy/`): Demonstrates concepts like `embassy`.
- **Flash** (`examples/flash/`): Code examples covering flash.
- **Hals** (`examples/hals/`): Code examples covering hals.
- **RTOS** (`examples/rtos/`): Code examples covering rtos.
- **Sensors** (`examples/sensors/`): Code examples covering sensors.

## Usage

To check or run examples in this crate:

```console
cargo check --package embedded --examples
cargo test --package embedded
```

Code examples correspond to chapters in `bk/src/categories/embedded` or related book sections.

This package is a standalone workspace, kept outside the broad `later/crates` workspace so its host examples have independent package metadata and a lockfile.

The default examples run on the host and use a simulated pin, I2C bus, scheduler, and NOR flash. The registered Embassy example uses Embassy's standard-library executor and timer driver to test two async tasks exchanging messages through a bounded channel. The RP2040 firmware remains a non-compiled book snippet.

From the repository root, validate them with:

```powershell
cargo test --manifest-path later/crates/cats/embedded/Cargo.toml --locked --all-targets
cargo clippy --manifest-path later/crates/cats/embedded/Cargo.toml --locked --all-targets -- -D warnings
cargo +nightly-2026-10-22 fmt --manifest-path later/crates/cats/embedded/Cargo.toml --all -- --check
```

The standalone Embassy workspace demonstrates host task behavior; board-specific firmware uses a matching HAL and target configuration. See [the Embassy chapter](../../../src/categories/embedded/embassy.md) and [#751](https://github.com/john-cd/rust_howto/issues/751).
