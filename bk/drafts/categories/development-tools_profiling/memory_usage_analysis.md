# Memory Usage Analysis

{{#include memory_usage_analysis.incl.md}}

## Profile Memory and Track Allocations {profiling}

- While not Rust-specific, [`valgrind`][c~valgrind~docs]⮳{{hi:valgrind}} (with `massif` or `memcheck`) is a very common and powerful memory profiler. You'd run your Rust program under Valgrind.
- [`heaptrack`][c~heaptrack~docs]⮳{{hi:heaptrack}} is a heap profiler that can track memory allocations.

## Profile Heap Memory with `dhat` {#dhat}

[![dhat][c~dhat~docs~badge]][c~dhat~docs]{{hi:dhat}}
[![dhat~crates.io][c~dhat~crates.io~badge]][c~dhat~crates.io]
[![dhat~github][c~dhat~github~badge]][c~dhat~github]
[![dhat~lib.rs][c~dhat~lib.rs~badge]][c~dhat~lib.rs]

[`dhat`][c~dhat~docs]⮳{{hi:dhat}} is a library for heap profiling and ad-hoc profiling with DHAT.

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/src/memory_usage_analysis/dhat.rs:example}}
```

## Leak Detection {#leaks}

- [`valgrind`][c~valgrind~docs]⮳{{hi:valgrind}} (with `memcheck`) is excellent for detecting memory leaks.
- Address Sanitizer (`ASan`) can detect memory leaks and other memory errors. Enable it with compiler flags (e.g., `-fsanitize=address`).

## Examine Memory with Debugging Tools {#debugging}

[`gdb`][c~gdb~docs]⮳{{hi:gdb}}, [`lldb`][c~lldb~docs]⮳{{hi:lldb}} debuggers can be used to inspect memory, examine variables, and track allocations.

## Use Tracing to Understand Allocation Patterns {#tracing}

[`tracing`][c~tracing~docs]⮳{{hi:tracing}} can help you understand the flow of your program and identify areas where excessive allocations might be occurring.
See [[tracing | Tracing]].

## Execution Time Measurement {#timing}

[`measure_time`][c~measure_time~docs]⮳{{hi:measure_time}} can help you measure the execution time of code blocks, which can be useful when investigating memory-related performance issues.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[memory: write / organize](https://github.com/john-cd/rust_howto/issues/336)
</div>
