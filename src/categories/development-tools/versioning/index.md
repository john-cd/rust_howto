# Versioning

{{#include index.incl.md}}

## Parse and increment a {{i:version string}}

[![semver][semver-badge]][semver]  [![cat-config][cat-config-badge]][cat-config]

Constructs a [`semver::Version`][semver::Version]⮳ from a string literal using [`Version::parse`][semver::Version::parse]⮳ then increments it by patch, minor, and major version number one by one.

Note that in accordance with the [`Semantic Versioning Specification`][semver-spec]⮳, incrementing the minor {{i:version}} number resets the patch version number to 0 and incrementing the major version number resets both the minor and patch version numbers to 0.

```rust,editable
{{#include ../../../../deps/tests/semver-increment.rs}}
```

## Parse a complex version string

[![semver][semver-badge]][semver]  [![cat-config][cat-config-badge]][cat-config]

Constructs a [`semver::Version`][semver::Version]⮳ from a complex version string using [`Version::parse`][semver::Version::parse]⮳ The string contains {{i:pre-release}} and {{i:build metadata}} as defined in the [`Semantic Versioning Specification`][semver-spec]⮳.

Note that, in accordance with the Specification, build metadata is parsed but not considered when comparing versions. In other words, two versions may be equal even if their build strings differ.

```rust,editable
{{#include ../../../../deps/tests/semver-complex.rs}}
```

## Check if given version is pre-release

[![semver][semver-badge]][semver]  [![cat-config][cat-config-badge]][cat-config]

Given two versions, [`is_prerelease`][semver::Version]⮳ asserts that one is pre-release and the other is not.

```rust,editable
{{#include ../../../../deps/tests/semver-prerelease.rs}}
```

## Find the latest version satisfying given range

[![semver][semver-badge]][semver]  [![cat-config][cat-config-badge]][cat-config]

Given a list of version &strs, finds the latest [`semver::Version`][semver::Version]⮳
[`semver::VersionReq`][semver::VersionReq]⮳ filters the list with [`semver::VersionReq::matches`][semver::VersionReq::matches]⮳ Also demonstrates [`semver`][semver]⮳ pre-release preferences.

```rust,editable
{{#include ../../../../deps/tests/semver-latest.rs}}
```

## Check external command version for compatibility

[![semver][semver-badge]][semver]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]  [![cat-os][cat-os-badge]][cat-os]

Runs `git --version` using [`Command`][std::process::Command]⮳ then parses the {{i:version number}} into a
[`semver::Version`][semver::Version]⮳ using [`Version::parse`][semver::Version::parse]⮳  [`semver::VersionReq::matches`][semver::VersionReq::matches]⮳ compares
[`semver::VersionReq`][semver::VersionReq] to the parsed version. The command output resembles "git version x.y.z".

```rust,editable,no_run
{{#include ../../../../deps/tests/semver-command.rs}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
