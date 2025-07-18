# Cargo Plugins

[![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

Subcommands that extend the capabilities of Cargo.

| Topic | Rust Crates |
|---|---|
| Building | [`cargo-cache`][c~cargo_cache~docs]⮳{{hi:cargo-cache}}, [`cargo-sweep`][c~cargo_sweep~docs]⮳{{hi:cargo-sweep}}, and [`cargo-prune`][c~cargo_prune~docs]⮳{{hi:cargo-prune}} keep disk consumption by build artifacts and other regenerable files under control. |
| Code Quality & Analysis |  `cargo fmt` (built-in) formats your code. `cargo clippy` (built-in) lints your code. [`cargo-spellcheck`][c~cargo_spellcheck~docs]⮳{{hi:cargo-spellcheck}} checks for spelling errors. |
| Dependency Management | [`cargo-update`][c~cargo_update~docs]⮳{{hi:cargo-update}} updates dependencies as recorded in the local lock file (built-in). `cargo tree` (built-in) displays your dependency tree. Use `-d` to list crates where more than one version is getting pulled in and what's pulling each version in. [`cargo-outdated`][c~cargo_outdated~docs]⮳{{hi:cargo-outdated}} lists packages that have newer versions than what your `Cargo.toml` and `Cargo.lock` are pinning. `cargo add` adds dependencies to your Cargo.toml. `cargo rm` removes dependencies. [`cargo-edit`][c~cargo_edit~docs]⮳{{hi:cargo-edit}} edits your `Cargo.toml` file. It adds `cargo add <dependency>`, `cargo rm <dependency>` and `cargo upgrade` to update your Cargo.toml's versions. This functionality is planned to be part of Cargo itself. |
| Testing/Benchmarking | `cargo test` (built-in) runs your tests (built-in, but often considered a plugin). `cargo bench` (built-in) runs your benchmarks. `cargo fuzz` or [`cargo-afl`][c~cargo_afl~docs]⮳{{hi:cargo-afl}} run your fuzz tests. |
| Code Coverage | [`cargo-tarpaulin`][c~cargo_tarpaulin~docs]⮳{{hi:cargo-tarpaulin}} runs code coverage analysis. |
| Documentation | `cargo doc` generates documentation (built-in). [`cargo-deadlinks`][c~cargo_deadlinks~docs]⮳{{hi:cargo-deadlinks}} checks `cargo doc` output for broken old-style or manual intra-doc links. |
| Profiling | [`cargo-flamegraph`][c~cargo_flamegraph~docs]⮳{{hi:cargo-flamegraph}} generates flame graphs for profiling. |
| Code Inspection | Use [`cargo-asm`][c~cargo_asm~docs]⮳{{hi:cargo-asm}} to investigate what the compiler generates from your code. [`cargo-expand`][c~cargo_expand~docs]⮳{{hi:cargo-expand}} shows the expanded output from macros. [`cargo-modules`][c~cargo_modules~docs]⮳{{hi:cargo-modules}} renders a tree or Graphviz graph of the modules within a crate. |
| Security | [`cargo-audit`][c~cargo_audit~docs]⮳{{hi:cargo-audit}} checks whether any of your dependencies are of a version that has a security advisory out against them. [`cargo-geiger`][c~cargo_geiger~docs]⮳{{hi:cargo-geiger}} identifies dependencies with unsafe code, so you can either audit them or find alternatives. |
| Licensing | Use [`cargo-about`][c~cargo_about~docs]⮳{{hi:cargo-about}}, [`cargo-deny`][c~cargo_deny~docs]⮳{{hi:cargo-deny}}, [`cargo-license`][c~cargo_license~docs]⮳{{hi:cargo-license}}, or [`cargo-lichking`][c~cargo_lichking~docs]⮳{{hi:cargo-lichking}} for license compliance management. |
| Binary Size Optimization | [`cargo-bloat`][c~cargo_bloat~docs]⮳{{hi:cargo-bloat}} identifies what's contributing to your binary's size (eg. modules with generic functions or macros not designed with size-efficiency in mind). |
| Publishing, Distribution | `cargo publish` publishes your crate to crates.io (built-in). [`cargo-deb`][c~cargo_deb~docs]⮳{{hi:cargo-deb}} creates Debian packages. [`cargo-rpm`][c~cargo_rpm~docs]⮳{{hi:cargo-rpm}} creates RPM packages. |
| Change Watching | [`cargo-watch`][c~cargo_watch~docs]⮳{{hi:cargo-watch}} watches your project for changes and rebuilds / re-run a command every time the source changes (e.g. `cargo test`). |

## Writing Code

{{#include code_writing.incl.md}}

## Formatting and Linting

{{#include code_formatting_linting.incl.md}}

## Dependency Management

{{#include dependency_management.incl.md}}

## Building

{{#include building.incl.md}}

## Watching for Changes

{{#include watching_for_changes.incl.md}}

## Cross-compiling

{{#include cross_compiling.incl.md}}

## Auditing

{{#include auditing.incl.md}}

## Performance

{{#include performance.incl.md}}

## Maintenance

{{#include maintaining.incl.md}}

## Creating a Cargo Plugin

Cargo plugins are essentially just executables that follow a certain naming convention (i.e. `cargo-something`). To create a Cargo Plugin, create a regular Rust project (often a binary crate) and name the executable `cargo-<your name here>`. Cargo will automatically discover and run these executables.

### Useful Crates to Create a Cargo Plugin

| [[argument_parsing | Argument Parsing]] | Use [`clap`][c~clap~docs]⮳{{hi:clap}}, [`structopt`][c~structopt~docs]⮳{{hi:structopt}}, [`argh`][c~argh~docs]⮳{{hi:argh}} for parsing command-line arguments passed to the plugin. |
| Working with [[cargo | Cargo]] | Interact with `Cargo.toml` or other Cargo metadata. [`serde`][c~serde~docs]⮳{{hi:serde}} is often used for parsing TOML or JSON. |
| [[filesystem | Filesystem]] Operations | Use `std::fs`, [`pathdiff`][c~pathdiff~docs]⮳{{hi:pathdiff}}. |
| Process Management | Use `std::process` for running [[external_commands | External Commands]]. |
| Networking, [[web-programming_http-client | HTTP Client]] (if needed) | Use [`reqwest`][c~reqwest~docs]⮳{{hi:reqwest}}, [`hyper`][c~hyper~docs]⮳{{hi:hyper}}. |
| [[serde | Serialization/Deserialization]] | Use [`serde`][c~serde~docs]⮳{{hi:serde}} for handling configuration or data. |
| [[log | Logging]] | Use `tracing`, or[`log`][c~log~docs]⮳{{hi:log}} and [`env_logger`][c~env_logger~docs]⮳{{hi:env_logger}}. |

There is an unstable Cargo API, but it's not recommended for most plugins due to its instability.

## Related Topics

- [[development-tools_build-utils | Development Tools: Build Utils]].
- [[development-tools_debugging | Development Tools: Debugging]].
- [[development-tools_ffi | Development Tools: FFI]].
- [[development-tools_procedural-macro-helpers | Development Tools: Procedural Macro Helpers]]
- [[development-tools_profiling | Development Tools: Profiling]].
- [[testing | Testing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review in depth, reorg table NOW](https://github.com/john-cd/rust_howto/issues/311)

- [cargo-prefetch: Cargo subcommand to download popular crates.](https://github.com/ehuss/cargo-prefetch)
- [cargo-cyclonedx](https://crates.io/crates/cargo-cyclonedx)

</div>
