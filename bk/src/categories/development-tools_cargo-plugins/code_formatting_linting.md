# Code formatting and linting

{{#include code_formatting_linting.incl.md}}

## Format your code {#rustfmt}

[![rustfmt-nightly][c-rustfmt_nightly-badge]][c-rustfmt_nightly]{{hi:rustfmt-nightly}}
[![rustfmt-nightly-crates.io][c-rustfmt_nightly-crates.io-badge]][c-rustfmt_nightly-crates.io]
[![rustfmt-nightly-github][c-rustfmt_nightly-github-badge]][c-rustfmt_nightly-github]
[![rustfmt-nightly-lib.rs][c-rustfmt_nightly-lib.rs-badge]][c-rustfmt_nightly-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

`rustfmt`

```sh
# Install `rustfmt` if needed
rustup component add rustfmt

cargo fmt

# Fails if code is not formatted; use in CD / CI
cargo fmt -- --check
```

## Lint your code {#cargo-clippy}

[![clippy][c-clippy-badge]][c-clippy]{{hi:clippy}}
[![clippy-crates.io][c-clippy-crates.io-badge]][c-clippy-crates.io]
[![clippy-github][c-clippy-github-badge]][c-clippy-github]
[![clippy-lib.rs][c-clippy-lib.rs-badge]][c-clippy-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

`cargo-clippy` is the official Rust linter. It catches common mistakes and improves your Rust code.

```sh
rustup component add clippy # install if needed
cargo clippy
```

Mute a warning using the `#[allow(clippy::lint_name)]` attributes.

## Fix compiler warnings automatically {#rustfix}

[![rustfix][c-rustfix-badge]][c-rustfix]{{hi:rustfix}}
[![rustfix-crates.io][c-rustfix-crates.io-badge]][c-rustfix-crates.io]
[![rustfix-github][c-rustfix-github-badge]][c-rustfix-github]
[![rustfix-lib.rs][c-rustfix-lib.rs-badge]][c-rustfix-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

Can automatically fix compiler warnings that have a clear way to correct the problem that's likely what you want.

```sh
cargo fix
```

## Format or lint your code before committing it {#cargo-husky}

[![cargo-husky][c-cargo_husky-badge]][c-cargo_husky]{{hi:cargo-husky}}
[![cargo-husky-crates.io][c-cargo_husky-crates.io-badge]][c-cargo_husky-crates.io]
[![cargo-husky-github][c-cargo_husky-github-badge]][c-cargo_husky-github]
[![cargo-husky-lib.rs][c-cargo_husky-lib.rs-badge]][c-cargo_husky-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

[cargo-husky][c-cargo_husky-github]{{hi:cargo-husky}}⮳ setup Git hooks automatically for cargo projects with 🐶

Git hook scripts are useful for identifying simple issues (failing tests, trailing white spaces, [formatting][p-formatting] of the code, of [JSON][p-json], and YAML files...) before committing code, prior to submission to code review.

Add the [`cargo-husky`][c-cargo_husky]⮳{{hi:cargo-husky}} crate to the `[dev-dependencies]` section of your project's [`Cargo.toml`][book-cargo-cargo-toml]⮳{{hi:Cargo.toml}}.

```toml
[dev-dependencies]
cargo-husky = "1"
```

Then run tests in your project directory.

```sh
cargo test
```

See also [`pre-commit`][pre-commit.com-website]⮳, which is a Python framework for managing and [maintaining][p-maintaining] multi-language pre-commit hooks.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[code_formatting_linting: expand (P1)](https://github.com/john-cd/rust_howto/issues/310)
</div>
