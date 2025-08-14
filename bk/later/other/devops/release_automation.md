# Release Automation

{{#include release_automation.incl.md}}

Release automation involves automating the process of building, [testing][p~testing], packaging, and distributing your Rust applications.

How you distribute your application depends on the target platform:

- Libraries: publish (open-source) Rust libraries to [`crates.io`][crates.io~website]↗{{hi:crates.io}}.
- Executables: publish binaries to a service where they `can` be downloaded from (e.g., `GitHub Releases`).
- [Containers][p~containers]: Use [`Docker`][docker~website]↗{{hi:Docker}} to create the container image, push it to a private or public registry (e.g. [`DockerHub`][dockerhub~website]↗{{hi:DockerHub}}, `GitHub Packages`); optionally pull the image to deploy it to an orchestrator (e.g. [`Kubernetes`][kubernetes~website]↗{{hi:Kubernetes}}) or a Cloud service like [`AWS`][c~aws~docs]↗{{hi:AWS}} or [`Azure`][azure~website]↗{{hi:Azure}}.
- Packages: Create (`.deb`, `.rpm`...) packages for package management systems (e.g., Debian `dpkg`/`apt`, RPM, Flatpak, HomeBrew...) using [cargo][p~cargo] plugins like [`cargo-deb`][c~cargo-deb~docs]↗{{hi:cargo-deb}}, [`cargo-rpm`][c~cargo-rpm~docs]↗{{hi:cargo-rpm}}...

## Cargo {#skip}

Cargo handles building the release binaries or packaging your Rust code in `.crate` format suitable for `crates.io`:

- [`cargo build --release`][book~cargo~cargo-build]↗{{hi:cargo build}} creates optimized release binaries.
- [`cargo package`][book~cargo~cargo-package]↗{{hi:cargo package}} create a distributable, compressed .crate file with the source code of the package in the current directory. The resulting file will be stored in the `target/package` directory.
- [`cargo publish`][book~cargo~cargo-publish]↗{{hi:cargo publish}} uploads a package to the registry (typically `crates.io`).

## Publish a Crate to `crates.io` {#skip}

The following are the typical steps to release a crate to [`crates.io`][crates.io~website]↗{{hi:crates.io}}:

- `cargo update`, if desired.
- Update the changelog.
- Make sure everything works as advertised:
  - `cargo fmt --check`.
  - [`cargo doc`][book~cargo~cargo-doc]↗{{hi:cargo doc}} to inspect the documentation.
  - [`cargo clean`][book~cargo~cargo-clean]↗{{hi:cargo clean}} to build from a clean slate.
  - `cargo clippy`.
  - [`cargo test`][book~cargo~cargo-test]↗{{hi:cargo test}} or `cargo nextest run` to run unit and integration tests.
  - [`cargo build --locked --release`][book~cargo~cargo-build]↗{{hi:cargo build}} to build the production binaries.
  - [`cargo run --release`][book~cargo~cargo-run]↗{{hi:cargo run}} to manually exercise the application, if relevant.
- Increase the application's version number in [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}}.
- Make sure all code is committed and pushed to its repository.
- Create and push a git tag.
- Publish the package in the [cargo][p~cargo] registry (for example, crates.io).
  - Go to [`crates.io`][crates.io~website]↗{{hi:crates.io}}, sign in, and create an API token in `Account Settings` > `API Tokens`, if not done already.
  - Use [`cargo login`][book~cargo~cargo-login]↗{{hi:cargo login}} to save the token in `$CARGO_HOME/credentials.toml`.
  - Review `cargo package --list`, the list of source code files included in the package.
  - `cargo package`, then review the packaging output in `target/package`.
  - `cargo publish --dry-run`.
  - When ready, `cargo publish`.

## Publish a New Crate Version with `cargo release` {#cargo-release}

[![cargo-release][c~cargo-release~docs~badge]][c~cargo-release~docs]{{hi:cargo-release}}
[![cargo-release~crates.io][c~cargo-release~crates.io~badge]][c~cargo-release~crates.io]
[![cargo-release~github][c~cargo-release~github~badge]][c~cargo-release~github]
[![cargo-release~lib.rs][c~cargo-release~lib.rs~badge]][c~cargo-release~lib.rs]
[![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo-release`][c~cargo-release~docs]↗{{hi:cargo-release}} automates the release process, including version bumping and publishing to [`crates.io`][crates.io~website]↗{{hi:crates.io}}. It extends `cargo publish` with common release practices like validation, version management, tagging, and pushing. The process is customizable but with defaults that should help you get up and running quickly.

## `release-plz` {#release-plz}

[![release-plz][c~release-plz~docs~badge]][c~release-plz~docs]{{hi:release-plz}}
[![release-plz~crates.io][c~release-plz~crates.io~badge]][c~release-plz~crates.io]
[![release-plz~github][c~release-plz~github~badge]][c~release-plz~github]
[![release-plz~lib.rs][c~release-plz~lib.rs~badge]][c~release-plz~lib.rs]
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}
[![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}

[`release-plz`][c~release-plz~docs]↗{{hi:release-plz}} lets you release Rust crates from CI with a "Release PR".

## `cargo-dist` {#cargo-dist}

[![cargo-dist~website][c~cargo-dist~website~badge]][c~cargo-dist~website] [![cargo-dist][c~cargo-dist~docs~badge]][c~cargo-dist~docs] [![cargo-dist~crates.io][c~cargo-dist~crates.io~badge]][c~cargo-dist~crates.io] [![cargo-dist~github][c~cargo-dist~github~badge]][c~cargo-dist~github] [![cargo-dist~lib.rs][c~cargo-dist~lib.rs~badge]][c~cargo-dist~lib.rs]{{hi:cargo-dist}}

[`cargo-dist`][c~cargo-dist~docs]↗{{hi:cargo-dist}} packages shippable applications for Rust.

## Create Debian and RPM Packages with `cargo-deb` and `cargo-rpm` {#cargo-deb}

## Related Topics {#related-topics}

- [Testing][p~testing] Automation:
  - Code [[code_formatting_linting | linting]] .
  - Unit, integration, and end-to-end [[testing | tests]].
- Deployment:
  - [[cloud | Cloud]] platforms ([[aws | AWS]], Azure, GCP).
  - Servers.
  - [[containers | Container]] orchestration systems (Kubernetes).

See also [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[release_automation: write](https://github.com/john-cd/rust_howto/issues/604)
cover [`cargo-bump`][c~cargo-bump~github]↗{{hi:cargo-bump}} for simple version number increments.
Link:

- [book~cargo~cargo-login][book~cargo~cargo-login].
- [book~cargo~cargo-package][book~cargo~cargo-package].
- [book~cargo~cargo-publish][book~cargo~cargo-publish].

Review:

- [release-plz][release-plz~website].
- [comprehensive-guide-to-generating-deb-and-rpm-packages-for-rust-applications][blog~comprehensive-guide-to-generating-deb-and-rpm-packages-for-rust-applications].
- [List_of_software_package_management_systems][wikipedia~List_of_software_package_management_systems].
- [simplifying-debian-packaging-for-rust][blog~medium~simplifying-debian-packaging-for-rust].

</div>
