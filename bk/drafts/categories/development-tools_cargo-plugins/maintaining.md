<!-- TODO Reviewed ??-??-2026 -->

# Maintain Cargo Projects with Plugins

{{#include maintaining.incl.md}}

Use Cargo's built-in commands for routine build, test, and publish tasks. These plugins help inspect a crate's public API, inspect macro expansion, test feature combinations, and manage the local Cargo cache. For dependency declarations, formatting, auditing, and general testing, use the sibling guides linked below.

## Check Public API Changes for SemVer Breakage {#cargo-semver-checks}

[![cargo-semver-checks][c~cargo-semver-checks~docs~badge]][c~cargo-semver-checks~docs] [![cargo-semver-checks~crates.io][c~cargo-semver-checks~crates.io~badge]][c~cargo-semver-checks~crates.io] [![cargo-semver-checks~repo][c~cargo-semver-checks~repo~badge]][c~cargo-semver-checks~repo] [![cargo-semver-checks~lib.rs][c~cargo-semver-checks~lib.rs~badge]][c~cargo-semver-checks~lib.rs] [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]

[`cargo-semver-checks`][c~cargo-semver-checks~docs] checks a crate's public API against a compatibility baseline. Run it before publishing a library release; review the reported changes because a detected API difference still needs human SemVer judgment.

```sh
{{#include ../../../crates/cats/development_tools/examples/cargo_plugins/semver_checks.sh:example}}
```

## Inspect Macro Expansion with `cargo-expand` {#cargo-expand}

[![cargo-expand][c~cargo-expand~docs~badge]][c~cargo-expand~docs] [![cargo-expand~crates.io][c~cargo-expand~crates.io~badge]][c~cargo-expand~crates.io] [![cargo-expand~repo][c~cargo-expand~repo~badge]][c~cargo-expand~repo] [![cargo-expand~lib.rs][c~cargo-expand~lib.rs~badge]][c~cargo-expand~lib.rs] [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]

`cargo expand` prints the code produced by macros and derives, which is useful when debugging generated implementations. Treat the expanded output as a diagnostic view; it is not a replacement for the source or a stable compiler interface.

## Inspect and Reclaim the Cargo Cache with `cargo-cache` {#cargo-cache}

[![cargo-cache][c~cargo-cache~docs~badge]][c~cargo-cache~docs] [![cargo-cache~crates.io][c~cargo-cache~crates.io~badge]][c~cargo-cache~crates.io] [![cargo-cache~repo][c~cargo-cache~repo~badge]][c~cargo-cache~repo] [![cargo-cache~lib.rs][c~cargo-cache~lib.rs~badge]][c~cargo-cache~lib.rs] [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]

`cargo-cache` reports the size of Cargo's local downloads, registry sources, Git checkouts, and build artifacts, and can remove selected cache contents. Check which directories a cleanup option affects before using it; deleting cached build artifacts only causes Cargo to rebuild them.

## Test Feature Combinations with `cargo-hack` {#cargo-hack}

[![cargo-hack][c~cargo-hack~docs~badge]][c~cargo-hack~docs] [![cargo-hack~crates.io][c~cargo-hack~crates.io~badge]][c~cargo-hack~crates.io] [![cargo-hack~repo][c~cargo-hack~repo~badge]][c~cargo-hack~repo] [![cargo-hack~lib.rs][c~cargo-hack~lib.rs~badge]][c~cargo-hack~lib.rs] [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]

`cargo-hack` can check each feature independently or test feature combinations. Feature powersets grow exponentially, so reserve a full powerset run for crates with a manageable number of features.

```sh
{{#include ../../../crates/cats/development_tools/examples/cargo_plugins/cargo_hack.sh:example}}
```

## Related Topics {#related-topics .skip}

- [[dependency_management | Manage Dependencies]]
- [[code_formatting_linting | Format and Lint Code]]
- [[auditing | Audit Dependencies and Releases]]
- [Testing Rust code](../development-tools_testing/testing.md)
- [[building | Build and Run Cargo Projects]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
