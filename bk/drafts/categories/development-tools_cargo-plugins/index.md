<!-- TODO Reviewed ??-??-2026 -->

# Cargo Plugins

[![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

Cargo has built-in commands for common workflows. External Cargo subcommands add optional capabilities; examples below are independent projects, not part of Cargo itself.

| Task         | Selected commands or plugins                                                                                                                                                    | Purpose                                                                                     | Guide                                                                                                              |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Build        | [`cargo-make`][c~cargo-make~docs]↗{{hi:cargo-make}}                                                                                                                             | Define repeatable, cross-platform project tasks.                                            | [Build and run](building.md)                                                                                       |
| Test         | [`cargo-fuzz`][c~cargo-fuzz~docs]↗{{hi:cargo-fuzz}}, [`cargo-tarpaulin`][c~cargo-tarpaulin~docs]↗{{hi:cargo-tarpaulin}}                                                         | Fuzz code and measure test coverage.                                                        | [Fuzzing](../development-tools_testing/fuzzing.md); [Code Coverage](../development-tools_testing/code_coverage.md) |
| Lint         | [`cargo-spellcheck`][c~cargo-spellcheck~docs]↗{{hi:cargo-spellcheck}}                                                                                                           | Check spelling in Rust documentation and comments.                                          | [Code Formatting and Linting](code_formatting_linting.md)                                                          |
| Audit        | [`cargo-audit`][c~cargo-audit~docs]↗{{hi:cargo-audit}}, [`cargo-deny`][c~cargo-deny~docs]↗{{hi:cargo-deny}}, [`cargo-cyclonedx`][c~cargo-cyclonedx~docs]↗{{hi:cargo-cyclonedx}} | Check security advisories and dependency policy, and generate a software bill of materials. | [Audit](auditing.md)                                                                                               |
| Dependencies | [`cargo-edit`][c~cargo-edit~docs]↗{{hi:cargo-edit}}, [`cargo-outdated`][c~cargo-outdated~docs]↗{{hi:cargo-outdated}}                                                            | Edit dependency declarations and report newer releases.                                     | [Dependency Management](dependency_management.md)                                                                  |
| Release      | [`cargo-deb`][c~cargo-deb~docs]↗{{hi:cargo-deb}}; `cargo publish` is built in.                                                                                                  | Package a Rust application as a Debian package or publish a crate.                          | [Build and run](building.md); [Cargo](../development-tools/cargo/cargo.md)                                         |

Use `cargo help` to see built-in commands and `cargo help <command>` for their documentation. To find an external subcommand, Cargo looks for an executable named `cargo-<name>` on `PATH` and passes it the remaining arguments.

Before adopting a plugin, check its recent maintenance, supported Rust versions and platforms, license, and required file or network access. CI should pin tool versions when repeatable builds matter.

## Write Code

{{#include code_writing.incl.md}}

### Create an External Cargo Subcommand

Cargo discovers external subcommands as executables named `cargo-<name>` on `PATH` and passes them the remaining arguments. Keep the command independent of Cargo's private Rust APIs; use public command-line and manifest interfaces instead.

For argument parsing, consider [`clap`][c~clap~docs]↗{{hi:clap}} or [`argh`][c~argh~docs]↗{{hi:argh}}. [`serde`][c~serde~docs]↗{{hi:serde}} can read configuration, [`std::process`][c~std::process~docs]↗{{hi:std::process}} can launch commands, and [`tracing`][c~tracing~docs]↗{{hi:tracing}} or [`log`][c~log~docs]↗{{hi:log}} can provide diagnostics.

## Code Formatting and Linting

{{#include code_formatting_linting.incl.md}}

## Dependency Management

{{#include dependency_management.incl.md}}

## Build and run

{{#include building.incl.md}}

## Watching for Changes

{{#include watching_for_changes.incl.md}}

## Cross-compiling

{{#include cross_compiling.incl.md}}

## Audit

{{#include auditing.incl.md}}

## Performance

{{#include performance.incl.md}}

## Maintain

{{#include maintaining.incl.md}}

For adjacent topics, see [[development-tools_build-utils | Development Tools: Build Utils]], [[development-tools_debugging | Development Tools: Debugging]], [[development-tools_ffi | Development Tools: FFI]], [[development-tools_procedural-macro-helpers | Development Tools: Procedural Macro Helpers]], and [[development-tools_profiling | Development Tools: Profiling]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
