# Profiling and Performance

[![cat-development-tools::profiling][cat-development-tools::profiling-badge]][cat-development-tools::profiling]{{hi:Profiling}}

Development tools to help you figure out the performance of your code.

## Benchmarking

{{#include benchmarking.incl.md}}

## Memory usage analysis

{{#include memory_usage_analysis.incl.md}}

## Low-level profiling tools

{{#include assembly.incl.md}}

## See Also

[Rust Performance Book][book-rust-performance] [![book-rust-performance-github][book-rust-performance-github-badge]][book-rust-performance-github]{{hi:Rust performance}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[expand](https://github.com/john-cd/rust_howto/issues/337)

Profiling: Using profiling tools to identify performance bottlenecks.

Performance Optimization: Recipes for optimizing code performance, including using SIMD instructions and profiling tools.

---

## Flame Graphs

`cargo flamegraph`: The most popular tool for generating flame graphs from Rust programs. It integrates well with Cargo and makes profiling very easy.

## System Profilers (for more in-depth analysis)

[`perf`][c-perf]⮳{{hi:perf}} (Linux): A powerful system profiler. `cargo flamegraph` often uses perf under the hood.
[`dtrace`][c-dtrace]⮳{{hi:dtrace}} (macOS, BSD): Another system profiler.
`VTune` (Intel): A commercial profiler.

## Benchmarking2

`cargo bench`: (Built-in) Allows you to write benchmarks directly in your Rust code.

## In-Code Profiling (for specific regions)

[`measure_time`][c-measure_time]⮳{{hi:measure_time}}: A simple crate for measuring the execution time of code blocks.

## Memory Profiling

[`valgrind`][c-valgrind]⮳{{hi:valgrind}} (with `massif` or `memcheck`): While not Rust-specific, Valgrind is a powerful tool for memory profiling and leak detection. You'd run your Rust program under Valgrind.

## Tracing (for understanding program flow)

[`tracing`][c-tracing]⮳{{hi:tracing}}: While not strictly a profiler in the performance sense, [`tracing`][c-tracing]⮳{{hi:tracing}} allows you to instrument your code with spans and events, which can be invaluable for understanding the flow of execution and identifying bottlenecks. Often used in combination with other profiling tools.

## Sampling Profilers (for CPU usage)

[`samply`][c-samply]⮳{{hi:samply}}: A native sampling profiler focusing on ease of use.

Callgrind:

[`callgrind`][c-callgrind]⮳{{hi:callgrind}}: A performance analysis tool. Often used with kcachegrind for visualization.

It's important to choose the right profiling tool for the job. Flame graphs are excellent for visualizing CPU usage and identifying hot spots. System profilers like perf provide more detailed information. Benchmarking helps you measure the impact of code changes. Memory profilers help you find memory leaks and excessive allocations. Tracing helps you understand the flow of your program.

</div>
