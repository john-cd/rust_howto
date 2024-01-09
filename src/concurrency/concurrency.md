# Concurrency

This section covers concurrent programming, specifically parallel programming and async programming.

## Parallelism

- True simultaneous execution of multiple tasks on multiple cores or processors.
- Mechanism: uses operating system threads.
- Important for CPU-heavy computations.
- Often requires explicit management of threads and thread pools.
- Requires careful synchronization to prevent data races (using mechanisms like mutexes or atomics).
- Overhead due to thread creation and switching.

Key constructs in Rust:

- Threads: Independent units of execution that can be spawned using e.g. `std::thread::spawn`.
- Mutexes: Protect shared data from race conditions using e.g. `std::sync::Mutex`.
- Channels: Allow threads to communicate and exchange data using e.g. `std::sync::mpsc`.

Here are the topics we’ll cover:

- [Multithreading⮳](multithreading.md)
- [Message passing⮳](message_passing.md)
- [Shared-state concurrency⮳](shared_state.md)
- [Concurrent data structures⮳](concurrent_data_structures.md)

## Asynchronous programming

- Ability to make progress on multiple tasks, even if they don't execute at the exact same time.
- Mechanism: _cooperative_ multitasking - tasks yield control, allowing other tasks to run.
- Involves context switching on a single thread or, most often, among a few threads (the pool of which is opaquely managed by the async runtime).
- Achieves non-blocking I/O operations to improve responsiveness and efficiency.
- Lower overhead compared to multithreading.
- Multithreaded async programming also requires careful synchronization to prevent data races.

Key constructs in Rust:

- `async` / `await` keywords
- `Future`s

Here are the topics we’ll cover:

- [Async](async.md)
  - [Async and traits](async_traits.md)
  - [Tokio async runtime](tokio.md)
  - [Async channels](async_channels.md)
  - [Streams](streams.md)
  - [Futures crate](futures.md)

## See Also

[Concurrency (rust book)][concurrency-rust-book]⮳

{{#include ../link-refs.md}}
