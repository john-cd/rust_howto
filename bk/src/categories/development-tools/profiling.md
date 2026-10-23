<!-- TODO Reviewed ??-??-2026 -->
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

{{#include ../../refs/link-refs.md}}
