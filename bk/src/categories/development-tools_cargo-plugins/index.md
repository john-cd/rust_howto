# Cargo Plugins

[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

Subcommands that extend the capabilities of Cargo.

## Writing code

{{#include writing.incl.md}}

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

cargo-afl for fuzzing

cargo-asm and cargo-expand to investigate what the compiler generates from your code (cargo-expand shows the expanded output from macros)

cargo-audit for checking whether any of your dependencies are of a version that has a security advisory out against them.

cargo-bloat for identifying what's contributing to your binary's size (eg. modules with generic functions or macros not designed with size-efficiency in mind)

cargo-cache, cargo-sweep, and cargo-prune for keeping disk consumption by build artifacts and other regeneratable files under control.

cargo-deadlinks Check cargo doc output for broken old-style/manual intra-doc links.

cargo-edit for cargo add <dependency>, cargo rm <dependency> and cargo upgrade to update your Cargo.toml's versions. (Functionality eventually planned to be part of Cargo itself)

cargo-geiger for identifying dependencies with unsafe code so you can either audit them or find alternatives if you don't feel skilled enough to do your own auditing.

cargo-modules for rendering a tree or Graphviz graph of the modules within a crate

cargo-outdated for listing packages that have newer versions than what your Cargo.toml and Cargo.lock are pinning.

cargo-tree to investigate dependencies (I like to use -d to list crates where more than one version is getting pulled in and what's pulling each version in.)

cargo-update to provide a cargo install-update to check for and install new versions of cargo install's commands.

cargo-watch to re-run a command every time the source changes. (eg. cargo test)

flamegraph as an easy way to generate flamegraphs for visualizing performance profiles of Rust programs.

cargo-about, cargo-deny, cargo-license, or cargo-lichking for license compliance management

cargo-audit and cargo-sweep

cargo-spellcheck
</div>
