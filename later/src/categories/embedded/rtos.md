<!-- TODO Reviewed ??-??-2026 -->
# Real-Time Operating Systems (RTOS)

{{#include rtos.incl.md}}

An RTOS or embedded concurrency framework coordinates work that must respond to timers, interrupts, or peripheral events. The important choice is the execution model: whether tasks are preemptive or cooperative, how priorities and shared data are handled, and what memory and timing costs the design permits. A framework cannot make a system meet deadlines automatically; measure execution time and account for interrupt and bus contention.

## Execution Models {#execution-models}

| Framework | Model | Useful when |
| --- | --- | --- |
| [`RTIC`][c~rtic~docs]↗{{hi:rtic}} | Statically described tasks and resources, scheduled using supported hardware interrupt controllers; current RTIC releases also support async software tasks. | Priorities and shared resources are known at build time and bounded interrupt latency matters. |
| [`Embassy`][c~embassy~docs]↗{{hi:embassy}} | Cooperative `async` tasks run on an executor, with HALs and drivers for supported chips. | Many I/O operations wait on timers or peripherals and can yield without dedicating a thread and stack to each operation. |
| [FreeRTOS Rust bindings][c~freertos~docs]↗{{hi:FreeRTOS}} | Rust bindings to the FreeRTOS kernel's task, queue, timer, and synchronization model. | An existing product or vendor ecosystem already depends on FreeRTOS and its APIs. |

These are not interchangeable APIs. RTIC's statically analyzed resource model differs from a general-purpose thread-based kernel. Embassy tasks make progress when they yield at an `.await`; a long non-yielding future can delay other work on the same executor. FreeRTOS tasks have separate stacks and kernel scheduling behavior, so budget stack space and synchronization explicitly.

### RTIC {#rtic}

RTIC models tasks, priorities, and shared resources in the application. On supported single-core targets, it uses the interrupt controller and a resource-ceiling protocol to schedule work and guard shared state. Its compile-time model is useful when the task/resource relationships are stable and you need explicit priority reasoning. Confirm the current RTIC version and architecture support in the [RTIC book][c~rtic~docs]↗.

### Embassy {#embassy}

Embassy combines async executors with HALs and peripheral drivers for supported chips. Async tasks can wait for timers and I/O without holding a dedicated thread stack, but cooperative tasks must reach `.await` points so other tasks can progress. Check the target's HAL and executor documentation before assuming that a peripheral supports async operations.

### FreeRTOS {#freertos}

FreeRTOS is a conventional RTOS with kernel tasks, priorities, queues, timers, and synchronization primitives. Rust bindings can make an existing FreeRTOS system accessible from Rust, but the application still needs a compatible port, kernel configuration, interrupt setup, and clear ownership at the FFI boundary. Test the behavior and stack budgets on the actual target.

## Plan for Timing and Memory

Before choosing a framework, list the deadlines, event sources, and maximum time each operation can run. Include interrupt handlers, critical sections, flash writes, bus transactions, and logging in the timing budget. For each task, bound its stack and queue use, define what happens when input arrives faster than it can be processed, and keep interrupt handlers short.

Review priority inversion and lock ordering when tasks share state. Do not wait on a blocking peripheral operation from an async executor if it can stall unrelated tasks. Verify the selected framework's supported architecture, timers, HAL integration, and panic behavior for the exact chip.

## Bare Metal Is a Different Choice

Bare-metal firmware can use interrupts and a main loop without an RTOS. A small program with a few event sources may be simpler that way. A hardware abstraction layer or `no_std` runtime is not itself an RTOS; add a scheduler or executor only when its task model solves a concrete coordination problem.

## Related Topics {#related-topics .skip}

- [Hardware Abstraction Layers](hals.md) provide peripheral interfaces to tasks.
- [Peripheral Access Crates](pacs.md) expose chip-specific registers and interrupt names.
- Concurrency design and [no-std development](../no-std/index.md) provide related Rust patterns.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1202)
</div>
