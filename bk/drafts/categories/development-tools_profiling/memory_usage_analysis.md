<!-- TODO Reviewed ??-??-2026 -->

# Memory Usage Analysis

{{#include memory_usage_analysis.incl.md}}

## Measure process memory {#process-memory}

Start by deciding which quantity matters. Resident set size (RSS) or a platform's peak working set includes more than Rust heap allocations: it can include stacks, memory-mapped files, code pages, and allocator-retained memory. Heap allocation profilers answer a narrower question about allocations made by the program.

On Linux, `/usr/bin/time -v ./target/release/my_app` reports peak resident memory for one run. macOS provides peak-memory details with `/usr/bin/time -l`; on Windows, inspect the process's working set in Task Manager or Process Explorer. These measurements are operating-system specific, so compare the same executable, input, and environment and run more than once when the workload is variable.

## Profile heap peaks with Valgrind Massif {#massif}

[`Massif`][valgrind~massif~website]↗{{hi:massif}} samples heap usage over time and records allocation stacks near the peak. On a supported Valgrind platform, run `valgrind --tool=massif --massif-out-file=massif.out ./target/release/my_app`, then inspect the snapshots with `ms_print massif.out`. Massif reports heap usage; it is not a replacement for an operating-system measurement of total process RSS.

## Find heap allocation sites with Heaptrack {#heaptrack}

[`heaptrack`][c~heaptrack~docs]↗{{hi:heaptrack}} records allocation and deallocation events with their call stacks. Run `heaptrack ./target/release/my_app` on Linux, then open the resulting profile with `heaptrack_gui`. Use the allocation timeline and call-site view to find repeated allocations, high-volume sites, or objects retained longer than expected. Profiling adds overhead, so use a representative local build and input rather than measuring production latency under the profiler.

## Profile Heap Memory with `dhat` {#dhat}

{{#badges dhat}}

[`dhat`][c~dhat~docs]↗{{hi:dhat}} is a Rust library for heap profiling and ad-hoc event profiling. Add its global allocator and profiler to a dedicated example or feature so ordinary application builds do not pay the profiling cost. The included example writes `dhat-heap.json`, which can be explored with the DHAT viewer.

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/src/memory_usage_analysis/dhat.rs:example}}
```

## Assert on Heap Allocations in a Test with `dhat` {#dhat-testing}

[![dhat][c~dhat~docs~badge]][c~dhat~docs]{{hi:dhat}}
[![dhat~crates.io][c~dhat~crates.io~badge]][c~dhat~crates.io]
[![dhat~repo][c~dhat~repo~badge]][c~dhat~repo]

`dhat` can also check allocation behavior in a test. Create the profiler in testing mode (`Profiler::builder().testing().build()`), read the counters with `HeapStats::get()`, and use `dhat::assert!` and `dhat::assert_eq!`, which print the profile to `dhat-heap.json` when an assertion fails. Only one profiler can exist per process, so keep all such assertions in a single test (or run them with `--test-threads=1`). Allocation counts can change between compiler versions; assert on stable properties such as "no reallocation" rather than exact byte totals.

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/examples/dhat_assertions.rs:example}}
```

## Find leaks and memory errors {#leaks}

[`Memcheck`][valgrind~memcheck~website]↗{{hi:memcheck}} checks invalid reads and writes and reports unreachable allocations when the process exits. Run `valgrind --tool=memcheck --leak-check=full ./target/debug/my_app` on a supported platform; its instrumentation makes the program much slower, so use a small, representative input.

Rust's Address Sanitizer can find memory errors such as use-after-free and out-of-bounds access in unsafe code. Rust sanitizer support is experimental, requires nightly, and is limited to supported target/toolchain combinations. Follow the current Rust sanitizer documentation for the target-specific invocation rather than copying a C compiler's `-fsanitize=address` flag into a stable Cargo build.

## Examine Memory with Debugging Tools {#debugging}

[`gdb`][c~gdb~docs]↗{{hi:gdb}} and [`lldb`][c~lldb~docs]↗{{hi:lldb}} let you inspect process memory, variables, and pointers while stopped at a breakpoint. A debugger helps explain an object's state, but it does not aggregate allocation counts or replace a heap profiler.

## Use Tracing to Understand Allocation Patterns {#tracing}

[`tracing`][c~tracing~docs]↗{{hi:tracing}} spans can identify which requests or operations create and retain large values. Add fields that describe useful sizes or item counts when those measurements are available; tracing alone does not measure allocated bytes. See [[tracing | Tracing]].

## Related Topics {#related-topics .skip}

- [[memory-management | Memory Management]].
- [[benchmarking | Benchmarking]] for runtime measurements and throughput.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
