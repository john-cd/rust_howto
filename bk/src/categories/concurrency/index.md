# Concurrency

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

This section covers concurrent programming, and specifically parallel programming{{hi:Parallel programming}}.

Parallelism implies:

- True simultaneous execution of multiple tasks on multiple cores or processors.
- Mechanism: uses operating system threads{{hi:Operating system threads}}.
- Important for CPU-heavy computations{{hi:CPU-heavy computations}}.
- Often requires explicit management of threads and thread pools{{hi:Thread pools}}.
- Requires careful synchronization to prevent data races (using mechanisms like mutexes or atomics).
- Overhead due to thread creation and switching.

Key constructs in Rust:

- Threads{{hi:Threads}} are independent units of execution that can be spawned using e.g. [`std::thread::spawn`][c-std::thread::spawn]⮳{{hi:std::thread::spawn}}{{hi:std::thread::spawn}}.
- Mutexes e.g. [`std::sync::Mutex`][c-std::sync::Mutex]⮳{{hi:std::sync::Mutex}} protect shared data from race conditions.
- Channels{{hi:Channels}} e.g. [`std::sync::mpsc`][c-std::sync::mpsc::channel]⮳{{hi:std::sync::mpsc}} allow threads to communicate and exchange data.

## Explicit threads

Creating and managing threads, using mutexes and other synchronization primitives.

{{#include explicit_threads.incl.md}}

## Threadpools

Using thread pools for efficient task execution.

{{#include threadpool.incl.md}}

## Multithreading with the `crossbeam` crate

{{#include crossbeam.incl.md}}

## Message passing and channels

Communicating between threads using channels.

{{#include message_passing.incl.md}}

## Shared state

Shared Memory. Using atomic types for thread-safe data access.

{{#include shared_state.incl.md}}

## Concurrent data structures

{{#include concurrent_data_structures.incl.md}}

## Data parallelism

Using iterators in parallel with crates like `rayon`.

{{#include data_parallelism.incl.md}}

## `Send` and `Sync`

{{#include send_sync.incl.md}}

## Related Topics

- [[async | Async]].
- [[async_and_blocking | Async and Blocking]].
- [[async_channels | Async Channels]].
- [[concurrent_data_structures | Concurrent Data Structures]].
- [[message_passing | Message Passing]].
- [[explicit_threads | Explicit Threads]].
- [[processor | Processor]].

## References

- [![Rust concurrency book][book-rust-concurrency-badge]][book-rust-concurrency]⮳.
- [Rust Atomics][book-rust-atomics]⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[concurrency: add somewhere ](https://github.com/john-cd/rust_howto/issues/263)
</div>
