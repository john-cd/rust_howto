# Concurrency

This section covers concurrent and parallel programming{{hi:Parallel programming}}.

{{#include multithreading.incl.md}}

{{#include parallel.incl.md}}

{{#include threads.incl.md}}

{{#include message_passing.incl.md}}

## Parallelism

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

- True simultaneous execution of multiple tasks on multiple cores or processors.
- Mechanism: uses operating system threads{{hi:Operating system threads}}.
- Important for CPU-heavy computations{{hi:CPU-heavy computations}}.
- Often requires explicit management of threads and thread pools{{hi:Thread pools}}.
- Requires careful synchronization to prevent data races (using mechanisms like Mutexes or atomics).

- Overhead due to thread creation and switching.

Key constructs in Rust:

- Threads{{hi:Threads}} are independent units of execution that can be spawned using e.g. `std::thread::spawn`{{hi:std::thread::spawn}}.
- Mutexes e.g. `std::sync::Mutex`{{hi:std::sync::Mutex}} protect shared data from race conditions.
- Channels{{hi:Channels}} e.g. `std::sync::mpsc`{{hi:std::sync::mpsc}} allow threads to communicate and exchange data.

Here are the topics we’ll cover:

- [Multithreading][p-multithreading]⮳
- [Message passing][p-message_passing]⮳
- [Shared-state concurrency][p-shared_state]⮳
- [Concurrent data structures][p-concurrent_data_structures]⮳

## See Also

[![book-rust-concurrency][book-rust-concurrency-badge]][book-rust-concurrency]

[p-message_passing]: message_passing.md
[p-shared_state]: shared_state/index.md
[p-concurrent_data_structures]: shared_state/concurrent_data_structures.md
[p-multithreading]: multithreading.md
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: add crossbeam

## Crossbeam

[crossbeam][c-crossbeam-github]{{hi:crossbeam}}

## arc-swap

[![arc-swap][c-arc_swap-badge]][c-arc_swap]{{hi:arc-swap}}
[![arc-swap-crates.io][c-arc_swap-crates.io-badge]][c-arc_swap-crates.io]
[![arc-swap-github][c-arc_swap-github-badge]][c-arc_swap-github]
[![arc-swap-lib.rs][c-arc_swap-lib.rs-badge]][c-arc_swap-lib.rs]

Atomically swappable Arc

</div>
