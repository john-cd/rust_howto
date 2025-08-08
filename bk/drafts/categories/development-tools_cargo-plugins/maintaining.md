# Maintain

{{#include maintaining.incl.md}}

| Topic | Rust Crates |
|---|---|
| [[code_formatting_linting | Code Formatting]] | [`cargo fmt`](https://doc.rust-lang.org/cargo/commands/cargo-fmt.html)↗{{hi:cargo fmt}} formats your code to a consistent style. |
| [[code_formatting_linting | Linting]] | `cargo clippy` catches common code errors and style issues. |
| [[dependency_management | Dependency Management]] | [`cargo tree`](https://doc.rust-lang.org/cargo/commands/cargo-tree.html)↗{{hi:cargo tree}} displays your dependency tree to help you understand your project's dependencies. `cargo outdated` checks for outdated dependencies. `cargo audit` checks for crates with known security vulnerabilities. |
| Documentation Generation | [`cargo doc`](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)↗{{hi:cargo doc}} generates [documentation][p~documentation] from your code. `cargo deadlinks` finds broken links in your documentation. |
| [[testing | Testing]] | [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)↗{{hi:cargo test}} runs your unit and integration tests. |
| [[benchmarking | Benchmarking]] | `cargo bench` runs your benchmarks. |
| [[code_coverage | Code Coverage]] | `cargo tarpaulin` runs code coverage analysis. |
| Refactoring | Refactoring is often IDE-driven, but some tools exist: [`cargo-expand`][c~cargo_expand~docs]↗{{hi:cargo-expand}} expands [macros][p~macros], for example. |
| [[versioning | Version Management]] | [`cargo-bump`][c~cargo_bump~docs]↗{{hi:cargo-bump}} helps to automate version updates in your Cargo.toml. |

## Lint Your Crate API Changes for Semver Violations {#cargo-semver-checks}

[![cargo-semver-checks][c~cargo_semver_checks~docs~badge]][c~cargo_semver_checks~docs]
[![cargo-semver-checks~crates.io][c~cargo_semver_checks~crates.io~badge]][c~cargo_semver_checks~crates.io]
[![cargo-semver-checks~github][c~cargo_semver_checks~github~badge]][c~cargo_semver_checks~github]
[![cargo-semver-checks~lib.rs][c~cargo_semver_checks~lib.rs~badge]][c~cargo_semver_checks~lib.rs]
[![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}
[![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo-semver-checks`][c~cargo_semver_checks~docs]↗{{hi:cargo-semver-checks}} scans your Rust crate for [`semver`][c~semver~docs]↗{{hi:semver}} violations.

```sh
# If you Already Use `cargo-binstall` for Faster Tool installations:
$ cargo binstall cargo-semver-checks

# Otherwise:
$ cargo install cargo-semver-checks --locked

# Lint a new Release for SemVer Breakage Before `cargo publish`:
$ cargo semver-checks
```

## Manage the `cargo` Cache {#cargo-cache}

[![cargo-cache][c~cargo_cache~docs~badge]][c~cargo_cache~docs]{{hi:cargo-cache}}
[![cargo-cache~crates.io][c~cargo_cache~crates.io~badge]][c~cargo_cache~crates.io]
[![cargo-cache~github][c~cargo_cache~github~badge]][c~cargo_cache~github]
[![cargo-cache~lib.rs][c~cargo_cache~lib.rs~badge]][c~cargo_cache~lib.rs]
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}
[![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}
[![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo cache`][c~cargo_cache~github]{{hi:cargo-cache}}↗ manages the [`cargo`][c~cargo~docs]↗{{hi:cargo}} cache ($CARGO_HOME or ~/.cargo/), shows sizes and removes directories selectively.

## `cargo expand` {#cargo-expand}

[![cargo-expand][c~cargo_expand~docs~badge]][c~cargo_expand~docs] [![cargo-expand~crates.io][c~cargo_expand~crates.io~badge]][c~cargo_expand~crates.io] [![cargo-expand~github][c~cargo_expand~github~badge]][c~cargo_expand~github] [![cargo-expand~lib.rs][c~cargo_expand~lib.rs~badge]][c~cargo_expand~lib.rs]{{hi:cargo-expand}}{{hi:Cargo}}{{hi:Macros}}{{hi:Subcommand}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}} [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging]{{hi:Debugging}}{{hi:cargo-expand}}

`cargo expand` is a wrapper around `rustc -Zunpretty=expanded`. Shows the result of macro expansion and `#[derive]` expansion.

## `cargo hack` {#cargo-hack}

[![cargo-hack][c~cargo_hack~docs~badge]][c~cargo_hack~docs] [![cargo-hack~crates.io][c~cargo_hack~crates.io~badge]][c~cargo_hack~crates.io] [![cargo-hack~github][c~cargo_hack~github~badge]][c~cargo_hack~github] [![cargo-hack~lib.rs][c~cargo_hack~lib.rs~badge]][c~cargo_hack~lib.rs]{{hi:cargo-hack}}{{hi:Cargo}}{{hi:Subcommand}}{{hi:Testing}} [![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}}

[`cargo hack`][c~cargo_hack~docs]↗{{hi:cargo hack}} is a [`cargo`][c~cargo~docs]↗{{hi:cargo}} subcommand to provide various options useful for [testing][p~testing] and continuous integration.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[maintaining: fix; titles; decide what goes where](https://github.com/john-cd/rust_howto/issues/313)

- [SemVer parser](https://lib.rs/crates/semver)
- [SemVer Compatibility - The Cargo Book](https://doc.rust-lang.org/cargo/reference/semver.html)

</div>
