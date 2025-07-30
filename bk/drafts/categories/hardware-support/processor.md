# Processor

{{#include processor.incl.md}}

## Key Points {#skip}

- Rust generally doesn't require direct CPU manipulation. The compiler and standard library provide good abstractions.
- SIMD and atomic operations are important for performance.
- Direct access to CPU features (like inline [assembly][p~assembly]) is usually only necessary for very specialized or performance-critical code. Avoid it if you can.

## CPU Identification {#skip}

[`cpuid`][c~cpuid~docs]⮳{{hi:cpuid}}: A crate for getting CPU information (vendor, features, etc.)

### Check the Number of Logical cpu Cores {#check-number-of-logical-cpu-cores}

[![num_cpus][c~num_cpus~docs~badge]][c~num_cpus~docs] [![num_cpus~crates.io][c~num_cpus~crates.io~badge]][c~num_cpus~crates.io] [![num_cpus~github][c~num_cpus~github~badge]][c~num_cpus~github] [![num_cpus~lib.rs][c~num_cpus~lib.rs~badge]][c~num_cpus~lib.rs]{{hi:num_cpus}}{{hi:Cpus}}{{hi:Cores}}{{hi:Cpu}} [![cat~hardware-support][cat~hardware-support~badge]][cat~hardware-support]{{hi:Hardware support}}{{hi:Logical cpu cores}}

Shows the number of logical CPU cores{{hi:CPU cores}} in the current machine using [`num_cpus::get`][c~num_cpus::get~docs]{{hi:num_cpus::get}}⮳.

```rust,editable
{{#include ../../../crates/cats/hardware_support/examples/processor/cpu_count.rs:example}}
```

## Low-Level Programming and Optimization {#skip}

### Inline Assembly {#skip}

You can use inline [assembly][p~assembly] in Rust with the asm! macro, but it's generally discouraged unless absolutely necessary for performance reasons. It makes code less portable.

### Compiler Intrinsics {#skip}

Compiler intrinsics are functions that map directly to CPU instructions. They're often used for low-level optimization. Access to intrinsics is usually through `std::arch`.

## SIMD (Single Instruction, Multiple Data) Operations {#simd}

`std::arch`: (Standard library) Provides access to SIMD instructions if the target CPU supports them. This is essential for performance optimization.

[`packed_simd`][c~packed_simd~docs]⮳{{hi:packed_simd}}: A crate for portable SIMD.

## Related Topics {#related-topics}

### Atomic Operations {#skip}

See [[atomics | Atomics]].

### Profiling {#skip}

Profiling tools help you identify CPU-related performance issues.

`cargo flamegraph`, [`perf`][c~perf~docs]⮳{{hi:perf}} (Linux) help you identify performance bottlenecks in your code, which can be related to CPU usage.

See [[development-tools_profiling | Development Tools Profiling]] and [[memory_usage_analysis | Memory Usage Analysis]].

### Concurrency and Multithreading (Related to CPU Utilization) {#skip}

Concurrency and multithreading allow you to utilize multiple CPU cores.

`std::thread`: (Standard library) For creating and managing threads.
[`rayon`][c~rayon~docs]⮳{{hi:rayon}}: A [data parallelism][p~data-parallelism] library that makes it easy to parallelize computations.
[`tokio`][c~tokio~docs]⮳{{hi:tokio}}: An [asynchronous][p~asynchronous] runtime for writing concurrent applications.

See [[concurrency | Concurrency]].

## Memory Model {#skip}

Understanding the CPU's memory model is important for writing correct concurrent code. See [[memory-management | Memory Management]].

### Operating System Interaction {#skip}

System calls are used to interact with the operating system, which in turn interacts with the CPU. See [[os | OS]].

### Embedded Systems Programming {#skip}

In embedded systems, you often have more direct access to CPU features and peripherals. See the [Embedded][p~embedded] Systems section for relevant crates.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[processor: expand NOW](https://github.com/john-cd/rust_howto/issues/399)

- [Nine Rules for SIMD Acceleration of Your Rust Code (Part 1) | Towards Data Science](https://towardsdatascience.com/nine-rules-for-simd-acceleration-of-your-rust-code-part-1-c16fe639ce21/)
- [Towards fearless SIMD, 7 years later - Linebender](https://linebender.org/blog/towards-fearless-simd/)

</div>
