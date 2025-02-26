# Versioning

{{#include versioning.incl.md}}

## Parse and increment a version string {#parse-and-increment-a-version-string}

[![semver][c-semver-badge]][c-semver]{{hi:semver}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}} version string{{hi:Version string}}

Constructs a [`semver::Version`][c-semver::Version]{{hi:semver::Version}}⮳ from a string literal using [`semver::Version::parse`][c-semver::Version::parse]{{hi:semver::Version::parse}}⮳ then increments it by patch, minor, and major version number one by one.

Note that in accordance with the [semantic versioning specification`][c-semver-spec]{{hi:Semantic Versioning Specification}}⮳, incrementing the minor version number{{hi:Version number}} resets the patch version number to 0 and incrementing the major version number resets both the minor and patch version numbers to 0.

```rust,editable
{{#include ../../../../crates/cats/development_tools/tests/semver/semver_increment.rs:example}}
```

## Parse a complex version string {#parse-a-complex-version-string}

[![semver][c-semver-badge]][c-semver]{{hi:semver}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

Constructs a [`semver::Version`][c-semver::Version]{{hi:semver::Version}}⮳ from a complex version string using [`semver::Version::parse`][c-semver::Version::parse]{{hi:semver::Version::parse}}⮳ The string contains pre-release{{hi:Pre-release}} and build metadata{{hi:Build metadata}} as defined in the [semantic versioning specification`][c-semver-spec]{{hi:Semantic versioning specification}}⮳.

Note that, in accordance with the Specification, build metadata is parsed but not considered when comparing versions. In other words, two versions may be equal even if their build strings differ.

```rust,editable
{{#include ../../../../crates/cats/development_tools/tests/semver/semver_complex.rs:example}}
```

## Check if a given version is pre-release {#check-if-pre-release}

[![semver][c-semver-badge]][c-semver]{{hi:semver}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

Given two versions, [`semver::Version`][c-semver::Version]{{hi:semver::Version}}⮳ asserts that one is pre-release and the other is not.

```rust,editable
{{#include ../../../../crates/cats/development_tools/tests/semver/semver_prerelease.rs:example}}
```

## Find the latest version satisfying a given range {#find-latest-version-within-range}

[![semver][c-semver-badge]][c-semver]{{hi:semver}} [![cat-config][cat-config-badge]][cat-config]{{hi:Configuration}}

Given a list of version &strs, finds the latest [`semver::Version`][c-semver::Version]{{hi:semver::Version}}⮳
[`semver::VersionReq`][c-semver::VersionReq]{{hi:semver::VersionReq}}⮳ filters the list with [`semver::VersionReq::matches`][c-semver::VersionReq::matches]{{hi:semver::VersionReq::matches}}⮳ Also demonstrates [`semver`][c-semver]{{hi:semver}}⮳ pre-release preferences.

```rust,editable
{{#include ../../../../crates/cats/development_tools/tests/semver/semver_latest.rs:example}}
```

## Check external command version for compatibility {#check-external-command-version-for-compat}

[![semver][c-semver-badge]][c-semver]{{hi:semver}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}} [![cat-os][cat-os-badge]][cat-os]{{hi:OS}}

Runs `git --version` using [`std::process::Command`][c-std::process::Command]{{hi:std::process::Command}}⮳ then parses the version number{{hi:Version number}} into a
[`semver::Version`][c-semver::Version]{{hi:semver::Version}}⮳ using [`semver::Version::parse`][c-semver::Version::parse]{{hi:semver::Version::parse}}⮳ [`semver::VersionReq::matches`][c-semver::VersionReq::matches]{{hi:semver::VersionReq::matches}}⮳ compares
[`semver::VersionReq`][c-semver::VersionReq]{{hi:semver::VersionReq}} to the parsed version. The command output resembles "git version x.y.z".

```rust,editable
{{#include ../../../../crates/cats/development_tools/tests/semver/semver_command.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[ P1 review](https://github.com/john-cd/rust_howto/issues/920)

Semantic Versioning (SemVer): (The concept itself. No specific crate implements SemVer, but crates like `semver` help work with SemVer strings.)
Version Parsing and Comparison: `semver` (crate for parsing, comparing, and manipulating SemVer strings)
Version Bumping: cargo-bump (a tool to automate bumping versions in your Cargo.toml file according to SemVer rules)
Dependency Management (with version constraints): [cargo][p-cargo] (uses SemVer for specifying dependencies in Cargo.toml)
Release Management: (Often involves tagging Git releases, which is not Rust-specific)
Changelog Generation: (Often handled with tools outside of the Rust ecosystem, but some crates might assist with [parsing][p-parsing] commit messages, etc.)
</div>
