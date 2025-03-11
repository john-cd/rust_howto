# Write code

{{#include code_writing.incl.md}}

## Generate a Rust project from a template {#cargo-generate}

[![cargo-generate][c-cargo_generate-badge]][c-cargo_generate]{{hi:cargo-generate}}
[![cargo-generate-crates.io][c-cargo_generate-crates.io-badge]][c-cargo_generate-crates.io]
[![cargo-generate-github][c-cargo_generate-github-badge]][c-cargo_generate-github]
[![cargo-generate-lib.rs][c-cargo_generate-lib.rs-badge]][c-cargo_generate-lib.rs]

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:cargo plugins}}

[Cargo Generate][c-cargo_generate-crates.io]{{hi:cargo-generate}}⮳ is a developer tool to help you get up and running quickly with a new Rust project by leveraging a pre-existing git repository as a template.

## Quickly open the `crates.io` or `docs.rs` page for the latest version of a crate {#cargo-open}

[![cargo-crates][c-cargo_crates-badge]][c-cargo_crates]{{hi:cargo-crates}}
[![cargo-crates-crates.io][c-cargo_crates-crates.io-badge]][c-cargo_crates-crates.io]
[![cargo-crates-github][c-cargo_crates-github-badge]][c-cargo_crates-github]
[![cargo-crates-lib.rs][c-cargo_crates-lib.rs-badge]][c-cargo_crates-lib.rs]

`cargo crates` is a [cargo][p-cargo] command to quickly open the [`crates.io`][crates.io-website]{{hi:crates.io}}⮳ or[`docs.rs`][docs-rs]{{hi:docs.rs}}⮳ page for the latest version of a crate.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/924)

Code Generation (Procedural Macros): (Procedural [macros][p-macros] are the primary way to do code generation in Rust. There aren't specific [cargo][p-cargo] plugins for writing proc [macros][p-macros], but they are often used within projects that might also have other build-related plugins.)

Code Snippets/Templates: (No single dominant crate or plugin. Often handled with IDE features or custom scripts.)

Scaffolding/Project Generation: (Tools like [cargo][p-cargo] new are built-in. Other project templates might be managed separately or integrated into build tools.)

Documentation Generation: `cargo doc` (while primarily for documentation output, it does involve processing and "generating" [documentation][p-documentation] from your code).

API [Documentation][p-documentation] Generators (for REST APIs, etc.): (Often tied to web frameworks; no single dominant crate).

String Manipulation/Text Processing (Often used in code generation): [regex][p-regex], [`itertools`][c-itertools]⮳{{hi:itertools}} (not [`cargo`][c-cargo]⮳{{hi:cargo}} plugins, but commonly used crates).
</div>
