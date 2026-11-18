<!-- TODO Reviewed ??-??-2026. -->

# Development Tools

[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

Tools that provide developer-facing features such as testing, debugging, linting, performance profiling, autocompletion, formatting, and more.

| Topic | Rust Crates |
| --- | --- |
| Build System/Package Manager | [`cargo`][c~cargo~docs]↗{{hi:cargo}} |
| Formatter | [`rustfmt`][c~rustfmt~repo]↗{{hi:rustfmt}} |
| Linter | [`clippy`][c~clippy~docs]↗{{hi:clippy}} |
| Testing Framework | [`cargo test`][book~cargo~cargo-test]↗{{hi:cargo test}} (built-in), [`rstest`][c~rstest~docs]↗{{hi:rstest}} (for data-driven tests) |
| Debugging | [`gdb`][c~gdb~docs]↗{{hi:gdb}}, [`lldb`][c~lldb~docs]↗{{hi:lldb}} (often used via IDE integration), `cargo-debug` |
| Profiling | `cargo flamegraph`, [`perf`][c~perf~docs]↗{{hi:perf}} (system profiler) |
| Documentation Generator | [`cargo doc`][book~cargo~cargo-doc]↗{{hi:cargo doc}} |
| Versioning and Release Management | [Versioning](versioning/versioning.md) covers SemVer parsing, compatibility checks, and version-related release tooling. |
| Code Coverage | [`grcov`][c~grcov~docs]↗{{hi:grcov}}, [`tarpaulin`][c~cargo-tarpaulin~docs]↗{{hi:tarpaulin}} |
| Continuous Integration | often uses CI platforms directly; no specific Rust crates needed, but [`xtask`][c~xtask~docs]↗{{hi:xtask}} can help manage CI tasks |
| Code Analysis | `cargo clippy`, `rust-analyzer` (for Language Server Protocol support in IDEs) |

See also [Rust tools][rust-tools]↗{{hi:Rust tools}}.

## Cargo and Workspace

- [Cargo](cargo/cargo.md), [Package Layout](cargo/package_layout.md), and [Crate Registries](cargo/crate_registries.md).

## Build and Compilation {#rust-compilation}

- [Build Rust Code](compilation/building.md), [Reduce Compilation Duration](compilation/reduce_compilation_duration.md), and [Faster Linking](compilation/faster_linking.md).

## Documentation and Formatting

- [Documentation](documentation/documentation.md), [`mdBook`](documentation/mdbook.md), [`mdbook-utils`](documentation/mdbook_utils.md), [`templ`](documentation/templ.md), and [Badges](documentation/badges.md).
- [Code Formatting](formatting/formatting.md).

## Installation and Releases

- [Rustup](installation/rustup.md), [Rust and Binary Installation](installation/install.md), and [Versioning](versioning/versioning.md).

## Repository Automation and Portability

- [Repository Automation](automation/index.md) covers project-specific checks, task scripts, and Git history analysis.
- [Cross-compilation](cross-compilation/cross_compilation.md) covers target setup and linker selection; see also [Cargo cross-compilation tools](../development-tools_cargo-plugins/cross_compiling.md).
- [Transpilers](transcompilation/transpilers.md) covers translating source code into Rust.

## Verification and Utilities

- [Code Verification](other/code_verification.md), [`miri`](other/miri.md), [Analyze and Generate Rust Code](other/rust_source.md), and [Other Tools](other/other.md).
- [Development Tools - Testing](../development-tools_testing/index.md) covers tests, coverage, and fuzzing.

## Related Topics {#related-topics .skip}

- [Development Tools - Build Utils](../development-tools_build-utils/index.md) for build scripts, native code, and build caches.
- [Development Tools - Cargo Plugins](../development-tools_cargo-plugins/index.md) for community Cargo subcommands and task runners.
- [Development Tools - Debugging](../development-tools_debugging/index.md) for debuggers, logs, traces, and diagnostics.
- [Development Tools - Profiling](../development-tools_profiling/index.md) for runtime and memory measurements.
- [Development Tools - Testing](../development-tools_testing/index.md) for tests, coverage, and fuzzing.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
