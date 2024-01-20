# Versioning

## Parse and increment a version string

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Constructs a [`semver::Version`] from a string literal using [`Version::parse`],
then increments it by patch, minor, and major version number one by one.

Note that in accordance with the [Semantic Versioning Specification],
incrementing the minor version number resets the patch version number to 0 and
incrementing the major version number resets both the minor and patch version
numbers to 0.

```rust,editable
{#include ../../../deps/examples/semver-increment.rs}
```

## Parse a complex version string

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Constructs a [`semver::Version`] from a complex version string using [`Version::parse`]. The string
contains pre-release and build metadata as defined in the [Semantic Versioning Specification].

Note that, in accordance with the Specification, build metadata is parsed but not considered when
comparing versions. In other words, two versions may be equal even if their build strings differ.

```rust,editable
{#include ../../../deps/examples/semver-complex.rs}
```

## Check if given version is pre-release

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Given two versions, [`is_prerelease`] asserts that one is pre-release and the other is not.

```rust,editable
{#include ../../../deps/examples/semver-prerelease.rs}
```

## Find the latest version satisfying given range

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Given a list of version &strs, finds the latest [`semver::Version`].
[`semver::VersionReq`] filters the list with [`VersionReq::matches`].
Also demonstrates `semver` pre-release preferences.

```rust,editable
{#include ../../../deps/examples/semver-latest.rs}
```

## Check external command version for compatibility

[![semver-badge]][semver] [![cat-text-processing-badge]][cat-text-processing] [![cat-os-badge]][cat-os]

Runs `git --version` using [`Command`], then parses the version number into a
[`semver::Version`] using [`Version::parse`]. [`VersionReq::matches`] compares
[`semver::VersionReq`] to the parsed version.  The command output resembles
"git version x.y.z".

```rust,editable,no_run
{#include ../../../deps/examples/semver-command.rs}
```

[`is_prerelease`]: https://docs.rs/semver/*/semver/struct.Version.html#method.is_prerelease
[`Version::parse`]: https://docs.rs/semver/*/semver/struct.Version.html#method.parse
[Semantic Versioning Specification]: http://semver.org/
[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html
[`semver::VersionReq`]: https://docs.rs/semver/*/semver/struct.VersionReq.html
[`VersionReq::matches`]: https://docs.rs/semver/*/semver/struct.VersionReq.html#method.matches
{{#include ../refs/link-refs.md}}
