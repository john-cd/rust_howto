<!-- TODO Reviewed ??-??-2026. -->

# OS Linux APIS

This crate contains code examples for the **OS Linux APIS** category on crates.io.

## Examples Covered

- **Linux** (`examples/linux/`): Demonstrates concepts like `statx_metadata`.

## Usage

To check or run examples in this crate:

```console
cargo check --package os_linux_apis --examples
cargo test --package os_linux_apis
```

Code examples correspond to chapters in `bk/src/categories/os_linux-apis` or related book sections.

## Portable Helper Contract

The `process_environment` example runs on Windows and Unix without exercising statx:

```sh
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p os_linux_apis --example process_environment
```

Its chapter is `bk/src/other/native-os/index.md`. Native statx tests require Linux;
a successful fallback or cross-target check does not establish Linux runtime behavior.
