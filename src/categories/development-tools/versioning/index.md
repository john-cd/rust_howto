# Versioning

{{#include index.incl.md}}

## Parse and increment a {{i:version string}}

[![semver][semver-badge]][semver]  [![cat-config][cat-config-badge]][cat-config]

Constructs a [`{{i:semver::Version}}`][c-semver::Version]⮳ from a string literal using [`{{i:Version::parse}}`][c-semver::Version::parse]⮳ then increments it by patch, minor, and major version number one by one.

Note that in accordance with the [`{{i:Semantic Versioning Specification}}`][semver-spec]⮳, incrementing the minor {{i:version}} number resets the patch version number to 0 and incrementing the major version number resets both the minor and patch version numbers to 0.

```rust,editable
{{#include ../../../../deps/tests/semver-increment.rs}}
```

## Parse a complex version string

[![semver][semver-badge]][semver]  [![cat-config][cat-config-badge]][cat-config]

Constructs a [`{{i:semver::Version}}`][c-semver::Version]⮳ from a complex version string using [`{{i:Version::parse}}`][c-semver::Version::parse]⮳ The string contains {{i:pre-release}} and {{i:build metadata}} as defined in the [`{{i:Semantic Versioning Specification}}`][semver-spec]⮳.

Note that, in accordance with the Specification, build metadata is parsed but not considered when comparing versions. In other words, two versions may be equal even if their build strings differ.

```rust,editable
{{#include ../../../../deps/tests/semver-complex.rs}}
```

## Check if given version is pre-release

[![semver][semver-badge]][semver]  [![cat-config][cat-config-badge]][cat-config]

Given two versions, [`{{i:is_prerelease}}`][c-semver::Version]⮳ asserts that one is pre-release and the other is not.

```rust,editable
{{#include ../../../../deps/tests/semver-prerelease.rs}}
```

## Find the latest version satisfying given range

[![semver][semver-badge]][semver]  [![cat-config][cat-config-badge]][cat-config]

Given a list of version &strs, finds the latest [`{{i:semver::Version}}`][c-semver::Version]⮳
[`{{i:semver::VersionReq}}`][c-semver::VersionReq]⮳ filters the list with [`{{i:semver::VersionReq::matches}}`][c-semver::VersionReq::matches]⮳ Also demonstrates [`{{i:semver}}`][semver]⮳ pre-release preferences.

```rust,editable
{{#include ../../../../deps/tests/semver-latest.rs}}
```

## Check external command version for compatibility

[![semver][semver-badge]][semver]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]  [![cat-os][cat-os-badge]][cat-os]

Runs `git --version` using [`{{i:Command}}`][c-std::process::Command]⮳ then parses the {{i:version number}} into a
[`{{i:semver::Version}}`][c-semver::Version]⮳ using [`{{i:Version::parse}}`][c-semver::Version::parse]⮳  [`{{i:semver::VersionReq::matches}}`][c-semver::VersionReq::matches]⮳ compares
[`{{i:semver::VersionReq}}`][c-semver::VersionReq] to the parsed version. The command output resembles "git version x.y.z".

```rust,editable,no_run
{{#include ../../../../deps/tests/semver-command.rs}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
