<!-- TODO Reviewed ??-??-2026 -->

# Write a Cargo Plugin

{{#include write_cargo_plugin.incl.md}}

Cargo runs any executable named `cargo-<name>` found in `PATH` (or in `~/.cargo/bin`) when the user types `cargo <name>`. Cargo passes the subcommand name as the first argument, so `cargo hello --workspace` runs `cargo-hello hello --workspace`. A plugin is an ordinary binary crate; to test it, see [[testing | Testing]] and the `assert_cmd` recipe there.

To publish it, name the package `cargo-<name>` so that `cargo install cargo-<name>` installs a `cargo-<name>` binary.

This example is built as the `cargo-hello` executable. Its entry point forwards the arguments from `std::env::args_os()` to the parser, so Cargo's extra `hello` argument is handled by the same code shown below.

## Write a Cargo Subcommand {#write-cargo-subcommand}

[![clap-cargo][c~clap-cargo~docs~badge]][c~clap-cargo~docs]{{hi:clap-cargo}} [![clap-cargo~crates.io][c~clap-cargo~crates.io~badge]][c~clap-cargo~crates.io] [![clap-cargo~repo][c~clap-cargo~repo~badge]][c~clap-cargo~repo] [![clap-cargo~lib.rs][c~clap-cargo~lib.rs~badge]][c~clap-cargo~lib.rs] [![cargo_metadata][c~cargo_metadata~docs~badge]][c~cargo_metadata~docs]{{hi:cargo_metadata}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

The example declares the arguments with [`clap`][c~clap~docs]↗:

- `#[command(bin_name = "cargo")]` and a one-variant enum make clap accept (and skip) the subcommand name that Cargo passes.
- [`clap_cargo::Workspace`][c~clap-cargo~docs]↗ and `clap_cargo::Features` add `-p`/`--package`, `--workspace`, `--exclude`, `--features`, `--all-features` and `--no-default-features`, with the same meaning as in Cargo itself.
- [`cargo_metadata`][c~cargo_metadata~docs]↗ runs `cargo metadata`; `Features::forward_metadata` passes the feature flags on, and `Workspace::partition_packages` returns the packages selected by the flags. Do not call `no_deps` on the `MetadataCommand`.

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_cargo_plugins/examples/write_cargo_plugin/cargo_subcommand.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[building | Build and Run Cargo Projects]]
- [[maintaining | Maintaining]]
- [Testing Rust code](../development-tools_testing/testing.md)
- [[command-line-interface | Command Line Interface]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [write and test a cargo subcommand](https://github.com/john-cd/rust_howto/issues/1951)
</div>
