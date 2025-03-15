# Performance

{{#include performance.incl.md}}

## Configure your `cargo` project for Maximum Performance, Fast Compile Times or Minimal Binary Size {#cargo-wizard}

[![cargo-wizard][c-cargo_wizard-badge]][c-cargo_wizard] [![cargo-wizard-crates.io][c-cargo_wizard-crates.io-badge]][c-cargo_wizard-crates.io] [![cargo-wizard-github][c-cargo_wizard-github-badge]][c-cargo_wizard-github] [![cargo-wizard-lib.rs][c-cargo_wizard-lib.rs-badge]][c-cargo_wizard-lib.rs]{{hi:cargo-wizard}}{{hi:Profile}}{{hi:Cargo}}{{hi:Template}} [![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo wizard`][c-cargo_wizard-github]{{hi:cargo-wizard}}⮳ is a [`cargo`][c-cargo]⮳{{hi:cargo}} subcommand for configuring Cargo projects. It applies profile and config templates to your Cargo project to configure it for maximum performance, fast compile times or minimal binary size.

## `cargo hakari` {#cargo-hakari}

[![cargo-hakari][c-cargo_hakari-badge]][c-cargo_hakari] [![cargo-hakari-crates.io][c-cargo_hakari-crates.io-badge]][c-cargo_hakari-crates.io] [![cargo-hakari-github][c-cargo_hakari-github-badge]][c-cargo_hakari-github] [![cargo-hakari-lib.rs][c-cargo_hakari-lib.rs-badge]][c-cargo_hakari-lib.rs]{{hi:cargo-hakari}}{{hi:Cargo}}{{hi:Dependencies}}{{hi:Feature-unification}}{{hi:Guppy}}{{hi:Workspace-hack}} [![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

[cargo-hakari][c-cargo_hakari-crates.io]{{hi:cargo-hakari}}⮳ manage "workspace-hack" packages to speed up builds in large workspaces.

`cargo hakari` is a command-line application to manage "workspace-hack" [crates][p-crates]. Use it to speed up local `cargo build` and `cargo check` commands by up to 100x, and cumulatively by up to 1.7x or more.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[performance: expand](https://github.com/john-cd/rust_howto/issues/314)

Profiling:

cargo flamegraph: Generates flame graphs to visualize performance bottlenecks.
cargo bench: (Built-in) Runs benchmarks to measure performance.
Optimization Analysis:

cargo expand: Expands [macros][p-macros], which can sometimes help you understand generated code and identify optimization opportunities. (Not strictly a performance tool itself, but useful).
Benchmarking:

cargo bench: (Built-in) Runs benchmarks to measure performance.
Other Performance Tools: (Often used in conjunction with Rust, but not [Cargo][p-cargo] plugins)

perf: (System profiler)
valgrind (Memory debugging and profiling)
</div>
