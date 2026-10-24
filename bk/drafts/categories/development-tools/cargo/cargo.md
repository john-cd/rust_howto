<!-- TODO Reviewed ??-??-2026 -->

# Cargo, the Rust Package Manager

{{#include cargo.incl.md}}

## Create a Package {#create-a-package}

[The Cargo book][c~cargo~book]↗ [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}{{hi:cargo}}

[`cargo`][c~cargo~docs]↗{{hi:cargo}}

Create a new binary package with `cargo new hello-cargo`. Use `cargo new --lib hello-utils` for a library, or `cargo init --bin existing-directory` to initialize an existing directory. Use `cargo help` to list commands or `cargo <command> --help` for command-specific options.

## Build and Run {#basic-cargo-usage}

Select a package in a workspace with `-p PACKAGE`. Optional features can be enabled with `--features FEATURE` or a comma-separated list. Use `--release` to select the optimized release profile instead of the default development profile.

```bash
cargo check -p my-app
cargo build --release -p my-app --features cli
cargo run --release -p my-app --features cli -- --help
```

Pass arguments to the program after `--`, as in the `cargo run` example. `cargo clean` removes generated build artifacts and is rarely needed during normal development; Cargo rebuilds outputs when inputs change.

## Test and Benchmark {#test-and-benchmark}

```bash
cargo test --workspace
cargo test -p my-crate --features serde
cargo bench -p my-crate
```

## Manage Dependencies {#manage-dependencies}

Cargo includes `cargo add` and `cargo remove` for editing dependencies. `cargo update` refreshes versions in `Cargo.lock` while respecting the version requirements in `Cargo.toml`; add `-p PACKAGE` to update one package in a workspace.

```bash
cargo add serde
cargo remove serde
cargo update
cargo update -p regex
```

For example, `cargo add serde@1` adds a version requirement to the manifest. Use `cargo add --help` for options for features, workspace dependencies, and registries.

## Build Documentation {#build-documentation}

```bash
cargo doc --workspace --no-deps --open
```

## `Cargo.toml` and Lock Files {#cargo-toml}

```toml
# Configure the package
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

# See more keys and their definitions at
# <https://doc.rust-lang.org/cargo/reference/manifest.html>

[dependencies]
serde = "1"
```

By default, a version string such as `"1"` is a caret requirement, allowing compatible releases below the next breaking major version. The exact resolved versions are recorded in `Cargo.lock`.

Examples of Cargo's default caret version requirements{{hi:Version requirements}}:

```text
1.2.3  :=  >=1.2.3, <2.0.0
1.2    :=  >=1.2.0, <2.0.0
1      :=  >=1.0.0, <2.0.0
0.2.3  :=  >=0.2.3, <0.3.0
0.2    :=  >=0.2.0, <0.3.0
0.0.3  :=  >=0.0.3, <0.0.4
0.0    :=  >=0.0.0, <0.1.0
0      :=  >=0.0.0, <1.0.0
```

See [Specifying Dependencies][book~cargo~specifying-dependencies]↗ for registry, Git, path, and workspace dependency forms.

For a standalone reusable library, maintainers commonly omit `Cargo.lock` from version control because downstream users resolve the library as part of their own dependency graph.

For applications, command-line tools, and workspaces, commit the lockfile so development and CI use the same resolved dependency versions.

## Publish a Crate {#publish-a-crate}

Review the files in the package and perform a dry run before publishing. Publishing a version to crates.io is permanent.

```bash
cargo package --list -p my-crate
cargo package -p my-crate
cargo publish --dry-run -p my-crate
# Publish only after reviewing the package and dry-run output.
cargo publish -p my-crate
```

## Related Topics {#related-topics .skip}

- [Package layout and workspaces][ex~cargo~package-layout].
- [Cargo plugins][p~cargo-plugins], including dependency management.
- [Versioning][p~versioning] and [Cargo registries][ex~cargo~crate-registries].
- [Release automation][p~publication-to-crates-io].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
