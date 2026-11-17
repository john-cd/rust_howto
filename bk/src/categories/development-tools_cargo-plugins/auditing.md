<!-- TODO Reviewed ??-??-2026. -->

# Audit

{{#include auditing.incl.md}}

Cargo auditing tools answer different questions. Run them as part of a review or CI workflow, read their findings, and keep policy files in version control. They complement code review, tests, and release checks; no single tool establishes that a dependency is safe or compliant.

## Audit `cargo.lock` Files for Crates Containing Security Vulnerabilities {#cargo-audit}

{{#badges cargo-audit development-tools="Development tools" development-tools::cargo-plugins="Cargo plugins"}}

`cargo audit` checks the dependency versions recorded in `Cargo.lock` against the RustSec Advisory Database. It reports known vulnerabilities and advisories; it does not inspect source code for every possible defect.

```sh
cargo install --locked cargo-audit
cargo audit
```

Run it from the package or workspace root. Review each advisory and select an update or other remediation that fits the project.

## Embed the Exact Crate Versions in Your Rust Executable for Auditability {#cargo-auditable}

{{#badges cargo-auditable development-tools::cargo-plugins="Cargo plugins" encoding=Encoding}}

[`cargo-auditable`][c~cargo-auditable~repo]↗{{hi:cargo-auditable}} embeds dependency metadata in a compiled binary so compatible scanners can identify its Rust dependencies later. It does not scan the binary for vulnerabilities by itself.

Install the wrapper and use it in place of `cargo build` for release artifacts:

```sh
cargo install --locked cargo-auditable
cargo auditable build --release
```

## List the license(s) of Dependencies {#cargo-license}

{{#badges cargo-license development-tools::cargo-plugins="Cargo plugins"}}

[`cargo-license`][c~cargo-license~docs]↗{{hi:cargo-license}} is a [`cargo`][c~cargo~docs]↗{{hi:cargo}} subcommand to see license of dependencies.

Install [`cargo-license`][c~cargo-license~docs]↗{{hi:cargo-license}} with `cargo install --locked cargo-license` and run `cargo license` in the project directory to list dependency licenses. Review the result against the project's license policy.

## `cargo deny` {#cargo-deny}

{{#badges cargo-deny development-tools::cargo-plugins="Cargo plugins"}}

[`cargo-deny`][c~cargo-deny~docs]↗{{hi:cargo-deny}} is a [`cargo`][c~cargo~docs]↗{{hi:cargo}} plugin that lints your project's dependency graph to ensure all your dependencies conform to your expectations and requirements.

- Checks the license information for each crate.
- Checks for / bans specific [crates][p~crates] in your graph, as well as duplicates.
- Checks advisory sources for [crates][p~crates] with security vulnerabilities, or that have been marked as unmaintained or yanked from their source registry.
- Checks the source location for each crate.

Install and initialize a policy file with:

```sh
cargo install --locked cargo-deny
cargo deny init
cargo deny check
```

The generated `deny.toml` can configure checks for advisories, licenses, duplicate or disallowed crates, and dependency sources. A small starting point for license allowlisting is:

```toml
[licenses]
allow = ["Apache-2.0", "MIT"]
```

Choose the allowed licenses for your project and review any exceptions rather than copying this example as a universal policy.

## Review Dependency Supply Chains with `cargo vet` and `cargo-crev` {#cargo-vet}

[`cargo vet`][cargo-vet~repository]↗ records and verifies supply-chain audits for dependencies, including audits performed by organizations the project trusts. It helps teams review dependency changes and share audit evidence; it does not replace checking that the recorded audits match the versions and criteria the project needs.

```sh
cargo install --locked cargo-vet
cargo vet init
cargo vet
```

[`cargo-crev`][cargo-crev~repository]↗ is a separate, community-based system for publishing and evaluating code reviews and trust relationships. Its project describes itself as work in progress. Check its current maintenance and workflow before depending on it for a release policy.

```sh
cargo install --locked cargo-crev
cargo crev id new
cargo crev crate verify
```

## Find Unsafe Code and Unused Dependencies {#dependency-analysis}

- [`cargo-geiger`][cargo-geiger~repository]↗ counts unsafe code in a crate and its dependency tree. The count is a review signal, not a safety rating: unsafe code can be sound or unsound, and a low count does not prove safety.
- [`cargo-machete`][cargo-machete~repository]↗ finds dependencies declared in manifests that appear unused in source. Review findings before removing dependencies, especially when a crate is used through generated code, build scripts, or feature-specific paths.
- [`cargo-udeps`][cargo-udeps~repository]↗ detects unused dependencies with compiler analysis and currently requires a nightly toolchain. Review its findings for the same generated-code and feature caveats.

```sh
cargo install --locked cargo-geiger
cargo geiger

cargo install --locked cargo-machete
cargo machete

cargo install --locked cargo-udeps
cargo +nightly udeps
```

These tools differ in maintenance, scope, and toolchain requirements. Check their upstream releases and issue activity before adopting one. `cargo-crev` identifies itself as work in progress, and `cargo-udeps` requires nightly to run.

## Check Project License and Build an SBOM

Use `cargo license` to review dependency license metadata. Install `cargo-cyclonedx` and run `cargo cyclonedx` to generate a CycloneDX software bill of materials (SBOM) for downstream inventory and policy tools. An SBOM describes components; it does not certify that they are safe.

Run the checks selected for your project in continuous integration, using the committed lockfile and the same policy files as local development. For example, CI can run `cargo audit --locked` and `cargo deny check`; add `cargo vet` or inventory tools when the project has a defined process for reviewing their output.

## Generate a CycloneDX Software Bill of Materials {#cargo-cyclonedx}

[![cargo-cyclonedx][c~cargo-cyclonedx~docs~badge]][c~cargo-cyclonedx~docs]{{hi:cargo-cyclonedx}}
[![cargo-cyclonedx~crates.io][c~cargo-cyclonedx~crates.io~badge]][c~cargo-cyclonedx~crates.io]
[![cargo-cyclonedx~repo][c~cargo-cyclonedx~repo~badge]][c~cargo-cyclonedx~repo]
[![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo-cyclonedx`][c~cargo-cyclonedx~repo]↗{{hi:cargo-cyclonedx}} generates a CycloneDX software bill of materials (SBOM) from a Cargo project and its dependencies.

Install and run the plugin from the project directory:

```sh
cargo install --locked cargo-cyclonedx
cargo cyclonedx
```

The command invokes Cargo to inspect the project, so use it only with projects you trust; Cargo may run project-controlled code during that process.

## Related Topics {#related-topics .skip}

- Dependency management and [`cargo tree`][book~cargo~cargo-tree]↗.
- [Crate licenses][p~crates] and registry [advisories][rustsec~advisories].
- [Continuous integration][p~testing] for repeatable policy checks.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
