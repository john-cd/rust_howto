<!-- TODO Reviewed ??-??-2026. -->

# Development Tools Cargo Plugins

Book code examples for creating and using Cargo plugins and automating tasks with xshell. It demonstrates integration with key ecosystem crates including `anyhow`, `cargo_metadata`, `clap`, `clap-cargo`, `tempfile`, `tracing`.

## Examples Covered

- **Maintaining** (`examples/maintaining/`): Shell recipes for Cargo SemVer Checks and Cargo Hack.
- **Building** (`examples/building/`): Demonstrates concepts like `xshell`.
- **Write Cargo Plugin** (`examples/write_cargo_plugin/`): Demonstrates concepts like `cargo_subcommand`.

## Usage

To check or run examples in this crate:

```console
cargo check --package cargo --all-targets
cargo test --package cargo
```

Code examples correspond to chapters in `bk/src/categories/development-tools_cargo-plugins` or related book sections.
