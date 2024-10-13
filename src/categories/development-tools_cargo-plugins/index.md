# Cargo Plugins

Subcommands that extend the capabilities of Cargo.

{{#include index.incl.md}}

[![cat-cargo-plugins][cat-cargo-plugins-badge]][cat-cargo-plugins]

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

[![cargo_crates-github][c-cargo_crates-github-badge]][c-cargo_crates-github]

[![cargo_edit-github][c-cargo_edit-github-badge]][c-cargo_edit-github]

[![cargo_expand-github][c-cargo_expand-github-badge]][c-cargo_expand-github]

[![cargo_hack-github][c-cargo_hack-github-badge]][c-cargo_hack-github]

[![cargo_machete-github][c-cargo_machete-github-badge]][c-cargo_machete-github]

[![cargo_make-github][c-cargo_make-github-badge]][c-cargo_make-github]

[![cargo_xtask-github][c-cargo_xtask-github-badge]][c-cargo_xtask-github]

## Watch for changes

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]  [![cat-cargo-plugins][cat-cargo-plugins-badge]][cat-cargo-plugins]

```sh
cargo install cargo-watch

# Runs `cargo check` after every code change
cargo watch -x check

# Run cargo check after code changes.
# If it succeeds, it launches cargo test.
# If tests pass, it launches the application with cargo run.
cargo watch -x check -x test -x run
```

## Formatting

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]  [![cat-cargo-plugins][cat-cargo-plugins-badge]][cat-cargo-plugins]

```sh
# Install `rustfmt` if needed
rustup component add rustfmt

cargo fmt

# Fails if code is not formatted; use in CD / CI
cargo fmt -- --check
```

## Linting

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]  [![cat-cargo-plugins][cat-cargo-plugins-badge]][cat-cargo-plugins]

```sh
rustup component add clippy # install if needed
cargo clippy
```

Mute a warning using the `#[allow(clippy::lint_name)]` attributes

## Fix Compiler Warnings

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]  [![cat-cargo-plugins][cat-cargo-plugins-badge]][cat-cargo-plugins]

Can automatically fix compiler warnings that have a clear way to correct the problem that’s likely what you want.

```sh
cargo fix
```

## Code coverage

[![cargo_tarpaulin-github][c-cargo_tarpaulin-github-badge]][c-cargo_tarpaulin-github]  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]  [![cat-cargo-plugins][cat-cargo-plugins-badge]][cat-cargo-plugins]

## Security audit

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]  [![cat-cargo-plugins][cat-cargo-plugins-badge]][cat-cargo-plugins]

```sh
cargo install cargo-audit
cargo audit
```

## Unused dependencies

[udeps][c-cargo_udeps-crates.io]⮳  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]  [![cat-cargo-plugins][cat-cargo-plugins-badge]][cat-cargo-plugins]

or (simpler) [Machete][blog-cargo-machete]⮳

```sh
cargo install cargo-machete
cargo machete
```

## Templates

[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]  [![cat-cargo-plugins][cat-cargo-plugins-badge]][cat-cargo-plugins]

[Cargo Generate][c-cargo_generate-crates.io]⮳ is a developer tool to help you get up and running quickly with a new Rust project by leveraging a pre-existing git repository as a template.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO:

## Cargo Make

Install with

```bash
cargo install --force cargo-make
cargo make --version
```

- [automating-your-rust-workflows-with-cargo-make-part-1-of-5-introduction-and-basics](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-1-of-5-introduction-and-basics-b19ced7e7057)

## Cargo plugins

GitHub Action for installing development tools (mainly from GitHub Releases). List of tools: [cargo plugins](https://github.com/taiki-e/install-action)

## Cargo auditable

Make production Rust binaries auditable [cargo-auditable](https://github.com/rust-secure-code/cargo-auditable)

## Cargo limit

Cargo with less noise: warnings are skipped until errors are fixed, Neovim integration, etc [crates.io/crates/cargo-limit](https://crates.io/crates/cargo-limit)

## Cargo cache

- [`cargo cache`](https://github.com/matthiaskrgr/cargo-cache)

## Cargo Husky

[cargo-husky](https://github.com/rhysd/cargo-husky) Setup Git hooks automatically for cargo projects with 🐶

See also: Git hook scripts are useful for identifying simple issues before submission to code review [pre-commit.com](https://pre-commit.com/index.html) A framework for managing and maintaining multi-language pre-commit hooks.

## Cargo xtask

[cargo-xtask](https://github.com/matklad/cargo-xtask) cargo-xtask is a way to add free-form automation to a Rust project, a-la `make`, `npm run` or bespoke bash scripts.

The two distinguishing features of xtask are:

- It doesn't require any other binaries besides `cargo` and `rustc`, it fully bootstraps from them
- Unlike bash, it can more easily be cross platform, as it doesn't use the shell.

### Devx

[devx](https://github.com/elastio/devx) Collection of utilities for writing your own dev scripts

Devx is a collection of utilities for writing your own dev scripts in Rust. The project is inspired by and intended for seamless usage with [`cargo-xtask` idioms (you are highly encouraged to study them first)](https://github.com/matklad/cargo-xtask).

### xshell: Making Rust a Better Bash

`xshell` provides a set of cross-platform utilities for writing cross-platform and ergonomic "bash" scripts. [github.com/matklad/xshell](https://github.com/matklad/xshell)

### Duct

[duct.rs](https://github.com/oconnor663/duct.rs) Duct is a library for running child processes. Duct makes it easy to build pipelines and redirect IO like a shell. At the same time, Duct helps you write correct, portable code: whitespace is never significant, errors from child processes get reported by default, and a variety of [gotchas, bugs, and platform inconsistencies](https://github.com/oconnor663/duct.py/blob/master/gotchas.md) are handled for you the Right Way™.

## Cargo hakari

[cargo-hakari](https://crates.io/crates/cargo-hakari) Manage workspace-hack packages to speed up builds in large workspaces.

## Cargo wizard

Cargo subcommand for configuring Cargo projects for best performance. [github.com/Kobzol/cargo-wizard](https://github.com/Kobzol/cargo-wizard)

</div>
