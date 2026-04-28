# Profiling and Performance

[![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}

This section covers profiling tools to identify performance bottlenecks, analyze memory usage, and benchmark your code.

### CPU & System Profiling

Flame graphs are excellent for visualizing CPU usage and identifying hot spots. System profilers like [`perf`][c~perf~docs]↗{{hi:perf}} provide more detailed information.

| Tool | Description |
|---|---|
| [`cargo flamegraph`][c~flamegraph~docs]↗{{hi:flamegraph}} | Generates flame graphs from Rust programs. |
| [`samply`][c~samply~docs]↗{{hi:samply}} | A sampling profiler for macOS and Linux. It records a profile and opens it in a web-based visualizer. |
| [`perf`][c~perf~docs]↗{{hi:perf}} | A powerful system profiler for Linux. |
| [`dtrace`][c~dtrace~docs]↗{{hi:dtrace}} | A system profiler for macOS and BSD. |
| [`VTune`][vtune-profiler~website]↗{{hi:VTune}} | A commercial performance profiler from Intel. |

### Benchmarking

Benchmarking helps measure the impact of code changes and ensure performance stability.

| Tool | Description |
|---|---|
| [`cargo bench`][book~cargo~cargo-bench]↗{{hi:cargo bench}} | Built-in tool for writing benchmarks directly in your Rust code. |
| [`criterion`][c~criterion~docs]↗{{hi:criterion}} | A statistically accurate benchmarking tool for Rust. |
| [`divan`][c~divan~docs]↗{{hi:divan}} | A simple yet powerful benchmarking library with allocation profiling. |
| [`hyperfine`][c~hyperfine~docs]↗{{hi:hyperfine}} | A command-line benchmarking tool for comparing compiled binaries. |

### Memory Analysis

Memory profilers help find memory leaks and excessive allocations.

| Tool | Description |
|---|---|
| [`dhat`][c~dhat~docs]↗{{hi:dhat}} | A library for heap profiling and ad-hoc profiling. |
| [`valgrind`][c~valgrind~docs]↗{{hi:valgrind}} | A powerful tool for memory profiling (with `massif`) and leak detection (with `memcheck`). |
| [`heaptrack`][c~heaptrack~docs]↗{{hi:heaptrack}} | A heap profiler that tracks memory allocations. |

### In-Code Profiling & Tracing

| Tool | Description |
|---|---|
| [`measure_time`][c~measure_time~docs]↗{{hi:measure_time}} | A simple crate for measuring the execution time of code blocks. |
| [`tracing`][c~tracing~docs]↗{{hi:tracing}} | Instruments code with spans and events to understand program flow. |

## Code Examples

### Benchmarking

{{#include benchmarking.incl.md}}

### Memory Usage Analysis

{{#include memory_usage_analysis.incl.md}}

### Low-level Profiling Tools

{{#include assembly.incl.md}}

## References

- The [Rust Performance Book][book~rust-performance]↗. [![book~rust-performance~repo][book~rust-performance~repo~badge]][book~rust-performance~repo]{{hi:Rust performance}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[expand / review](https://github.com/john-cd/rust_howto/issues/337)
</div>
