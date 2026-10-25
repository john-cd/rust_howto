<!-- TODO Reviewed ??-??-2026 -->

# Dependency Management

{{#include dependency_management.incl.md}}

Cargo resolves, builds, and records a Rust project's dependency graph. Use the manifest to express the versions your project accepts and the lockfile to record the exact versions selected for a build. Applications normally commit `Cargo.lock`; libraries often leave the lockfile out of their published source so downstream users can resolve the library with their own graph.

See [[cargo | Cargo]] for Cargo basics and [[development-tools_cargo-plugins | Cargo Plugins]] for other subcommands.

## Add or Remove Dependencies {#cargo-edit}

[![cargo-edit][c~cargo-edit~docs~badge]][c~cargo-edit~docs] [![cargo-edit~crates.io][c~cargo-edit~crates.io~badge]][c~cargo-edit~crates.io] [![cargo-edit~repo][c~cargo-edit~repo~badge]][c~cargo-edit~repo] [![cargo-edit~lib.rs][c~cargo-edit~lib.rs~badge]][c~cargo-edit~lib.rs]{{hi:cargo-edit}}{{hi:Cargo}}{{hi:Dependencies}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

Use Cargo's built-in `cargo add` and `cargo remove` to edit a manifest. `cargo add <crate>` can also select features or a renamed dependency; review the resulting `Cargo.toml` and lockfile before committing. The [`cargo-edit`][c~cargo-edit~docs]↗{{hi:cargo-edit}} project provides additional commands such as `cargo upgrade` for changing version requirements and `cargo set-version` for package versions.

## Inspect the Dependency Graph {#dependency-graph}

Use `cargo tree` to inspect direct and transitive dependencies. Add `-d` to find multiple versions of a package in the resolved graph. This is useful when investigating build size, duplicate versions, or why a crate is present.

`cargo tree` reports what the current manifest and lockfile resolve to; it does not determine whether your source uses every direct dependency.

## Update Locked Versions {#cargo-update}

`cargo update` refreshes `Cargo.lock` while keeping the version requirements in `Cargo.toml`. Use `cargo update -p <crate>` to focus an update on one package. Review the diff and run the project's checks because a newer compatible release can still change behavior.

[`cargo-outdated`][c~cargo-outdated~docs]↗{{hi:cargo-outdated}} reports dependencies for which newer versions are available. Treat its output as an update list to review: a newer version may require a manifest change, a Rust version change, or code updates. The `cargo upgrade` command from `cargo-edit` changes manifest requirements, while `cargo update` changes the resolved lockfile within existing requirements.

## Find Unused Dependencies {#unused-dependencies}

[![cargo-machete][c~cargo-machete~docs~badge]][c~cargo-machete~docs]{{hi:cargo-machete}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo-machete`][c~cargo-machete~docs]↗{{hi:cargo-machete}} quickly searches for dependencies that do not appear to be used in a package's source. It is a heuristic: build scripts, generated code, macros, and renamed crates can require configuration or manual review. Run `cargo machete` from the workspace and inspect each suggestion before removing a dependency; its `--fix` option edits manifests automatically.

[`cargo-udeps`][c~cargo-udeps~docs]↗{{hi:cargo-udeps}} uses compiler information to find unused dependencies and generally requires a nightly toolchain. Prefer it when source-text scanning is insufficient and the project can run the required toolchain.

## Check Advisories and Dependency Policy {#dependency-policy}

[`cargo-audit`][c~cargo-audit~docs]↗{{hi:cargo-audit}} checks the versions in `Cargo.lock` against the RustSec advisory database. [`cargo-deny`][c~cargo-deny~docs]↗{{hi:cargo-deny}} can also enforce advisory, license, source, and duplicate-dependency policies. Configure these checks for the project's policy and review exceptions narrowly; they answer different questions from an unused-dependency scan or an update report.

## Related Topics {#related-topics .skip}

- [[cargo | Cargo]].
- [[code_formatting_linting | Code Formatting and Linting]].
- [[maintaining | Maintaining Rust Projects]].
- [[versioning | Versioning]].
