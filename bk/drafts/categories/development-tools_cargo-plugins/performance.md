# Performance

{{#include performance.incl.md}}

| Topic | Rust Crates |
|---|---|
| Profiling and Benchmarking | `cargo flamegraph` generates flame graphs to visualize performance bottlenecks. `cargo bench` (built-in) runs benchmarks to measure performance. |
| Optimization Analysis| `cargo expand` expands [macros][p~macros], which can sometimes help understand generated code and identify optimization opportunities. |
| Other Performance Tools | [`perf`][c~perf~docs]↗{{hi:perf}}: (System profiler); [`valgrind`][c~valgrind~docs]↗{{hi:valgrind}} (Memory debugging and profiling) |

## Configure Your `cargo` Project for Maximum Performance, Fast Compile Times or Minimal Binary Size {#cargo-wizard}

[![cargo-wizard][c~cargo-wizard~docs~badge]][c~cargo-wizard~docs] [![cargo-wizard~crates.io][c~cargo-wizard~crates.io~badge]][c~cargo-wizard~crates.io] [![cargo-wizard~repo][c~cargo-wizard~repo~badge]][c~cargo-wizard~repo] [![cargo-wizard~lib.rs][c~cargo-wizard~lib.rs~badge]][c~cargo-wizard~lib.rs]{{hi:cargo-wizard}}{{hi:Profile}}{{hi:Cargo}}{{hi:Template}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[`cargo wizard`][c~cargo-wizard~repo]↗{{hi:cargo-wizard}} is a [`cargo`][c~cargo~docs]↗{{hi:cargo}} subcommand for configuring Cargo projects. It applies profile and config templates to your Cargo project to configure it for maximum performance, fast compile times or minimal binary size.

## `cargo hakari` {#cargo-hakari}

[![cargo-hakari][c~cargo-hakari~docs~badge]][c~cargo-hakari~docs] [![cargo-hakari~crates.io][c~cargo-hakari~crates.io~badge]][c~cargo-hakari~crates.io] [![cargo-hakari~repo][c~cargo-hakari~repo~badge]][c~cargo-hakari~repo] [![cargo-hakari~lib.rs][c~cargo-hakari~lib.rs~badge]][c~cargo-hakari~lib.rs]{{hi:cargo-hakari}}{{hi:Cargo}}{{hi:Dependencies}}{{hi:Feature-unification}}{{hi:Guppy}}{{hi:Workspace-hack}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

[cargo-hakari][c~cargo-hakari~crates.io]↗{{hi:cargo-hakari}} manage "workspace-hack" packages to speed up builds in large workspaces.

`cargo hakari` is a command-line application to manage "workspace-hack" [crates][p~crates]. Use it to speed up local `cargo build` and `cargo check` commands by up to 100x, and cumulatively by up to 1.7x or more.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[performance: expand](https://github.com/john-cd/rust_howto/issues/314)
</div>
