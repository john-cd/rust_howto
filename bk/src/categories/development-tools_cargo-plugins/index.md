# Cargo Plugins

[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

Subcommands that extend the capabilities of Cargo.

## Writing code

{{#include code_writing.incl.md}}

## Formatting and Linting

{{#include code_formatting_linting.incl.md}}

## Building

{{#include building.incl.md}}

## Watching for changes

{{#include watching_for_changes.incl.md}}

## Cross-compiling

{{#include cross_compiling.incl.md}}

## Auditing

{{#include auditing.incl.md}}

## Performance

{{#include performance.incl.md}}

## Maintenance

{{#include maintaining.incl.md}}

## See also

- Testing.
- Build Utils.
- Debugging.
- FFI.
- Procedural Macro Helpers.
- Profiling.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[development-tools_cargo-plugins/index: review](https://github.com/john-cd/rust_howto/issues/311)

## Dependency Management

{{#include dependency_management.incl.md}}

Here's a short list of Cargo plugins by topic:

Code Quality/Analysis:

`cargo clippy`: Lints your code.
`cargo fmt`: Formats your code.
`cargo deadlinks`: Finds broken links in your documentation.
`cargo audit`: Checks for dependencies with known vulnerabilities.

Dependency Management:

`cargo tree`: Displays your dependency tree.
`cargo outdated`: Checks for outdated dependencies.
`cargo add`: Adds dependencies to your Cargo.toml.
`cargo rm`: Removes dependencies.

Testing/Benchmarking:

`cargo test`: Runs your tests (built-in, but often considered a plugin).
`cargo bench`: Runs your benchmarks (built-in).
`cargo fuzz`: Runs your fuzz tests.

Documentation:

`cargo doc`: Generates documentation (built-in).

Publishing/Distribution:

`cargo publish`: Publishes your crate to crates.io (built-in).
[`cargo-deb`][c-cargo_deb]⮳{{hi:cargo-deb}}: Creates Debian packages.
[`cargo-rpm`][c-cargo_rpm]⮳{{hi:cargo-rpm}}: Creates RPM packages.

Miscellaneous:

[`cargo-tarpaulin`][c-cargo_tarpaulin]⮳{{hi:cargo-tarpaulin}}: Runs code coverage analysis.
[`cargo-flamegraph`][c-cargo_flamegraph]⮳{{hi:cargo-flamegraph}}: Generates flame graphs for profiling.
[`cargo-edit`][c-cargo_edit]⮳{{hi:cargo-edit}}: Edits your Cargo.toml file.
[`cargo-watch`][c-cargo_watch]⮳{{hi:cargo-watch}}: Watches your project for changes and rebuilds.

---

[`cargo-afl`][c-cargo_afl]⮳{{hi:cargo-afl}} for fuzzing

[`cargo-asm`][c-cargo_asm]⮳{{hi:cargo-asm}} and [`cargo-expand`][c-cargo_expand]⮳{{hi:cargo-expand}} to investigate what the compiler generates from your code ([`cargo-expand`][c-cargo_expand]⮳{{hi:cargo-expand}} shows the expanded output from macros)

[`cargo-audit`][c-cargo_audit]⮳{{hi:cargo-audit}} for checking whether any of your dependencies are of a version that has a security advisory out against them.

[`cargo-bloat`][c-cargo_bloat]⮳{{hi:cargo-bloat}} for identifying what's contributing to your binary's size (eg. modules with generic functions or macros not designed with size-efficiency in mind)

[`cargo-cache`][c-cargo_cache]⮳{{hi:cargo-cache}}, [`cargo-sweep`][c-cargo_sweep]⮳{{hi:cargo-sweep}}, and [`cargo-prune`][c-cargo_prune]⮳{{hi:cargo-prune}} for keeping disk consumption by build artifacts and other regeneratable files under control.

[`cargo-deadlinks`][c-cargo_deadlinks]⮳{{hi:cargo-deadlinks}} Check `cargo doc` output for broken old-style/manual intra-doc links.

[`cargo-edit`][c-cargo_edit]⮳{{hi:cargo-edit}} for `cargo add <dependency>`, `cargo rm <dependency>` and `cargo upgrade` to update your Cargo.toml's versions. (Functionality eventually planned to be part of Cargo itself)

[`cargo-geiger`][c-cargo_geiger]⮳{{hi:cargo-geiger}} for identifying dependencies with unsafe code so you can either audit them or find alternatives if you don't feel skilled enough to do your own auditing.

[`cargo-modules`][c-cargo_modules]⮳{{hi:cargo-modules}} for rendering a tree or Graphviz graph of the modules within a crate

[`cargo-outdated`][c-cargo_outdated]⮳{{hi:cargo-outdated}} for listing packages that have newer versions than what your Cargo.toml and Cargo.lock are pinning.

[`cargo-tree`][c-cargo_tree]⮳{{hi:cargo-tree}} to investigate dependencies (I like to use -d to list crates where more than one version is getting pulled in and what's pulling each version in.)

[`cargo-update`][c-cargo_update]⮳{{hi:cargo-update}} to provide a cargo install-update to check for and install new versions of `cargo install` commands.

[`cargo-watch`][c-cargo_watch]⮳{{hi:cargo-watch}} to re-run a command every time the source changes. (eg. `cargo test`)

[`flamegraph`][c-flamegraph]⮳{{hi:flamegraph}} as an easy way to generate flamegraphs for visualizing performance profiles of Rust programs.

[`cargo-about`][c-cargo_about]⮳{{hi:cargo-about}}, [`cargo-deny`][c-cargo_deny]⮳{{hi:cargo-deny}}, [`cargo-license`][c-cargo_license]⮳{{hi:cargo-license}}, or [`cargo-lichking`][c-cargo_lichking]⮳{{hi:cargo-lichking}} for license compliance management

[`cargo-audit`][c-cargo_audit]⮳{{hi:cargo-audit}} and [`cargo-sweep`][c-cargo_sweep]⮳{{hi:cargo-sweep}}

[`cargo-spellcheck`][c-cargo_spellcheck]⮳{{hi:cargo-spellcheck}}

## Creating a Cargo plugin

Cargo plugins are essentially just executables that follow a certain naming convention (cargo-something). Therefore, there aren't specific crates for creating `cargo` plugins, but rather crates that are used within [`cargo`][c-cargo]⮳{{hi:cargo}} plugins. Here's a breakdown:

Creating a Cargo Plugin: You create a regular Rust project (often a binary crate) and name the executable cargo-something. Cargo will automatically discover and run these executables.

Essential Crates (for plugin functionality):

Argument Parsing: [`clap`][c-clap]⮳{{hi:clap}}, [`structopt`][c-structopt]⮳{{hi:structopt}}, [`argh`][c-argh]⮳{{hi:argh}} (for parsing command-line arguments passed to the plugin)
Working with Cargo: (You might interact with `Cargo.toml` or other Cargo metadata. No single crate, but [`serde`][c-serde]⮳{{hi:serde}} is often used for parsing TOML or JSON.)
File System Operations: `std::fs`, [`pathdiff`][c-pathdiff]⮳{{hi:pathdiff}}
Process Management: `std::process` (for running other commands)
Networking/HTTP (if needed): [`reqwest`][c-reqwest]⮳{{hi:reqwest}}, [`hyper`][c-hyper]⮳{{hi:hyper}}
Serialization/Deserialization: [`serde`][c-serde]⮳{{hi:serde}} (for handling configuration or data)
Logging: [`log`][c-log]⮳{{hi:log}}, [`env_logger`][c-env_logger]⮳{{hi:env_logger}}
Cargo API (Unstable): There is an unstable Cargo API that plugins could use, but it's not recommended for most plugins due to its instability. If you need very deep integration, you might explore it, but be prepared for changes.

In essence, you build a regular Rust application. The "Cargo plugin" aspect comes from the naming convention and how Cargo discovers and executes it. The crates you use depend entirely on what your plugin is designed to do.

</div>
