<!-- TODO Reviewed ??-??-2026 -->

# Write Code

{{#include code_writing.incl.md}}

| Task                                      | Tool                                                                                                  |
| ----------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| Expand macros and conditional compilation | [`cargo-expand`][ex~development-tools_cargo-plugins~cargo-expand]↗                                    |
| Generate a project from a template        | [`cargo-generate`][ex~development-tools_cargo-plugins~cargo-generate]↗                                |
| Inspect module structure                  | [`cargo-modules`][ex~development-tools_cargo-plugins~cargo-modules]↗                                  |
| Check feature combinations                | [`cargo-hack`][ex~development-tools_cargo-plugins~cargo-hack]↗                                        |
| Add or remove dependencies                | Built-in [`cargo add`][book~cargo~cargo-add]↗ and [`cargo remove`][book~cargo~cargo-remove]↗ commands |

For dependency updates, [`cargo-edit`][c~cargo-edit~docs]↗{{hi:cargo-edit}} currently provides `cargo upgrade` and `cargo set-version`; current Cargo versions already include `cargo add` and `cargo remove`.

## Expand a Crate for Inspection {#cargo-expand}

[`cargo-expand`][c~cargo-expand~repo]↗ prints macro-expanded source for inspection. Install it and expand the current crate with:

```bash
cargo install cargo-expand
cargo expand
```

## Generate a Rust Project from a Template {#cargo-generate}

[![cargo-generate][c~cargo-generate~docs~badge]][c~cargo-generate~docs]{{hi:cargo-generate}}
[![cargo-generate~crates.io][c~cargo-generate~crates.io~badge]][c~cargo-generate~crates.io]
[![cargo-generate~repo][c~cargo-generate~repo~badge]][c~cargo-generate~repo]
[![cargo-generate~lib.rs][c~cargo-generate~lib.rs~badge]][c~cargo-generate~lib.rs]

[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:cargo plugins}}

[Cargo Generate][c~cargo-generate~crates.io]↗{{hi:cargo-generate}} is a developer tool to get up and running quickly with a new Rust project by leveraging a pre-existing git repository as a template.

Install it and pass a template repository URL to `cargo generate`:

```bash
cargo install cargo-generate
cargo generate --git https://github.com/user/template
```

## Inspect Module Structure {#cargo-modules}

[`cargo-modules`][c~cargo-modules~repo]↗ analyzes a crate's module tree. Install the subcommand and print the current crate's structure with:

```bash
cargo install cargo-modules
cargo modules structure
```

## Check Feature Combinations {#cargo-hack}

[`cargo-hack`][c~cargo-hack~repo]↗ adds commands for checking feature combinations and other workspace-wide Cargo operations. Its `--each-feature` option runs the selected Cargo operation once per feature:

```bash
cargo install cargo-hack
cargo hack check --each-feature
```

## Quickly Open the `crates.io` or `docs.rs` Page for the Latest Version of a Crate {#cargo-open}

[![cargo-crates][c~cargo-crates~docs~badge]][c~cargo-crates~docs]{{hi:cargo-crates}}
[![cargo-crates~crates.io][c~cargo-crates~crates.io~badge]][c~cargo-crates~crates.io]
[![cargo-crates~repo][c~cargo-crates~repo~badge]][c~cargo-crates~repo]
[![cargo-crates~lib.rs][c~cargo-crates~lib.rs~badge]][c~cargo-crates~lib.rs]

`cargo crates` is a [cargo][p~cargo] command to quickly open the [`crates.io`][crates.io~website]↗{{hi:crates.io}} or [`docs.rs`][docs.rs~website]↗{{hi:docs.rs}} page for the latest version of a crate.

## Related Topics {#related-topics .skip}

| Topic                                                                 | Rust Crates                                                                                                                                                                                   |
| --------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Documentation Generation                                              | [`cargo doc`][book~cargo~cargo-doc]↗{{hi:cargo doc}} (while primarily for documentation output, it does involve processing and "generating" [documentation][p~documentation] from your code). |
| API [Documentation][p~documentation] Generators (for REST APIs, etc.) | Often tied to web frameworks; no single dominant crate.                                                                                                                                       |
| String Manipulation / Text Processing (Often used in code generation) | [regex][p~regex], [`itertools`][c~itertools~docs]↗{{hi:itertools}} (not [`cargo`][c~cargo~docs]↗{{hi:cargo}} plugins, but commonly used crates).                                              |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
