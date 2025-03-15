# Maintain

{{#include maintaining.incl.md}}

## Edit `Cargo.toml` {#cargo-edit}

[![cargo-edit][c-cargo_edit-badge]][c-cargo_edit] [![cargo-edit-crates.io][c-cargo_edit-crates.io-badge]][c-cargo_edit-crates.io] [![cargo-edit-github][c-cargo_edit-github-badge]][c-cargo_edit-github] [![cargo-edit-lib.rs][c-cargo_edit-lib.rs-badge]][c-cargo_edit-lib.rs]{{hi:cargo-edit}}{{hi:Cargo}}{{hi:Cargo-subcommand}}{{hi:Cli}}{{hi:Crates}}{{hi:Dependencies}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

`cargo edit` provides commands for modifying a [`Cargo.toml`][book-cargo-cargo-toml]⮳{{hi:Cargo.toml}} file. It allows you to add, remove, and upgrade dependencies by modifying your [`Cargo.toml`][book-cargo-cargo-toml]⮳{{hi:Cargo.toml}} file from the command line.

Currently available subcommands:

- `cargo upgrade`.
- `cargo set-version`.

## Find Unused Dependencies {#find-unused-dependencies}

### `cargo udeps` {#skip1}

[![cargo-udeps][c-cargo_udeps-badge]][c-cargo_udeps]{{hi:cargo-udeps}}
[![cargo-udeps-crates.io][c-cargo_udeps-crates.io-badge]][c-cargo_udeps-crates.io]
[![cargo-udeps-github][c-cargo_udeps-github-badge]][c-cargo_udeps-github]
[![cargo-udeps-lib.rs][c-cargo_udeps-lib.rs-badge]][c-cargo_udeps-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}
[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`udeps`][c-cargo_udeps-crates.io]{{hi:udeps}}⮳ find unused dependencies in [`Cargo.toml`][book-cargo-cargo-toml]⮳{{hi:Cargo.toml}} .

While compilation of this tool also works on Rust stable, it needs Rust nightly to actually run.

### `cargo machete` {#skip2}

[![cargo-machete][c-cargo_machete-badge]][c-cargo_machete]{{hi:cargo-machete}}
[![cargo-machete-crates.io][c-cargo_machete-crates.io-badge]][c-cargo_machete-crates.io]
[![cargo-machete-github][c-cargo_machete-github-badge]][c-cargo_machete-github]
[![cargo-machete-lib.rs][c-cargo_machete-lib.rs-badge]][c-cargo_machete-lib.rs]

[`cargo-machete`][blog-cargo-machete]⮳ is a [`cargo`][c-cargo]⮳{{hi:cargo}} tool that detects unused dependencies in Rust projects, in a fast (yet imprecise) way.

Install and run with:

```sh
cargo install cargo-machete
cargo machete
```

## Detect Dependencies that are out of Date {#cargo-outdated}

[![cargo-outdated][c-cargo_outdated-badge]][c-cargo_outdated]{{hi:cargo-outdated}}
[![cargo-outdated-crates.io][c-cargo_outdated-crates.io-badge]][c-cargo_outdated-crates.io]
[![cargo-outdated-github][c-cargo_outdated-github-badge]][c-cargo_outdated-github]
[![cargo-outdated-lib.rs][c-cargo_outdated-lib.rs-badge]][c-cargo_outdated-lib.rs]

[`cargo-outdated`][c-cargo_outdated]⮳{{hi:cargo-outdated}} is a [`cargo`][c-cargo]⮳{{hi:cargo}} subcommand for displaying when dependencies are out of date.

If you are using VS Code, also look into the `Dependi` VS Code plugin.

## Lint Your Crate API Changes for Semver Violations {#cargo-semver-checks}

[![cargo-semver-checks][c-cargo_semver_checks-badge]][c-cargo_semver_checks]
[![cargo-semver-checks-crates.io][c-cargo_semver_checks-crates.io-badge]][c-cargo_semver_checks-crates.io]
[![cargo-semver-checks-github][c-cargo_semver_checks-github-badge]][c-cargo_semver_checks-github]
[![cargo-semver-checks-lib.rs][c-cargo_semver_checks-lib.rs-badge]][c-cargo_semver_checks-lib.rs]
[![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]{{hi:Command line utilities}}
[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo-semver-checks`][c-cargo_semver_checks]⮳{{hi:cargo-semver-checks}} scans your Rust crate for [`semver`][c-semver]⮳{{hi:semver}} violations.

```sh
# If you Already Use `cargo-binstall` for Faster Tool installations:
$ cargo binstall cargo-semver-checks

# Otherwise:
$ cargo install cargo-semver-checks --locked

# Lint a new Release for SemVer Breakage Before `cargo publish`:
$ cargo semver-checks
```

## Manage the `cargo` Cache {#cargo-cache}

[![cargo-cache][c-cargo_cache-badge]][c-cargo_cache]{{hi:cargo-cache}}
[![cargo-cache-crates.io][c-cargo_cache-crates.io-badge]][c-cargo_cache-crates.io]
[![cargo-cache-github][c-cargo_cache-github-badge]][c-cargo_cache-github]
[![cargo-cache-lib.rs][c-cargo_cache-lib.rs-badge]][c-cargo_cache-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}
[![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]{{hi:Command line utilities}}
[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo cache`][c-cargo_cache-github]{{hi:cargo-cache}}⮳ manages the [`cargo`][c-cargo]⮳{{hi:cargo}} cache ($CARGO_HOME or ~/.cargo/), shows sizes and removes directories selectively.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[maintaining: fix](https://github.com/john-cd/rust_howto/issues/313)

## `cargo expand` {#cargo-expand}

[![cargo-expand][c-cargo_expand-badge]][c-cargo_expand] [![cargo-expand-crates.io][c-cargo_expand-crates.io-badge]][c-cargo_expand-crates.io] [![cargo-expand-github][c-cargo_expand-github-badge]][c-cargo_expand-github] [![cargo-expand-lib.rs][c-cargo_expand-lib.rs-badge]][c-cargo_expand-lib.rs]{{hi:cargo-expand}}{{hi:Cargo}}{{hi:Macros}}{{hi:Subcommand}} [![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}} [![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}{{hi:cargo-expand}}

`cargo expand` is a wrapper around `rustc -Zunpretty=expanded`. Shows the result of macro expansion and `#[derive]` expansion.

## `cargo hack` {#cargo-hack}

[![cargo-hack][c-cargo_hack-badge]][c-cargo_hack] [![cargo-hack-crates.io][c-cargo_hack-crates.io-badge]][c-cargo_hack-crates.io] [![cargo-hack-github][c-cargo_hack-github-badge]][c-cargo_hack-github] [![cargo-hack-lib.rs][c-cargo_hack-lib.rs-badge]][c-cargo_hack-lib.rs]{{hi:cargo-hack}}{{hi:Cargo}}{{hi:Subcommand}}{{hi:Testing}} [![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]{{hi:Command line utilities}} [![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}} [![cat-development-tools::testing][cat-development-tools::testing-badge]][cat-development-tools::testing]{{hi:Testing}}

`cargo hack` is a [`cargo`][c-cargo]⮳{{hi:cargo}} subcommand to provide various options useful for [testing][p-testing] and continuous integration.

Code Formatting:

cargo fmt: Formats your code to a consistent style.
Linting:

cargo clippy: Catches common code errors and style issues.
Dependency Management:

cargo tree: Displays your dependency tree to help you understand your project's dependencies.
cargo outdated: Checks for outdated dependencies.
cargo audit: Checks for crates with known security vulnerabilities.
Dead Code Detection:

cargo deadlinks: Finds broken links in your documentation.
Documentation Generation:

cargo doc: Generates [documentation][p-documentation] from your code.
Testing:

cargo test: Runs your unit and integration tests.
Benchmarking:

cargo bench: Runs your benchmarks.
Code Coverage:

cargo tarpaulin: Runs code coverage analysis.
Refactoring: (Often IDE-driven, but some tools exist)

cargo-expand: Expands [macros][p-macros], which can be helpful for understanding code and refactoring. (Not strictly refactoring itself, but a helpful tool.)
Version Management:

cargo-bump: Helps to automate version updates in your Cargo.toml.

</div>
