# Maintain

## Edit `Cargo.toml` {#cargo-edit}

[![cargo_edit-github][c-cargo_edit-github-badge]][c-cargo_edit-github]{{hi:cargo-edit}}

Cargo commands for modifying a `Cargo.toml` file

This tool extends Cargo to allow you to add, remove, and upgrade dependencies by modifying your `Cargo.toml` file from the command line.

Currently available subcommands:

- cargo upgrade
- cargo set-version

## Find unused dependencies {#find-unused-dependencies}

### `cargo udeps` {#cargo-udeps}

[![cargo-udeps][c-cargo_udeps-badge]][c-cargo_udeps]{{hi:cargo-udeps}}
[![cargo-udeps-crates.io][c-cargo_udeps-crates.io-badge]][c-cargo_udeps-crates.io]
[![cargo-udeps-github][c-cargo_udeps-github-badge]][c-cargo_udeps-github]
[![cargo-udeps-lib.rs][c-cargo_udeps-lib.rs-badge]][c-cargo_udeps-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}
[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

[udeps][c-cargo_udeps-crates.io]{{hi:udeps}}⮳ find unused dependencies in `Cargo.toml`.

While compilation of this tool also works on Rust stable, it needs Rust nightly to actually run.

### `cargo machete` {#cargo-machete}

[![cargo-machete][c-cargo_machete-badge]][c-cargo_machete]{{hi:cargo-machete}}
[![cargo-machete-crates.io][c-cargo_machete-crates.io-badge]][c-cargo_machete-crates.io]
[![cargo-machete-github][c-cargo_machete-github-badge]][c-cargo_machete-github]
[![cargo-machete-lib.rs][c-cargo_machete-lib.rs-badge]][c-cargo_machete-lib.rs]

`cargo-machete` is a Cargo tool that detects unused dependencies in Rust projects, in a fast (yet imprecise) way.

[Machete][blog-cargo-machete]⮳

Install and run with:

```sh
cargo install cargo-machete
cargo machete
```

## Detect dependencies that are out of date {#cargo-outdated}

[![cargo-outdated][c-cargo_outdated-badge]][c-cargo_outdated]{{hi:cargo-outdated}}
[![cargo-outdated-crates.io][c-cargo_outdated-crates.io-badge]][c-cargo_outdated-crates.io]
[![cargo-outdated-github][c-cargo_outdated-github-badge]][c-cargo_outdated-github]
[![cargo-outdated-lib.rs][c-cargo_outdated-lib.rs-badge]][c-cargo_outdated-lib.rs]

Cargo subcommand for displaying when dependencies are out of date.

If you are using VS Code, also look into the `Dependi` VS Code plugin.

## Lint your crate API changes for semver violations {#cargo-semver-checks}

[![cargo-semver-checks][c-cargo_semver_checks-badge]][c-cargo_semver_checks]
[![cargo-semver-checks-crates.io][c-cargo_semver_checks-crates.io-badge]][c-cargo_semver_checks-crates.io]
[![cargo-semver-checks-github][c-cargo_semver_checks-github-badge]][c-cargo_semver_checks-github]
[![cargo-semver-checks-lib.rs][c-cargo_semver_checks-lib.rs-badge]][c-cargo_semver_checks-lib.rs]
[![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]{{hi:Command line utilities}}
[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

`cargo-semver-checks` scans your Rust crate for semver violations.

```sh
# If you already use `cargo-binstall` for faster tool installations:
$ cargo binstall cargo-semver-checks

# Otherwise:
$ cargo install cargo-semver-checks --locked

# Lint a new release for SemVer breakage before `cargo publish`:
$ cargo semver-checks
```

## Manage the `cargo` cache {#cargo-cache}

[![cargo-cache][c-cargo_cache-badge]][c-cargo_cache]{{hi:cargo-cache}}
[![cargo-cache-crates.io][c-cargo_cache-crates.io-badge]][c-cargo_cache-crates.io]
[![cargo-cache-github][c-cargo_cache-github-badge]][c-cargo_cache-github]
[![cargo-cache-lib.rs][c-cargo_cache-lib.rs-badge]][c-cargo_cache-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}
[![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]{{hi:Command line utilities}}
[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo cache`][c-cargo_cache-github]{{hi:cargo-cache}}⮳ manages the `cargo` cache ($CARGO_HOME or ~/.cargo/), shows sizes and removes directories selectively.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO
[![cargo_expand-github][c-cargo_expand-github-badge]][c-cargo_expand-github]{{hi:cargo-expand}}
[![cargo_hack-github][c-cargo_hack-github-badge]][c-cargo_hack-github]{{hi:cargo-hack}}
</div>
