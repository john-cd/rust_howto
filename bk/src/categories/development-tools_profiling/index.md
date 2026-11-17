<!-- TODO Reviewed ??-??-2026. -->

# Development Tools - Profiling

[![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}

Use a representative workload to investigate CPU time, memory use, or code-level timing. A sampling profiler locates hot functions with little source instrumentation; an instrumentation profiler measures scopes that you mark in the code; a benchmark compares a specific workload across runs. Keep the target, build profile, input, and machine consistent when comparing measurements.

## CPU and System Profiling

Flame graphs help locate hot functions. System profilers provide additional detail, but their setup depends on the operating system and may require permissions or kernel support. On Linux, `perf` may need access to hardware performance counters; macOS and BSD use different system tools.

| Tool | Description |
| --- | --- |
| [`cargo flamegraph`][c~flamegraph~docs]↗{{hi:flamegraph}} | Samples a Rust program and generates a flame graph. |
| [`samply`][c~samply~docs]↗{{hi:samply}} | A sampling profiler for macOS and Linux with a web-based visualizer. |
| [`perf`][c~perf~docs]↗{{hi:perf}} | Linux system profiler for CPU samples and hardware counters. |
| [`dtrace`][c~dtrace~docs]↗{{hi:dtrace}} | System tracing and profiling on macOS and BSD. |
| [`VTune`][vtune-profiler~website]↗{{hi:VTune}} | Intel's commercial performance profiler. |

## Instrumentation Profiling

Mark functions or scopes in your code to measure where a frame or request spends time. The [`profiling`][c~profiling~docs]↗ facade lets an application select a backend; [`puffin`][c~puffin~docs]↗ records frame-oriented scopes for a viewer.

{{#include instrumentation_profiling.incl.md}}

## Benchmarking

Benchmark first to establish a repeatable baseline. Use a function-level benchmark for a hot path and a command-line benchmark for end-to-end behavior. Do not treat one measurement as proof of a performance change.

| Tool | Description |
| --- | --- |
| [`cargo bench`][book~cargo~cargo-bench]↗{{hi:cargo bench}} | Runs Rust benchmark targets in optimized mode. |
| [`criterion`][c~criterion~docs]↗{{hi:criterion}} | A statistics-oriented Rust benchmarking library. |
| [`divan`][c~divan~docs]↗{{hi:divan}} | A microbenchmarking library with allocation profiling support. |
| [`hyperfine`][c~hyperfine~docs]↗{{hi:hyperfine}} | A command-line tool for comparing whole-process commands. |

{{#include benchmarking.incl.md}}

## Memory Analysis

Distinguish process memory from heap allocation data. RSS and peak working set include more than Rust heap allocations; heap profilers record allocation sites or heap use over time. Valgrind and Heaptrack require supported host tools, and profiling overhead can change the observed behavior.

{{#include memory_usage_analysis.incl.md}}

## Inspect Generated Code

Inspect assembly after measurement identifies a meaningful cost. Generated instructions vary with compiler, target, features, and build profile.

{{#include assembly.incl.md}}

## Related Topics {#related-topics .skip}

- [Development Tools - Debugging](../development-tools_debugging/index.md) for logs, traces, and runtime diagnostics.
- [Diagnose Rust Build Time](../development-tools/build_time_diagnosis.md) for compile-time analysis.
- [Compare Benchmark Results in CI](benchmark_regressions.md) for repeatable revision comparisons.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
