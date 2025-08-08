# Dependency Management

{{#include dependency_management.incl.md}}

Rust's dependency management is handled by [`Cargo`][c~cargo~docs]↗{{hi:Cargo}}, its built-in package manager.

- [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html)↗{{hi:Cargo.toml}}, located at the root of a Rust project, is the manifest file that defines the project's metadata and dependencies.
The `[dependencies]` section within `Cargo.toml` is where you specify the external crates (libraries) your project relies on.
- [`Cargo.lock`](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)↗{{hi:Cargo.lock}} records the exact versions of all dependencies used in a project. It ensures that builds are reproducible, even if new versions of dependencies are released.
- [Cargo][p~cargo] automates the process of downloading from [`crates.io`](https://crates.io)↗{{hi:crates.io}}, building, and linking dependencies. It resolves dependency versions, ensuring compatibility and preventing conflicts.

See the [[cargo | Cargo]] and [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]] chapters.

## Edit `Cargo.toml` {#cargo-edit}

[![cargo-edit][c~cargo_edit~docs~badge]][c~cargo_edit~docs] [![cargo-edit~crates.io][c~cargo_edit~crates.io~badge]][c~cargo_edit~crates.io] [![cargo-edit~github][c~cargo_edit~github~badge]][c~cargo_edit~github] [![cargo-edit~lib.rs][c~cargo_edit~lib.rs~badge]][c~cargo_edit~lib.rs]{{hi:cargo-edit}}{{hi:Cargo}}{{hi:Cargo-subcommand}}{{hi:Cli}}{{hi:Crates}}{{hi:Dependencies}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo edit`](https://docs.rs/crate/cargo-edit/latest)↗{{hi:cargo edit}} provides commands for modifying a [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} file. It allows you to add, remove, and upgrade dependencies by modifying your `Cargo.toml` file from the command line.

Currently available subcommands:

- `cargo upgrade`.
- `cargo set-version`.

## Find Unused Dependencies {#find-unused-dependencies}

### `cargo udeps` {#skip1}

[![cargo-udeps][c~cargo_udeps~docs~badge]][c~cargo_udeps~docs]{{hi:cargo-udeps}}
[![cargo-udeps~crates.io][c~cargo_udeps~crates.io~badge]][c~cargo_udeps~crates.io]
[![cargo-udeps~github][c~cargo_udeps~github~badge]][c~cargo_udeps~github]
[![cargo-udeps~lib.rs][c~cargo_udeps~lib.rs~badge]][c~cargo_udeps~lib.rs]
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}
[![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`udeps`][c~cargo_udeps~crates.io]{{hi:udeps}}↗ find unused dependencies in [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} .

While compilation of this tool also works on Rust stable, it needs Rust nightly to actually run.

### `cargo machete` {#skip2}

[![cargo-machete][c~cargo_machete~docs~badge]][c~cargo_machete~docs]{{hi:cargo-machete}}
[![cargo-machete~crates.io][c~cargo_machete~crates.io~badge]][c~cargo_machete~crates.io]
[![cargo-machete~github][c~cargo_machete~github~badge]][c~cargo_machete~github]
[![cargo-machete~lib.rs][c~cargo_machete~lib.rs~badge]][c~cargo_machete~lib.rs]

[`cargo-machete`][blog~cargo-machete]↗ is a [`cargo`][c~cargo~docs]↗{{hi:cargo}} tool that detects unused dependencies in Rust projects, in a fast (yet imprecise) way.

Install and run with:

```sh
cargo install cargo-machete
cargo machete
```

## Detect Dependencies that are Out of Date {#cargo-outdated}

[![cargo-outdated][c~cargo_outdated~docs~badge]][c~cargo_outdated~docs]{{hi:cargo-outdated}}
[![cargo-outdated~crates.io][c~cargo_outdated~crates.io~badge]][c~cargo_outdated~crates.io]
[![cargo-outdated~github][c~cargo_outdated~github~badge]][c~cargo_outdated~github]
[![cargo-outdated~lib.rs][c~cargo_outdated~lib.rs~badge]][c~cargo_outdated~lib.rs]

[`cargo-outdated`][c~cargo_outdated~docs]↗{{hi:cargo-outdated}} is a [`cargo`][c~cargo~docs]↗{{hi:cargo}} subcommand for displaying when dependencies are out of date.

If you are using VS Code, also look into the [`Dependi`](https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi)↗{{hi:Dependi}} VS Code plugin.

## `deps.rs` {#deps-rs}

[`deps.rs`][deps.rs~website]↗ [(github)][deps.rs~github]↗

The [`deps.rs`][deps.rs~website]↗ website uses semantic versioning to detect outdated or insecure dependencies in your project's [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}}.

## Rust Digger {#rust-digger}

[`rust-digger.code-maven.com`][rust-digger~website]↗ collects data about [Rust Crates][crates.io~website]↗ to find the common practices of Open Source Rust developers and trying to improve the Rust ecosystem.

## Related Topics {#related-topics}

- [[cargo | Cargo]].
- [[development_tools | Development Tools]].
- [[versioning | Versioning]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[dependency_management: review](https://github.com/john-cd/rust_howto/issues/597)
</div>
