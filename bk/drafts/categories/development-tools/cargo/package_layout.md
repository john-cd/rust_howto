<!-- TODO Reviewed ??-??-2026 -->

# Package Layout {#package-layout}

{{#include package_layout.incl.md}}

```text
.
├── Cargo.lock
├── Cargo.toml
├── build.rs                      # Optional package build script.
├── src/
│   ├── lib.rs                      # The default library file is src/lib.rs.
│   ├── main.rs                     # The default executable file is src/main.rs.
│   └── bin/                        # Other executables can be placed in src/bin/,
│       ├── named-executable.rs     # even in library projects.
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs                   # `cargo run --example simple`
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/                          # Each top-level .rs file is an integration-test target.
    ├── api.rs                      # Tests in src files are unit tests; docs are doctests.
    └── api/                         # Modules used by api.rs can live alongside it.
        └── helpers.rs
```

Cargo discovers conventional targets from these paths: `src/lib.rs` for a library, `src/main.rs` for the default executable, Rust files in `src/bin/` for additional executables, and top-level Rust files in `examples/`, `tests/`, and `benches/` for those target kinds. Put modules beside a target file or below its same-named directory. Use `[[bin]]`, `[[example]]`, `[[test]]`, or `[[bench]]` in `Cargo.toml` when a target needs a nonstandard path or settings.

Put `build.rs` beside the package manifest when a build script is needed. It runs before the package is compiled and is described by the package's `[build-dependencies]`; it is not an application entry point.

- A package{{hi:Package}} is a bundle of one or more crates - as defined by a [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} file.
- A crate{{hi:Crate}} is the smallest amount of code that the Rust compiler considers at a time.
- A crate can come in one of two forms: a binary crate (must have a function called [`main`][book~rust-reference~main]↗{{hi:main}}) or a library crate.
- A package can contain as many binary crate{{hi:Binary crate}}s as needed, but at most only one library crate{{hi:Library crate}}.
- If a package contains `src/main.rs`{{hi:src/main.rs}} and `src/lib.rs`{{hi:src/lib.rs}}, it has two crates: a binary and a library, both with the same name as the package.

## Arrange a Cargo Workspace {#workspace-layout}

A workspace groups related packages under one root manifest. A virtual workspace manifest has a `[workspace]` table but no `[package]` table. Cargo uses one root `Cargo.lock` and, by default, a shared `target/` directory for its member packages.

```text
.
├── Cargo.toml                  # Virtual workspace manifest.
├── Cargo.lock
└── crates/
    ├── cli/
    │   ├── Cargo.toml
    │   └── src/main.rs
    └── core/
        ├── Cargo.toml
        └── src/lib.rs
```

Declare members and shared dependency versions at the workspace root, then opt member packages into those dependencies:

```toml
# Workspace Cargo.toml
[workspace]
members = ["crates/*"]
resolver = "3"

[workspace.dependencies]
serde = "1"
```

```toml
# crates/core/Cargo.toml
[dependencies]
serde = { workspace = true }
```

Commit the workspace lockfile so members and CI use the same resolved dependency versions. For a standalone reusable library, maintainers commonly omit its lockfile from version control; applications generally commit theirs.

## Related Topics {#related-topics .skip}

- [Cargo commands][ex~cargo~basic-cargo-usage], including package selection.
- [Cargo workspaces][book~cargo~workspaces]↗ and shared dependencies.
- Modules and paths.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
