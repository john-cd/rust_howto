<!-- TODO Reviewed ??-??-2026 -->

# Release Automation

{{#include release_automation.incl.md}}

Release automation coordinates versioning, changelog updates, validation, packaging, and publication. Decide which artifacts you ship first: libraries usually go to [`crates.io`][crates.io~website]↗{{hi:crates.io}}, command-line applications may use GitHub Releases or installers, and services may also publish container images.

## Cargo Release Commands {#cargo}

Cargo provides the core commands to build an optimized artifact, inspect a crate package, validate a publish without uploading, and publish a package. This guide puts those commands into a release sequence; see the [Cargo chapter][p~cargo] for command details.

## Set the Version {#version}

Choose a version that follows your public API compatibility and release policy. For a direct manifest edit, [`cargo set-version`][cargo-edit~repo]↗{{hi:cargo set-version}} from `cargo-edit` can set an explicit version or bump a package:

```bash
cargo install cargo-edit
cargo set-version --bump patch
```

For a workspace, select the intended package or use `--workspace` when all workspace packages should change. Review every edited `Cargo.toml`, especially when versions are inherited from `[workspace.package]`.

Other tools named `cargo-bump` or `cargo-bumpp` exist. Check the exact package's maintenance, workspace support, and Git side effects before adopting it; [`cargo-bumpp`][cargo-bumpp~docs]↗ can commit, tag, and push as part of its workflow. For manifest-only changes, `cargo set-version` keeps the operation explicit.

## Prepare the Changelog {#changelog}

[`git-cliff`][git-cliff~docs]↗{{hi:git-cliff}} generates a changelog from commit history and can be configured to group commits and format entries. Initialize its configuration once, then review the generated file before committing it:

```bash
cargo install git-cliff
git-cliff --init
git-cliff --output CHANGELOG.md
```

## Validate the Release {#validate}

Run the checks required by your project before tagging or publishing. A typical library release includes formatting, linting, tests, documentation, and optimized builds; choose workspace, feature, and target flags to match the project's support matrix. For example:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps
cargo build --workspace --release
```

For libraries, [`cargo-semver-checks`][cargo-semver-checks~repo]↗{{hi:cargo-semver-checks}} compares the new public API with a baseline and reports changes that may not match the declared version bump:

```bash
cargo install cargo-semver-checks --locked
cargo semver-checks
```

Before publishing, inspect the exact package contents and ask Cargo to build the package without uploading it:

```bash
cargo package --list
cargo publish --dry-run
```

## Publish to `crates.io` {#publish-to-crates-io}

For a manual publish, create a scoped registry token on [`crates.io`][crates.io~website]↗{{hi:crates.io}}, then use [`cargo login`][book~cargo~cargo-login]↗{{hi:cargo login}} to store it in Cargo's credentials file. Treat it as a secret. After the checks above pass, publish the reviewed version:

```bash
cargo login
cargo publish
```

Publishing a version to crates.io is generally permanent. For CI, prefer [trusted publishing][crates.io~trusted-publishing~docs]↗{{hi:trusted publishing}} where supported. A trusted publisher exchanges the workflow's GitHub identity for a short-lived registry token. New crates still need an initial manual publish before trusted publishing can release later versions.

## Automate Crate Releases {#automation}

### `cargo-release` {#cargo-release}

[`cargo-release`][c~cargo-release~docs]↗{{hi:cargo-release}} automates a command-driven release, including validation, version changes, tags, and publishing. Its default behavior is a dry run, so review the proposed changes before running the release for real.

### `release-plz` {#release-plz}

[`release-plz`][c~release-plz~docs]↗{{hi:release-plz}} is designed for CI: it can open a release pull request with version and changelog updates, then publish after that pull request is merged. The following minimal GitHub Actions workflow shows the two jobs. Configure each crate's trusted publisher on crates.io first; the release job needs `id-token: write` and does not need a long-lived `CARGO_REGISTRY_TOKEN` secret:

```yaml
name: Release
on:
  push:
    branches: [main]
jobs:
  release:
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: read
      id-token: write
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 0
          persist-credentials: false
      - uses: dtolnay/rust-toolchain@stable
      - uses: release-plz/action@v0
        with:
          command: release
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  release-pr:
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 0
          persist-credentials: false
      - uses: dtolnay/rust-toolchain@stable
      - uses: release-plz/action@v0
        with:
          command: release-pr
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

Review the workflow permissions and release policy for your repository. See the [`release-plz` GitHub Actions guide][release-plz~quickstart]↗ for current setup details.

## Package Binaries and System Installers {#cargo-dist}

[`cargo-dist`][c~cargo-dist~docs]↗{{hi:cargo-dist}} builds and packages command-line applications for distribution through GitHub Releases and supported installer formats. For Linux system packages, [`cargo-deb`][c~cargo-deb~docs]↗{{hi:cargo-deb}} and [`cargo-rpm`][c~cargo-rpm~docs]↗{{hi:cargo-rpm}} build Debian and RPM packages from Cargo projects. Review each tool's configuration and target requirements before adding its packages to a release workflow.

### Debian and RPM Packages {#cargo-deb}

Use [`cargo-deb`][c~cargo-deb~docs]↗{{hi:cargo-deb}} or [`cargo-rpm`][c~cargo-rpm~docs]↗{{hi:cargo-rpm}} when your users install software through a Debian or RPM package manager. These tools produce native system packages; they do not replace publishing Rust libraries to a Cargo registry.

## Related Topics {#related-topics .skip}

- [Cargo][p~cargo] for package metadata, workspaces, and publishing commands.
- [Versioning][p~versioning] for semantic-versioning guidance.
- [CI/CD](cd_ci.md) and [GitHub Actions](github_actions.md) for automated validation and deployment.
- [Testing][p~testing] and [Cargo auditing][p~maintaining] for release checks.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
