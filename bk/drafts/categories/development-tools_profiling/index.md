# Profiling and Performance

[![cat~development-tools::profiling][cat~development-tools::profiling~badge]][cat~development-tools::profiling]{{hi:Profiling}}

This section covers profiling tools to identify performance bottlenecks.

Flame graphs are excellent for visualizing CPU usage and identifying hot spots. System profilers like [`perf`][c~perf~docs]↗{{hi:perf}} provide more detailed information. Benchmarking helps you measure the impact of code changes. Memory profilers help you find memory leaks and excessive allocations. Tracing helps you understand the flow of your program.

| Topic | Rust Crates |
|---|---|
| Flame Graphs | [`cargo flamegraph`][c~flamegraph~docs]↗{{hi:flamegraph}} generates flame graphs from Rust programs. |
| System Profilers (In-depth Analysis) | [`perf`][c~perf~docs]↗{{hi:perf}} is a powerful system profiler for Linux. `cargo flamegraph` often uses `perf` under the hood. [`dtrace`][c~dtrace~docs]↗{{hi:dtrace}} is another system profiler for macOS, BSD. [`VTune`][vtune-profiler~website]↗{{hi:VTune}} (Intel) is a commercial profiler. |
| Benchmarking | Built-in [`cargo bench`][book~cargo~cargo-bench]↗{{hi:cargo bench}} allows you to write benchmarks directly in your Rust code. |
| In-Code Profiling (Specific Code Regions) | [`measure_time`][c~measure_time~docs]↗{{hi:measure_time}} is a simple crate for measuring the execution time of code blocks. |
| Memory Profiling | [`valgrind`][c~valgrind~docs]↗{{hi:valgrind}} (with [`massif`][valgrind~massif~website]↗{{hi:massif}} or [`memcheck`][valgrind~memcheck~website]↗{{hi:memcheck}}): While not Rust-specific, 'Valgrind' is a powerful tool for memory profiling and leak detection. You'd run your Rust program under Valgrind. |
| Tracing - Understanding Program Flow | [`tracing`][c~tracing~docs]↗{{hi:tracing}}: While not strictly a profiler in the performance sense, [`tracing`][c~tracing~docs]↗{{hi:tracing}} allows you to instrument your code with spans and events, which can be invaluable for understanding the flow of execution and identifying bottlenecks. Often used in combination with other profiling tools. |
| Sampling Profilers (CPU Usage) | [`samply`][c~samply~docs]↗{{hi:samply}} is a native sampling profiler focusing on ease of use. [`callgrind`][c~callgrind~docs]↗{{hi:callgrind}} is a performance analysis tool often used with `kcachegrind` for visualization. |

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
