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

- Testing
- Build Utils
- Debugging
- FFI
- Procedural Macro Helpers
- Profiling

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[development-tools_cargo-plugins/index: review (P1)](https://github.com/john-cd/rust_howto/issues/311)

Here's a short list of Cargo plugins by topic:

Code Quality/Analysis:

cargo clippy: Lints your code.
cargo fmt: Formats your code.
cargo deadlinks: Finds broken links in your documentation.
cargo audit: Checks for dependencies with known vulnerabilities.

Dependency Management:

cargo tree: Displays your dependency tree.
cargo outdated: Checks for outdated dependencies.
cargo add: Adds dependencies to your Cargo.toml.
cargo rm: Removes dependencies.

Testing/Benchmarking:

cargo test: Runs your tests (built-in, but often considered a plugin).
cargo bench: Runs your benchmarks (built-in).
cargo fuzz: Runs your fuzz tests.
Documentation:

cargo doc: Generates documentation (built-in).

Publishing/Distribution:

cargo publish: Publishes your crate to crates.io (built-in).
cargo-deb: Creates Debian packages.
cargo-rpm: Creates RPM packages.

Miscellaneous:

cargo-tarpaulin: Runs code coverage analysis.
cargo-flamegraph: Generates flame graphs for profiling.
cargo-edit: Edits your Cargo.toml file.
cargo-watch: Watches your project for changes and rebuilds.

---

`cargo-afl` for fuzzing

`cargo-asm` and `cargo-expand` to investigate what the compiler generates from your code (`cargo-expand` shows the expanded output from macros)

`cargo-audit` for checking whether any of your dependencies are of a version that has a security advisory out against them.

`cargo-bloat` for identifying what's contributing to your binary's size (eg. modules with generic functions or macros not designed with size-efficiency in mind)

`cargo-cache`, `cargo-sweep`, and `cargo-prune` for keeping disk consumption by build artifacts and other regeneratable files under control.

`cargo-deadlinks` Check cargo doc output for broken old-style/manual intra-doc links.

`cargo-edit` for `cargo add <dependency>`, `cargo rm <dependency>` and `cargo upgrade` to update your Cargo.toml's versions. (Functionality eventually planned to be part of Cargo itself)

`cargo-geiger` for identifying dependencies with unsafe code so you can either audit them or find alternatives if you don't feel skilled enough to do your own auditing.

`cargo-modules` for rendering a tree or Graphviz graph of the modules within a crate

`cargo-outdated` for listing packages that have newer versions than what your Cargo.toml and Cargo.lock are pinning.

`cargo-tree` to investigate dependencies (I like to use -d to list crates where more than one version is getting pulled in and what's pulling each version in.)

`cargo-update` to provide a cargo install-update to check for and install new versions of cargo install's commands.

`cargo-watch` to re-run a command every time the source changes. (eg. `cargo test`)

`flamegraph` as an easy way to generate flamegraphs for visualizing performance profiles of Rust programs.

`cargo-about`, `cargo-deny`, `cargo-license`, or `cargo-lichking` for license compliance management

`cargo-audit` and `cargo-sweep`

`cargo-spellcheck`

## Creating a Cargo plugin

Cargo plugins are essentially just executables that follow a certain naming convention (cargo-something). Therefore, there aren't specific crates for creating cargo plugins, but rather crates that are used within cargo plugins. Here's a breakdown:

Creating a Cargo Plugin: You create a regular Rust project (often a binary crate) and name the executable cargo-something. Cargo will automatically discover and run these executables.

Essential Crates (for plugin functionality):

Argument Parsing: clap, structopt, argh (for parsing command-line arguments passed to the plugin)
Working with Cargo: (You might interact with Cargo.toml or other Cargo metadata. No single crate, but serde is often used for parsing TOML or JSON.)
File System Operations: std::fs, pathdiff
Process Management: std::process (for running other commands)
Networking/HTTP (if needed): reqwest, hyper
Serialization/Deserialization: serde (for handling configuration or data)
Logging: log, env_logger
Cargo API (Unstable): There is an unstable Cargo API that plugins could use, but it's not recommended for most plugins due to its instability. If you need very deep integration, you might explore it, but be prepared for changes.

In essence, you build a regular Rust application. The "Cargo plugin" aspect comes from the naming convention and how Cargo discovers and executes it. The crates you use depend entirely on what your plugin is designed to do.

</div>
