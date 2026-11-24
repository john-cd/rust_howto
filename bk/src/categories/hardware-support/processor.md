<!-- TODO Reviewed ??-??-2026. -->

# Processor

{{#include processor.incl.md}}

## Key Points {#key-points .skip}

- Rust's standard library and compiler handle most processor details. Start with portable APIs and ordinary optimized Rust.
- A program's target architecture and enabled CPU features determine which instructions it may use. A binary built for one processor may not run on another.
- Use architecture-specific intrinsics only when measurement shows a need and a portable implementation is not sufficient.
- SIMD processes several values with one instruction; atomics coordinate concurrent access. They solve different problems and both require workload-specific reasoning.

## CPU Identification {#cpu-identification .skip}

Use [`std::thread::available_parallelism`][c~std::thread~docs]↗{{hi:std::thread::available_parallelism}} when an application needs a hint for sizing a worker pool. It reports the parallelism available to the process when the operating system can provide that information; it is not a promise that many workers will improve performance.

For vendor, model, cache, or instruction-set inventory on x86 and x86-64, [`raw-cpuid`][c~raw-cpuid~docs]↗{{hi:raw-cpuid}} can read CPUID data. It is architecture-specific and does not replace the runtime feature-detection check required before calling an architecture-specific intrinsic. CPU identification is not required for ordinary application logic.

### Check the Number of Logical CPU Cores {#check-number-of-logical-cpu-cores}

{{#badges num_cpus}} {{hi:Cpus}} {{hi:Cores}} {{hi:Cpu}} {{#categories hardware-support="Hardware support"}} {{hi:Logical cpu cores}}

Shows the number of logical CPU cores{{hi:CPU cores}} in the current machine using [`num_cpus::get`][c~num_cpus::get~docs]↗{{hi:num_cpus::get}}.

```rust,editable
{{#include ../../../crates/cats/hardware_support/examples/processor/cpu_count.rs:example}}
```

When distributing one binary to several CPUs, keep a portable path and select a specialized routine only on CPUs that support the required instructions.

## Low-Level Programming and Optimization {#low-level .skip}

### Inline Assembly {#inline-assembly .skip}

Rust supports inline [assembly](../development-tools_profiling/assembly.md) through the `asm!` macro. Assembly is architecture-specific and bypasses many compiler guarantees, so use it only when a supported intrinsic or library cannot express a measured requirement. Keep it in a small, documented boundary and provide a portable implementation where practical.

### Compiler Intrinsics {#compiler-intrinsics .skip}

Compiler intrinsics map to operations supported by a processor architecture. Rust exposes many of them through [`std::arch`][c~std::arch~docs]↗{{hi:std::arch}}. Their availability depends on the target architecture and, for some instructions, on the CPU that runs the program.

There are two common ways to enable a feature:

- Compile the whole program for a known processor or feature set. This can produce fast code, but the binary requires a compatible CPU. Treat `-C target-cpu=native` as a local or deployment-specific choice, not a portable release default.
- Compile an optimized function separately and select it at runtime after checking that the running CPU supports its required features. Keep a baseline fallback for other processors. The detection check, architecture configuration, and function's `#[target_feature]` annotation must agree; calling an unsupported intrinsic can cause undefined behavior.

The `std::arch` documentation describes the architecture-specific modules and runtime feature-detection macros. Put target-specific code behind `#[cfg]` so other architectures still compile.

## SIMD (Single Instruction, Multiple Data) Operations {#simd}

SIMD can speed up data-parallel operations such as processing pixels or numeric arrays, but it is not automatically faster. Measure representative inputs, account for the cost of dispatch and data conversion, and keep a scalar or otherwise portable path when the deployment target is not fixed.

[`std::arch`][c~std::arch~docs]↗{{hi:std::arch}} provides vendor-specific intrinsics. These APIs are not portable across architectures, and some intrinsics require runtime feature detection before use.

[`core::simd`][c~core::simd~docs]↗{{hi:core::simd}} is the portable SIMD API. It is currently a nightly-only experimental API, so stable applications should check its current status before depending on it.

### Compute a Dot Product with `std::arch` and `wide` {#simd-dot-product}

[![std::arch][c~std::arch~docs~badge]][c~std::arch~docs]{{hi:std::arch}} {{#badges wide}} {{hi:SIMD}} {{hi:Intrinsics}} {{hi:Runtime feature detection}} {{#categories hardware-support="Hardware support"}}

The example computes a dot product three ways:

- With `std::arch` intrinsics (SSE2 and AVX2 on x86-64), selected at run time with `is_x86_feature_detected!`. The `unsafe` code sits behind a small safe function that documents its `SAFETY` invariants.
- With the scalar loop, which is the fallback on other targets and the reference in the tests.
- With the portable [`wide`][c~wide~docs]↗{{hi:wide}} crate, which works on stable Rust on any target.

All three implementations use the common prefix when the input lengths differ. The vectorized versions truncate both inputs before splitting them into vector-sized chunks, so the scalar tail covers the same positions.

```rust,editable
{{#include ../../../crates/cats/hardware_support/examples/simd/simd_arch.rs:example}}
```

When SIMD pays off:

- SIMD helps on long, contiguous numeric loops. Measure with [Criterion](../development-tools_profiling/benchmarking.md) before and after.
- SIMD changes the order of floating-point additions, so cancellation can produce a large relative difference from the scalar result. The example's fixtures use small integers, which `f32` represents exactly.
- Handle `NaN` and infinity deliberately; lane-wise operations propagate them without any check.
- WebAssembly has its own `simd128` target feature, enabled at compile time rather than detected at run time.

## Related Topics {#related-topics .skip}

### Atomic Operations {#atomic-operations .skip}

See [Atomics](../concurrency/atomics.md).

### Profiling {#profiling .skip}

Profiling tools help identify CPU-related performance issues.

`perf` on Linux and tools such as `cargo flamegraph` help identify performance bottlenecks. See [Profiling](../development-tools_profiling/index.md).

For memory-related bottlenecks, see [Memory Usage Analysis](../development-tools_profiling/memory_usage_analysis.md).

### Concurrency and Multithreading (Related to CPU Utilization) {#concurrency-and-multithreading .skip}

Concurrency and multithreading can use multiple CPU cores when the work can be divided safely.

[`std::thread`][c~std::thread~docs]↗ creates and manages operating-system threads. [`rayon`][c~rayon~docs]↗ simplifies [data-parallel work](../concurrency/data_parallelism.md); [`tokio`][c~tokio~docs]↗ is an [asynchronous runtime](../asynchronous/index.md) for I/O-bound tasks.

See [Concurrency](../concurrency/index.md).

## Memory Model {#memory-model .skip}

Understanding the CPU memory model is important for correct concurrent code. See [Atomics](../concurrency/atomics.md) and [Memory Management](../memory-management/index.md).

### Operating System Interaction {#operating-system-interaction .skip}

System calls let programs request services from the operating system. For platform-specific APIs, see the Rust documentation for [`std::os::unix`](https://doc.rust-lang.org/std/os/unix/index.html) and [`std::os::windows`](https://doc.rust-lang.org/std/os/windows/index.html).

### Embedded Systems Programming {#embedded-systems-programming .skip}

Embedded systems expose CPU features and peripherals through board or chip support crates. Start with the [Embedded Rust Book](https://docs.rust-embedded.org/book/).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

- [Nine Rules for SIMD Acceleration of Your Rust Code (Part 1)][blog~towardsdatascience-nine-rules-for-simd-acceleration-of-your-rust-code-part-1]↗.
- [Towards fearless SIMD, 7 years later][blog~towards-fearless-simd]↗ - Linebender.

</div>
