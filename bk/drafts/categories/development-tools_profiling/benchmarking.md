# Benchmarking

{{#include benchmarking.incl.md}}

## `cargo flamegraph` {#cargo-flamegraph}

[![cargo-flamegraph][c~flamegraph~docs~badge]][c~flamegraph~docs]{{hi:cargo-flamegraph}}
[![cargo-flamegraph~crates.io][c~flamegraph~crates.io~badge]][c~flamegraph~crates.io]
[![cargo-flamegraph~github][c~flamegraph~github~badge]][c~flamegraph~github]
[![cargo-flamegraph~lib.rs][c~flamegraph~lib.rs~badge]][c~flamegraph~lib.rs]

`cargo flamegraph` generates execution flamegraphs.

## `criterion` {#criterion}

[![criterion][c~criterion~docs~badge]][c~criterion~docs]{{hi:criterion}}
[![criterion~crates.io][c~criterion~crates.io~badge]][c~criterion~crates.io]
[![criterion~github][c~criterion~github~badge]][c~criterion~github]
[![criterion~lib.rs][c~criterion~lib.rs~badge]][c~criterion~lib.rs]
[![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}

[`criterion`][c~criterion~docs]↗{{hi:criterion}} is a statistically accurate benchmarking tool. [`criterion`][c~criterion~docs]↗{{hi:criterion}} helps you write fast code by detecting and measuring [performance][p~performance] improvements or regressions, even small ones, quickly and accurately. You can optimize with confidence, knowing how each change affects the [performance][p~performance] of your code.

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/examples/benchmarking/criterion.rs:example}}
```

## `divan` {#divan}

[![divan][c~divan~docs~badge]][c~divan~docs]{{hi:divan}}
[![divan~crates.io][c~divan~crates.io~badge]][c~divan~crates.io]
[![divan~github][c~divan~github~badge]][c~divan~github]
[![divan~lib.rs][c~divan~lib.rs~badge]][c~divan~lib.rs]
[![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}

[`divan`][c~divan~docs]↗{{hi:divan}} is a simple yet powerful benchmarking library with allocation profiling.

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/benches/divan.rs:example}}
```

## `hyperfine` {#hyperfine}

[![hyperfine][c~hyperfine~docs~badge]][c~hyperfine~docs]{{hi:hyperfine}}
[![hyperfine~crates.io][c~hyperfine~crates.io~badge]][c~hyperfine~crates.io]
[![hyperfine~github][c~hyperfine~github~badge]][c~hyperfine~github]
[![hyperfine~lib.rs][c~hyperfine~lib.rs~badge]][c~hyperfine~lib.rs]
[![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}

[`hyperfine`][c~hyperfine~docs]↗{{hi:hyperfine}} is a tool for benchmarking compiled binaries (similar to unix 'time' command but better).

- Statistical analysis across multiple runs.
- Support for arbitrary shell commands.
- Constant feedback about the benchmark progress and current estimates.
- Warm-up runs can be executed before the actual benchmark.
- Cache-clearing commands can be set up before each timing run.
- Statistical outlier detection to detect interference from other programs and [caching][p~caching] effects.
- Export results to various formats: [CSV][p~csv], [JSON][p~json], [Markdown][p~markdown], AsciiDoc.
- Parameterized benchmarks (e.g. vary the number of threads).
- Cross-platform.

```sh
cargo install --locked hyperfine
# or
apt install hyperfine
```

```sh
hyperfine 'sleep 0.3'
# Change the number of runs to perform
hyperfine --runs 5 'sleep 0.3'
# Compare the run times of different programs
hyperfine 'hexdump file' 'xxd file'
# Run the benchmark on a warm cache
hyperfine --warmup 3 'grep -R TODO *'
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[benchmarking: write](https://github.com/john-cd/rust_howto/issues/335)
</div>
