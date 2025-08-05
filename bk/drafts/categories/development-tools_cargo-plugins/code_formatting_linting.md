# Code Formatting and Linting

{{#include code_formatting_linting.incl.md}}

| Topic | Rust Crates |
|---|---|
| Linting | `cargo clippy` is the primary linter for Rust code, catching stylistic issues and potential bugs. `rust-analyzer`: While primarily an LSP (Language Server Protocol) implementation for IDEs, it also performs code analysis checks. |
| Formatting | `cargo fmt` is the standard Rust code formatter. |
| Dead Link Detection | `cargo deadlinks` finds broken links in your documentation. |

## Format Your Code {#rustfmt}

[![rustfmt-nightly][c~rustfmt_nightly~docs~badge]][c~rustfmt_nightly~docs]{{hi:rustfmt-nightly}}
[![rustfmt-nightly~crates.io][c~rustfmt_nightly~crates.io~badge]][c~rustfmt_nightly~crates.io]
[![rustfmt-nightly~github][c~rustfmt_nightly~github~badge]][c~rustfmt_nightly~github]
[![rustfmt-nightly~lib.rs][c~rustfmt_nightly~lib.rs~badge]][c~rustfmt_nightly~lib.rs]
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`rustfmt`][c~rustfmt~docs]↗{{hi:rustfmt}}

```sh
# Install `rustfmt` if needed
rustup component add rustfmt

cargo fmt

# Fails if code is not formatted; use in CD / CI
cargo fmt -- --check
```

## Lint Your Code {#cargo-clippy}

[![clippy][c~clippy~docs~badge]][c~clippy~docs]{{hi:clippy}}
[![clippy~crates.io][c~clippy~crates.io~badge]][c~clippy~crates.io]
[![clippy~github][c~clippy~github~badge]][c~clippy~github]
[![clippy~lib.rs][c~clippy~lib.rs~badge]][c~clippy~lib.rs]
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo-clippy`][c~clippy~book]↗{{hi:cargo-clippy}} is the official Rust linter. It catches common mistakes and improves your Rust code.

```sh
rustup component add clippy # install if needed
cargo clippy
```

Mute a warning using the `#[allow(clippy::lint_name)]` attributes.

## Fix Compiler Warnings Automatically {#rustfix}

[![rustfix][c~rustfix~docs~badge]][c~rustfix~docs]{{hi:rustfix}}
[![rustfix~crates.io][c~rustfix~crates.io~badge]][c~rustfix~crates.io]
[![rustfix~github][c~rustfix~github~badge]][c~rustfix~github]
[![rustfix~lib.rs][c~rustfix~lib.rs~badge]][c~rustfix~lib.rs]
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

Can automatically fix compiler warnings that have a clear way to correct the problem that's likely what you want.

```sh
cargo fix
```

## Format or Lint Your Code Before Committing it {#cargo-husky}

[![cargo-husky][c~cargo_husky~docs~badge]][c~cargo_husky~docs]{{hi:cargo-husky}}
[![cargo-husky~crates.io][c~cargo_husky~crates.io~badge]][c~cargo_husky~crates.io]
[![cargo-husky~github][c~cargo_husky~github~badge]][c~cargo_husky~github]
[![cargo-husky~lib.rs][c~cargo_husky~lib.rs~badge]][c~cargo_husky~lib.rs]
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

[cargo-husky][c~cargo_husky~github]{{hi:cargo-husky}}↗ setup Git hooks automatically for [`cargo`][c~cargo~docs]↗{{hi:cargo}} projects with 🐶

Git hook scripts are useful for identifying simple issues (failing tests, trailing white spaces, [formatting][p~formatting] of the code, of [JSON][p~json], and YAML files...) before committing code, prior to submission to code review.

Add the [`cargo-husky`][c~cargo_husky~docs]↗{{hi:cargo-husky}} crate to the `[dev-dependencies]` section of your project's [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}}.

```toml
[dev-dependencies]
cargo-husky = "1"
```

Then run tests in your project directory.

```sh
cargo test
```

See also [`pre-commit`][pre-commit.com~website]↗, which is a Python framework for managing and [maintaining][p~maintaining] multi-language pre-commit hooks.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[code_formatting_linting: expand](https://github.com/john-cd/rust_howto/issues/310)

- [overcommit: A fully configurable and extendable Git hook manager](https://github.com/sds/overcommit)

</div>
