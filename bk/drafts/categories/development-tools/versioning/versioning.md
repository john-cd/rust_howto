<!-- TODO Reviewed ??-??-2026 -->

# Versioning

{{#include versioning.incl.md}}

This chapter follows the [Development Tools](../index.md) overview. It focuses on parsing and comparing semantic versions and checking a tool's reported version; Cargo's dependency requirement syntax is covered in the [Cargo chapter][p~cargo].

| Topic                                            | Rust Crates                                                                                                                                                                                                                                                |
| ------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Version Parsing and Comparison                   | [`semver`][c~semver~docs]↗{{hi:semver}} parses, compares, and manipulates SemVer (Semantic Versioning) strings.                                                                                                                                            |
| Version Bumping                                  | [`cargo-bump`][c~cargo-bump~docs]↗{{hi:cargo-bump}} is a tool to automate bumping versions in your [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} file according to SemVer rules. See [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]]. |
| Dependency Management (with version constraints) | [cargo][p~cargo] uses SemVer for specifying dependencies in `Cargo.toml`. See the [[cargo                                                                                                                                        | Cargo]] chapter for more details.                                                                                     |
| Release Management                               | Often involves tagging [`git`][git~website]↗{{hi:git}} releases.                                                                                                                                                                                           |
| Changelog Generation                             | Often handled with tools outside of the Rust ecosystem, but some crates might assist with [parsing][p~parsing] commit messages, etc.                                                                                                                       |

## Parse a Version String {#parse-a-version-string}

[![semver][c~semver~docs~badge]][c~semver~docs]{{hi:semver}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}} version string{{hi:Version string}}

Constructs a [`semver::Version`][c~semver::Version~docs]↗{{hi:semver::Version}} from a string literal using [`semver::Version::parse`][c~semver::Version::parse~docs]↗{{hi:semver::Version::parse}}.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/semver/semver_parse.rs:example}}
```

## Parse a Complex Version String {#parse-a-complex-version-string}

[![semver][c~semver~docs~badge]][c~semver~docs]{{hi:semver}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}}

Constructs a [`semver::Version`][c~semver::Version~docs]↗{{hi:semver::Version}} from a complex version string using [`semver::Version::parse`][c~semver::Version::parse~docs]↗{{hi:semver::Version::parse}}. The string contains pre-release{{hi:Pre-release}} and build metadata{{hi:Build metadata}} as defined by the [Semantic Versioning specification][c~semver~spec]↗{{hi:Semantic Versioning specification}}.

Note that, in accordance with the SemVer Specification, build metadata is parsed but not considered when comparing versions. In other words, two versions may be equal even if their build strings differ.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/semver/semver_complex.rs:example}}
```

## Check if a Given Version is Pre-release {#check-if-pre-release}

[![semver][c~semver~docs~badge]][c~semver~docs]{{hi:semver}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}}

Given two versions, [`semver::Version`][c~semver::Version~docs]↗{{hi:semver::Version}} asserts that one is pre-release and the other is not.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/semver/semver_prerelease.rs:example}}
```

## Find the Latest Version Satisfying a Given Range {#find-latest-version-within-range}

[![semver][c~semver~docs~badge]][c~semver~docs]{{hi:semver}} [![cat~config][cat~config~badge]][cat~config]{{hi:Configuration}}

Given a list of version &strs, finds the latest [`semver::Version`][c~semver::Version~docs]↗{{hi:semver::Version}}.
[`semver::VersionReq`][c~semver::VersionReq~docs]↗{{hi:semver::VersionReq}} filters the list with [`semver::VersionReq::matches`][c~semver::VersionReq::matches~docs]↗{{hi:semver::VersionReq::matches}} Also demonstrates [`semver`][c~semver~docs]↗{{hi:semver}} pre-release preferences.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/semver/semver_latest.rs:example}}
```

## Check External Command Version for Compatibility {#check-external-command-version-for-compat}

[![semver][c~semver~docs~badge]][c~semver~docs]{{hi:semver}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}} [![cat~os][cat~os~badge]][cat~os]{{hi:OS}}

Runs [`git --version`][git~website]↗{{hi:git}} using [`std::process::Command`][c~std::process::Command~docs]↗{{hi:std::process::Command}}, then parses the final whitespace-separated token into a [`semver::Version`][c~semver::Version~docs]↗{{hi:semver::Version}}. [`semver::VersionReq::matches`][c~semver::VersionReq::matches~docs]↗{{hi:semver::VersionReq::matches}} compares it with the required [`semver::VersionReq`][c~semver::VersionReq~docs]↗{{hi:semver::VersionReq}}. If the command is missing, the example prints an installation hint and exits successfully; a failed command or an invalid version remains an error.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/semver/semver_command.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Cargo][p~cargo] dependency requirements.
- [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]], including `cargo-semver-checks` and release automation.
- [Cross-compilation with Cargo](../cross-compilation/cross_compilation.md) for target-specific version and build constraints.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
