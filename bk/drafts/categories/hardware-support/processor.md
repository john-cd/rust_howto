<!-- TODO Reviewed ??-??-2026 -->

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

### Check the Number of Logical cpu Cores {#check-number-of-logical-cpu-cores}

{{#badges num_cpus}} {{hi:Cpus}} {{hi:Cores}} {{hi:Cpu}} {{#categories hardware-support="Hardware support"}} {{hi:Logical cpu cores}}

Shows the number of logical CPU cores{{hi:CPU cores}} in the current machine using [`num_cpus::get`][c~num_cpus::get~docs]↗{{hi:num_cpus::get}}.

```rust,editable
{{#include ../../../crates/cats/hardware_support/examples/processor/cpu_count.rs:example}}
```

## Low-Level Programming and Optimization {#low-level .skip}

### Inline Assembly {#inline-assembly .skip}

Rust supports inline [assembly][p~assembly] through the `asm!` macro. Assembly is architecture-specific and bypasses many compiler guarantees, so use it only when a supported intrinsic or library cannot express a measured requirement. Keep it in a small, documented boundary and provide a portable implementation where practical.

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

- SIMD helps on long, contiguous numeric loops. Measure with [`criterion`][c~criterion~docs]↗{{hi:criterion}} before and after.
- SIMD changes the order of floating-point additions, so results can differ from the scalar version in the last bits. The example's fixtures use small integers, which `f32` represents exactly.
- Handle `NaN` and infinity deliberately; lane-wise operations propagate them without any check.
- WebAssembly has its own `simd128` target feature, enabled at compile time rather than detected at run time.

## Related Topics {#related-topics .skip}

### Atomic Operations {#atomic-operations .skip}

See [[atomics | Atomics]].

### Profiling {#profiling .skip}

Profiling tools help identify CPU-related performance issues.

`cargo flamegraph`, [`perf`][c~perf~docs]↗{{hi:perf}} (Linux) help identify performance bottlenecks in your code, which can be related to CPU usage.

See [[development-tools_profiling | Development Tools Profiling]] and [[memory_usage_analysis | Memory Usage Analysis]].

### Concurrency and Multithreading (Related to CPU Utilization) {#concurrency-and-multithreading .skip}

Concurrency and multithreading allow the uses of multiple CPU cores.

`std::thread`: (Standard library) For creating and managing threads.
[`rayon`][c~rayon~docs]↗{{hi:rayon}}: A [data parallelism][p~data-parallelism] library that makes it easy to parallelize computations.
[`tokio`][c~tokio~docs]↗{{hi:tokio}}: An [asynchronous][p~asynchronous] runtime for writing concurrent applications.

See [[concurrency | Concurrency]].

## Memory Model {#memory-model .skip}

Understanding the CPU's memory model is important for writing correct concurrent code. See [[memory-management | Memory Management]].

### Operating System Interaction {#operating-system-interaction .skip}

System calls are used to interact with the operating system, which in turn interacts with the CPU. See [[os | OS]].

### Embedded Systems Programming {#embedded-systems-programming .skip}

In embedded systems, we often have more direct access to CPU features and peripherals. See the [Embedded][p~embedded] Systems section for relevant crates.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
processor: expand

- [Nine Rules for SIMD Acceleration of Your Rust Code (Part 1)][blog~towardsdatascience-nine-rules-for-simd-acceleration-of-your-rust-code-part-1]↗.
- [Towards fearless SIMD, 7 years later][blog~towards-fearless-simd]↗ - Linebender.

</div>
