<!-- TODO Reviewed ??-??-2026 -->

# `xtask` Project Checks

{{#include xtask.incl.md}}

The `xtask` pattern puts repository automation in a regular workspace member, so contributors run `cargo xtask <command>` and CI runs the same command. Checks that `clippy` cannot express, such as naming rules, architecture boundaries or manifest conventions, become ordinary Rust programs that exit with a non-zero status when they find a violation.

## Lay Out an `xtask` Crate {#xtask-layout}

[![cargo-xtask][c~cargo-xtask~repo~badge]][c~cargo-xtask~repo]{{hi:xtask}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Automation}}

Add a binary crate named `xtask` to the workspace and give it a Cargo alias in `.cargo/config.toml` at the workspace root:

```toml
# .cargo/config.toml
[alias]
xtask = "run --package xtask --"
```

```toml
# Cargo.toml (workspace root)
[workspace]
members = ["xtask", "crates/*"]
```

`cargo xtask lints` then runs `xtask`'s `main` with the argument `lints`. Use [`clap`][c~clap~docs]↗{{hi:clap}} for subcommands, keep each check in its own module with a function that returns the list of findings, and make `main` print the findings and exit with a failing status if the list is not empty. Mark the crate `publish = false`. The checks below are written so that a unit test can feed them a string, which keeps them testable without touching the file system.

## Check that Every Member Uses the Workspace Lints {#workspace-lints}

[![cargo_metadata][c~cargo_metadata~docs~badge]][c~cargo_metadata~docs] [![toml_edit][c~toml_edit~docs~badge]][c~toml_edit~docs]{{hi:cargo_metadata}}{{hi:toml_edit}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Linting}}

A `[workspace.lints]` table is only applied to members that opt in with `[lints] workspace = true`. [`cargo_metadata`][c~cargo_metadata~docs]↗{{hi:cargo_metadata}} lists the manifests of the members, and [`toml_edit`][c~toml_edit~docs]↗{{hi:toml_edit}} parses them. Unlike `toml`, `toml_edit` preserves comments and formatting, so the same code can also fix a manifest in place.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/automation/workspace_lints.rs:example}}
```

## Find `unwrap()` Calls {#find-unwraps}

[![syn][c~syn~docs~badge]][c~syn~docs]{{hi:syn}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Linting}}

Walking the syntax tree with [`syn`][c~syn~docs]↗{{hi:syn}} finds patterns a text search would get wrong. See [Write a Project Check that Finds `unwrap()` Calls][ex~rust_source~find-unwraps] for the complete example.

## Related Topics {#related-topics .skip}

- [[git | Git History with `gix`]]
- [[rust_source | Analyze and Generate Rust Code]]

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
