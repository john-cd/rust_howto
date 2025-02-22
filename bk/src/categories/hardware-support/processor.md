# Processor

{{#include processor.incl.md}}

## Check the number of logical cpu cores {#check-number-of-logical-cpu-cores}

[![num_cpus][c-num_cpus-badge]][c-num_cpus] [![num_cpus-crates.io][c-num_cpus-crates.io-badge]][c-num_cpus-crates.io] [![num_cpus-github][c-num_cpus-github-badge]][c-num_cpus-github] [![num_cpus-lib.rs][c-num_cpus-lib.rs-badge]][c-num_cpus-lib.rs]{{hi:num_cpus}}{{hi:Cpus}}{{hi:Cores}}{{hi:Cpu}} [![cat-hardware-support][cat-hardware-support-badge]][cat-hardware-support]{{hi:Hardware support}}{{hi:Logical cpu cores}}

Shows the number of logical CPU cores{{hi:CPU cores}} in the current machine using [`num_cpus::get`][c-num_cpus::get]{{hi:num_cpus::get}}⮳.

```rust,editable
{{#include ../../../crates/cats/hardware_support/tests/cpu_count.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[processor: expand (P2)](https://github.com/john-cd/rust_howto/issues/399)

## Key Points

Rust generally doesn't require direct CPU manipulation. The compiler and standard library provide good abstractions.
SIMD and atomic operations are important for performance.
Concurrency and multithreading allow you to utilize multiple CPU cores.
Profiling tools help you identify CPU-related performance issues.
Direct access to CPU features (like inline [assembly][p-assembly]) is usually only necessary for very specialized or performance-critical code. Avoid it if you can.

## SIMD (Single Instruction, Multiple Data)

`std::arch`: (Standard library) Provides access to SIMD instructions if the target CPU supports them. This is essential for performance optimization.

`packed_simd`: A crate for portable SIMD.

## Atomic Operations

`std::sync::atomic`: (Standard library) Provides atomic types for safe concurrent access to data. Essential for multi-threaded programming.

## CPU Identification

`cpuid`: A crate for getting CPU information (vendor, features, etc.).

## Low-Level Programming and Optimization

### Inline Assembly

You can use inline [assembly][p-assembly] in Rust with the asm! macro, but it's generally discouraged unless absolutely necessary for performance reasons. It makes code less portable.

### Compiler Intrinsics

Compiler intrinsics are functions that map directly to CPU instructions. They're often used for low-level optimization. Access to intrinsics is usually through `std::arch`.

### Profiling

`cargo flamegraph`, `perf` (Linux): These tools help you identify performance bottlenecks in your code, which can be related to CPU usage.

### Concurrency and Multithreading (Related to CPU Utilization)

`std::thread`: (Standard library) For creating and managing threads.
`rayon`: A [data parallelism][p-data-parallelism] library that makes it easy to parallelize computations.
`tokio`: An [asynchronous][p-asynchronous] runtime for writing concurrent applications.

### Other Relevant Topics

Memory Model: Understanding the CPU's memory model is important for writing correct concurrent code.
Operating System Interaction: System calls are used to interact with the operating system, which in turn interacts with the CPU.
Embedded Systems Programming: In embedded systems, you often have more direct access to CPU features and peripherals. (See the [Embedded][p-embedded] Systems section for relevant crates.)

</div>
