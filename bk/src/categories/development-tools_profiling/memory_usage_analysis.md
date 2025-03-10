# Memory usage analysis

{{#include memory_usage_analysis.incl.md}}

## Profile heap memory {#dhat}

[![dhat][c-dhat-badge]][c-dhat]{{hi:dhat}}
[![dhat-crates.io][c-dhat-crates.io-badge]][c-dhat-crates.io]
[![dhat-github][c-dhat-github-badge]][c-dhat-github]
[![dhat-lib.rs][c-dhat-lib.rs-badge]][c-dhat-lib.rs]

[`dhat`][c-dhat]⮳{{hi:dhat}} is a library for heap profiling and ad-hoc profiling with DHAT.

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/src/dhat.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[memory: write](https://github.com/john-cd/rust_howto/issues/336)

## Memory Profiling (Allocation Tracking) {#skip}

[`valgrind`][c-valgrind]⮳{{hi:valgrind}} (with `massif` or `memcheck`): While not Rust-specific, Valgrind is a very common and powerful memory profiler. You'd run your Rust program under Valgrind.
[`heaptrack`][c-heaptrack]⮳{{hi:heaptrack}}: A heap profiler that can track memory allocations.

## Leak Detection {#skip}

[`valgrind`][c-valgrind]⮳{{hi:valgrind}} (with `memcheck`): Excellent for detecting memory leaks.
Address Sanitizer (`ASan`): Often integrated into the compiler; can detect memory leaks and other memory errors. Enable it with compiler flags (e.g., `-fsanitize=address`).

## Memory Usage Analysis (for specific regions) {#skip}

[`measure_time`][c-measure_time]⮳{{hi:measure_time}}: While not strictly a memory profiler, it can help you measure the execution time of code blocks, which can be useful when investigating memory-related performance issues.

## Debugging Tools (for examining memory) {#skip}

[`gdb`][c-gdb]⮳{{hi:gdb}}, [`lldb`][c-lldb]⮳{{hi:lldb}}: Debuggers can be used to inspect memory, examine variables, and track allocations.

Tracing (for understanding allocation patterns):

[`tracing`][c-tracing]⮳{{hi:tracing}}: While not a direct memory profiler, [tracing][p-tracing] can help you understand the flow of your program and identify areas where excessive allocations might be occurring.

## Heap Inspection Tools {#skip}

`heapdump`: A tool for creating heap dumps of running processes, which can then be analyzed with other tools.
</div>
