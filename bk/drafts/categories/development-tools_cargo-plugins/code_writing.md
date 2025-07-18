# Write Code

{{#include code_writing.incl.md}}

| Topic | Rust Crates |
|---|---|
| Code Generation (Procedural Macros) | Procedural [macros][p~macros] are the primary way to do code generation in Rust. There aren't specific [cargo][p~cargo] plugins for writing proc [macros][p~macros], but they are often used within projects that might also have other build-related plugins. |
| Code Snippets/Templates | No single dominant crate or plugin. Often handled with IDE features or custom scripts. |
| Scaffolding/Project Generation | Tools like `cargo new` are built-in. Other project templates might be managed separately or integrated into build tools. |

## Generate a Rust Project from a Template {#cargo-generate}

[![cargo-generate][c~cargo_generate~docs~badge]][c~cargo_generate~docs]{{hi:cargo-generate}}
[![cargo-generate~crates.io][c~cargo_generate~crates.io~badge]][c~cargo_generate~crates.io]
[![cargo-generate~github][c~cargo_generate~github~badge]][c~cargo_generate~github]
[![cargo-generate~lib.rs][c~cargo_generate~lib.rs~badge]][c~cargo_generate~lib.rs]

[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:cargo plugins}}

[Cargo Generate][c~cargo_generate~crates.io]{{hi:cargo-generate}}⮳ is a developer tool to help you get up and running quickly with a new Rust project by leveraging a pre-existing git repository as a template.

## Quickly Open the `crates.io` or `docs.rs` Page for the Latest Version of a Crate {#cargo-open}

[![cargo-crates][c~cargo_crates~docs~badge]][c~cargo_crates~docs]{{hi:cargo-crates}}
[![cargo-crates~crates.io][c~cargo_crates~crates.io~badge]][c~cargo_crates~crates.io]
[![cargo-crates~github][c~cargo_crates~github~badge]][c~cargo_crates~github]
[![cargo-crates~lib.rs][c~cargo_crates~lib.rs~badge]][c~cargo_crates~lib.rs]

`cargo crates` is a [cargo][p~cargo] command to quickly open the [`crates.io`][crates.io~website]{{hi:crates.io}}⮳ or[`docs.rs`][docs.rs~website]{{hi:docs.rs}}⮳ page for the latest version of a crate.

## Related Topics

| Topic | Rust Crates |
|---|---|
| Documentation Generation | `cargo doc` (while primarily for documentation output, it does involve processing and "generating" [documentation][p~documentation] from your code). |
| API [Documentation][p~documentation] Generators (for REST APIs, etc.) | Often tied to web frameworks; no single dominant crate. |
| String Manipulation / Text Processing (Often used in code generation) | [regex][p~regex], [`itertools`][c~itertools~docs]⮳{{hi:itertools}} (not [`cargo`][c~cargo~docs]⮳{{hi:cargo}} plugins, but commonly used crates). |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/924)
</div>
