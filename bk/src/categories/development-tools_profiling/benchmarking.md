<!-- TODO Reviewed ??-??-2026. -->

# Benchmarking

{{#include benchmarking.incl.md}}

## `cargo flamegraph` {#cargo-flamegraph}

[![cargo-flamegraph][c~flamegraph~docs~badge]][c~flamegraph~docs]{{hi:cargo-flamegraph}}
[![cargo-flamegraph~crates.io][c~flamegraph~crates.io~badge]][c~flamegraph~crates.io]
[![cargo-flamegraph~repo][c~flamegraph~repo~badge]][c~flamegraph~repo]
[![cargo-flamegraph~lib.rs][c~flamegraph~lib.rs~badge]][c~flamegraph~lib.rs]

`cargo flamegraph` samples a running program and writes a flame graph that
helps locate hot functions. It complements a benchmark: use it to explain a
measured regression, then rerun the benchmark after a change. Profiling support
depends on the host; Linux may require `perf` to be installed and configured.
Install the Cargo tool with:

```console
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/install_flamegraph.sh}}
```

From the `bk/crates` workspace, pass the package and target you want to profile.
The script writes separate files for the example and benchmark:

```console
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/flamegraph.sh}}
```

## `criterion` {#criterion}

{{#badges criterion development-tools::profiling=Profiling}}

[`criterion`][c~criterion~docs]↗{{hi:criterion}} measures a Rust function over
repeated samples and reports statistical comparisons with prior runs. Keep
setup outside the measured closure when possible, and use `black_box` to keep
the input and result observable to the optimizer. Run the example in release
mode with:

```console
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/run_criterion.sh}}
```

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p development_tools_profiling --example benchmarking`.

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/criterion.rs:example}}
```

## `divan` {#divan}

{{#badges divan development-tools::profiling=Profiling}}

[`divan`][c~divan~docs]↗{{hi:divan}} is a Rust benchmarking library with
allocation profiling support. This package already declares the benchmark
target with `harness = false`.

Run the benchmark target with:

```console
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/run_divan.sh}}
```

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/benches/divan.rs:example}}
```

## `hyperfine` {#hyperfine}

{{#badges hyperfine command-line-utilities="Command line utilities"}}

[`hyperfine`][c~hyperfine~docs]↗{{hi:hyperfine}} measures whole commands and
reports run-to-run variation. It is useful for comparing end-to-end behavior;
run equivalent commands with the same representative input. Install it with:

```console
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/install_hyperfine.sh}}
```

The included script builds a deterministic sorting workload, compares stable
and unstable sorting of the same input, and saves a Markdown report. It is a
Bash script; on Windows, use the corresponding `.exe` target path in a native
Hyperfine command.

```console
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/hyperfine.sh}}
```

## Criterion Benchmarks from the Book {#book-benchmarks}

These Criterion targets compare real implementations from the book's example
crates. Run them from `bk/crates`; results depend on the machine and build
configuration, so use them to compare changes on the same system.

### Hash Functions {#hashing-benchmark}

Compare hash functions over small and medium byte buffers:

```console
cargo bench --package algorithms --bench hashing
```

Run this benchmark from `bk/crates` with `cargo bench --locked -p algorithms --bench hashing`.

```rust,noplayground
{{#include ../../../crates/cats/algorithms/benches/hashing.rs:example}}
```

### Arena and Heap Allocation {#arena-benchmark}

Compare `bumpalo` arena allocation with `Box` allocation:

```console
cargo bench --package memory_management --bench arena
```

Run this benchmark from `bk/crates` with `cargo bench --locked -p memory_management --bench arena`.

```rust,noplayground
{{#include ../../../crates/cats/memory_management/benches/arena.rs:example}}
```

### Hash Map Implementations {#hashmap-benchmark}

Compare the standard library, `ahash`, and `rustc-hash` for string and integer
keys:

```console
cargo bench --package data_structures --bench hashmap
```

Run this benchmark from `bk/crates` with `cargo bench --locked -p data_structures --bench hashmap`.

```rust,noplayground
{{#include ../../../crates/cats/data_structures/benches/hashmap.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Development Tools - Debugging](../development-tools_debugging/index.md).
- [Instrumentation Profiling](instrumentation_profiling.md).
- [Memory Usage Analysis](memory_usage_analysis.md).
- [Compare Benchmark Results in CI](benchmark_regressions.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
