# Concurrency

This section covers concurrent, parallel programming.

{{#include message_passing.incl.md}}

{{#include multithreading.incl.md}}

{{#include parallel.incl.md}}

{{#include threads.incl.md}}

## Parallelism

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

- True simultaneous execution of multiple tasks on multiple cores or processors.
- Mechanism: uses operating system threads.
- Important for CPU-heavy computations.
- Often requires explicit management of threads and thread pools.
- Requires careful synchronization to prevent data races (using mechanisms like Mutexes or atomics).

- Overhead due to thread creation and switching.

Key constructs in Rust:

- Threads are independent units of execution that can be spawned using e.g. `std::thread::spawn`.
- Mutexes e.g. `std::sync::Mutex` protect shared data from race conditions.
- Channels e.g. `std::sync::mpsc` allow threads to communicate and exchange data.

Here are the topics we’ll cover:

- [Multithreading](multithreading.md)
- [Message passing](message_passing.md)
- [Shared-state concurrency](shared_state/index.md)
- [Concurrent data structures](shared_state/concurrent_data_structures.md)

## See Also

[![book-rust-concurrency][book-rust-concurrency-badge]][book-rust-concurrency]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
