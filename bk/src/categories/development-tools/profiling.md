<!-- TODO Reviewed ??-??-2026. -->

# Profiling and Benchmarking

Benchmarks measure how long a workload takes on a particular compiler, target,
and machine. Keep benchmark inputs representative of the work you want to
measure, and compare runs made under the same conditions.

## Benchmark with Divan {#divan}

[`divan`][c~divan~docs]↗ is a Rust benchmarking framework. The example below
registers a CPU-bound Fibonacci workload. From the `bk/crates` workspace, run
it with:

```console
cargo bench -p development_tools_profiling --bench divan
```

The example follows the setup in [Nikolai Vazquez's Divan introduction][divan~blog]↗.

```rust
{{#include ../../../crates/cats/development_tools_profiling/benches/divan.rs:example}}
```

## Measure Build Time

For a repeatable diagnosis, see [Diagnose Rust Build Time](build_time_diagnosis.md).

For runtime comparisons between revisions, see [Compare Benchmark Results in CI](benchmark_regressions.md).

Run `cargo build --timings` and inspect Cargo's timing report to find expensive crate builds and the critical path. Keep toolchain, features, and cache state fixed when comparing changes. The [Corrode compilation guide][corrode~build-timings]↗ shows how to use the report to find a slow crate. [Feldera's build-time case study][feldera~build-times]↗ illustrates why dependency structure and feature choices also matter.

{{#include ../../refs/link-refs.md}}
