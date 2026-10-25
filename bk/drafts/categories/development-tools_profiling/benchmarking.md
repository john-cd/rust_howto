<!-- TODO Reviewed ??-??-2026 -->

# Benchmarking

{{#include benchmarking.incl.md}}

## `cargo flamegraph` {#cargo-flamegraph}

[![cargo-flamegraph][c~flamegraph~docs~badge]][c~flamegraph~docs]{{hi:cargo-flamegraph}}
[![cargo-flamegraph~crates.io][c~flamegraph~crates.io~badge]][c~flamegraph~crates.io]
[![cargo-flamegraph~repo][c~flamegraph~repo~badge]][c~flamegraph~repo]
[![cargo-flamegraph~lib.rs][c~flamegraph~lib.rs~badge]][c~flamegraph~lib.rs]

`cargo flamegraph` samples a running program and writes a flamegraph that helps
locate hot functions. It complements a benchmark: use it to explain a measured
regression, then rerun the benchmark after a change. Install it with:

```console
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/install_flamegraph.sh}}
```

From the `bk/crates` workspace, pass the package and target you want to profile:

```console
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/flamegraph.sh}}
```

## `criterion` {#criterion}

[![criterion][c~criterion~docs~badge]][c~criterion~docs]{{hi:criterion}}
[![criterion~crates.io][c~criterion~crates.io~badge]][c~criterion~crates.io]
[![criterion~repo][c~criterion~repo~badge]][c~criterion~repo]
[![criterion~lib.rs][c~criterion~lib.rs~badge]][c~criterion~lib.rs]
[![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}

[`criterion`][c~criterion~docs]↗{{hi:criterion}} measures a Rust function over
repeated samples and reports statistical comparisons with prior runs. Keep
setup outside the measured closure when possible, and use `black_box` to keep
the input and result observable to the optimizer. Run the example in release
mode with:

```console
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/run_criterion.sh}}
```

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/criterion.rs:example}}
```

## `divan` {#divan}

[![divan][c~divan~docs~badge]][c~divan~docs]{{hi:divan}}
[![divan~crates.io][c~divan~crates.io~badge]][c~divan~crates.io]
[![divan~repo][c~divan~repo~badge]][c~divan~repo]
[![divan~lib.rs][c~divan~lib.rs~badge]][c~divan~lib.rs]
[![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}

[`divan`][c~divan~docs]↗{{hi:divan}} is a simple yet powerful benchmarking library with allocation profiling.

Add the benchmark as a Cargo bench target with `harness = false`, then run it
with:

```console
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/run_divan.sh}}
```

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/benches/divan.rs:example}}
```

## `hyperfine` {#hyperfine}

[![hyperfine][c~hyperfine~docs~badge]][c~hyperfine~docs]{{hi:hyperfine}}
[![hyperfine~crates.io][c~hyperfine~crates.io~badge]][c~hyperfine~crates.io]
[![hyperfine~repo][c~hyperfine~repo~badge]][c~hyperfine~repo]
[![hyperfine~lib.rs][c~hyperfine~lib.rs~badge]][c~hyperfine~lib.rs]
[![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}

[`hyperfine`][c~hyperfine~docs]↗{{hi:hyperfine}} measures whole commands and
reports run-to-run variation. It is useful for comparing end-to-end behavior;
run the same representative input for each command. Install it with:

```console
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/install_hyperfine.sh}}
```

```console
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/hyperfine.sh}}
```

## Related Topics {#related-topics .skip}

- [Debugging](../development-tools_debugging/index.md)
- [Cargo Plugins](../development-tools_cargo-plugins/index.md)
- [Memory Usage Analysis][p~memory-usage-analysis]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
