# Audit

{{#include auditing.incl.md}}

## Audit `cargo.lock` Files for Crates Containing Security Vulnerabilities {#cargo-audit}

[![cargo-audit][c-cargo_audit-badge]][c-cargo_audit]{{hi:cargo-audit}}
[![cargo-audit-crates.io][c-cargo_audit-crates.io-badge]][c-cargo_audit-crates.io]
[![cargo-audit-github][c-cargo_audit-github-badge]][c-cargo_audit-github]
[![cargo-audit-lib.rs][c-cargo_audit-lib.rs-badge]][c-cargo_audit-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

`cargo audit` checks for crates with known security vulnerabilities.

```sh
cargo install cargo-audit
cargo audit
```

## Embed the Exact Crate Versions in Your Rust Executable for Auditability {#cargo-auditable}

[![cargo-auditable][c-cargo_auditable-badge]][c-cargo_auditable]{{hi:cargo-auditable}}
[![cargo-auditable-crates.io][c-cargo_auditable-crates.io-badge]][c-cargo_auditable-crates.io]
[![cargo-auditable-github][c-cargo_auditable-github-badge]][c-cargo_auditable-github]
[![cargo-auditable-lib.rs][c-cargo_auditable-lib.rs-badge]][c-cargo_auditable-lib.rs]
[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}
[![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

[cargo-auditable][c-cargo_auditable-github]{{hi:cargo-auditable}}⮳ makes production Rust binaries auditable.

It audits binaries for known bugs or security vulnerabilities in production, at scale, with zero bookkeeping.

This works by embedding data about the dependency tree in [JSON][p-json] format into a dedicated linker section of the compiled executable.

## List the license(s) of Dependencies {#cargo-license}

[![cargo-license][c-cargo_license-badge]][c-cargo_license]{{hi:cargo-license}}
[![cargo-license-crates.io][c-cargo_license-crates.io-badge]][c-cargo_license-crates.io]
[![cargo-license-github][c-cargo_license-github-badge]][c-cargo_license-github]
[![cargo-license-lib.rs][c-cargo_license-lib.rs-badge]][c-cargo_license-lib.rs]
[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo-license`][c-cargo_license]⮳{{hi:cargo-license}} is a [`cargo`][c-cargo]⮳{{hi:cargo}} subcommand to see license of dependencies.

You can install `cargo-license` with `cargo install cargo-license` and run it in your project directory with: `cargo license` or [`cargo-license`][c-cargo_license]⮳{{hi:cargo-license}}.

## `cargo deny` {#cargo-deny}

[![cargo-deny][c-cargo_deny-badge]][c-cargo_deny]{{hi:cargo-deny}}
[![cargo-deny-crates.io][c-cargo_deny-crates.io-badge]][c-cargo_deny-crates.io]
[![cargo-deny-github][c-cargo_deny-github-badge]][c-cargo_deny-github]
[![cargo-deny-lib.rs][c-cargo_deny-lib.rs-badge]][c-cargo_deny-lib.rs]
[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo-deny`][c-cargo_deny]⮳{{hi:cargo-deny}} is a [`cargo`][c-cargo]⮳{{hi:cargo}} plugin that lets you lint your project's dependency graph to ensure all your dependencies conform to your expectations and requirements.

- Checks the license information for each crate.
- Checks for / bans specific [crates][p-crates] in your graph, as well as duplicates.
- Checks advisory [databases][p-databases] for [crates][p-crates] with security vulnerabilities, or that have been marked as Unmaintained, or which have been yanked from their source registry.
- Checks the source location for each crate.

Install with:

```sh
cargo install --locked cargo-deny

# Or, if you're an Arch user
pacman -S cargo-deny
```

```sh
cargo deny init

cargo deny check
cargo deny check licenses
```

## Related Topics

- Dependency Analysis: Useful for understanding your supply chain and potential risks
  - `cargo tree` displays your dependency tree, showing all transitive dependencies.
  - `cargo outdated` checks for outdated dependencies, which might have security fixes available.
- Linting/Style: `cargo clippy` lints your code for stylistic issues and potential bugs.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/922)
</div>
