# Dependency Management

{{#include dependency_management.incl.md}}

Rust's dependency management is handled by [`Cargo`][c-cargo]⮳{{hi:Cargo}}, its built-in package manager.

- `Cargo.toml`, located at the root of a Rust project, is the manifest file that defines the project's metadata and dependencies.
The [dependencies] section within `Cargo.toml` is where you specify the external crates (libraries) your project relies on.
- `Cargo.lock` records the exact versions of all dependencies used in a project. It ensures that builds are reproducible, even if new versions of dependencies are released.
- [Cargo][p-cargo] automates the process of downloading from `crates.io`, building, and linking dependencies. It resolves dependency versions, ensuring compatibility and preventing conflicts.

See the [[cargo | Cargo]] and [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]] chapters.

## `deps.rs` {#deps-rs}

[`deps.rs`][deps-rs-website]⮳ [(github)][deps-rs-github]⮳

The [`deps.rs`][deps-rs-website]⮳ website uses semantic versioning to detect outdated or insecure dependencies in your project's [`Cargo.toml`][book-cargo-cargo-toml]⮳{{hi:Cargo.toml}}.

## Rust Digger {#rust-digger}

[`rust-digger.code-maven.com`][rust-digger-website]⮳ collects data about [Rust Crates][crates.io-website]⮳ to find the common practices of Open Source Rust developers and trying to improve the Rust ecosystem.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[dependency_management: improve](https://github.com/john-cd/rust_howto/issues/597)

See [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]].

## `cargo-edit` {#cargo-edit}

Adds 'cargo upgrade' and '[cargo][p-cargo] set-version' commands to cargo

## `cargo-outdated` {#cargo-outdated}

Finds dependencies that have available updates

## `cargo-audit` {#cargo-audit}

Check dependencies for reported security vulnerabilities

## `cargo-license` {#cargo-license}

Lists licenses of all dependencies

## `cargo-deny` {#cargo-deny}

Enforce policies on your code and dependencies.
</div>
