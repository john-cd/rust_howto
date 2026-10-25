<!-- TODO Reviewed ??-??-2026 -->

# Concurrency

[![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

Concurrency{{hi:Concurrency}} is about coordinating tasks whose lifetimes overlap. Parallelism{{hi:Parallelism}} is the simultaneous execution of work, often across CPU cores. Rust's ownership rules and the `Send` and `Sync` traits help make cross-thread sharing safe; use async tasks for I/O-bound coordination and threads or data-parallel iterators for CPU-bound work. The recipes below focus on operating-system threads and parallel computation.

| Topic                                                       | Use this chapter for                                 | Common building blocks                          |
| ----------------------------------------------------------- | ---------------------------------------------------- | ----------------------------------------------- |
| [Explicit Threads](explicit_threads.md)                     | Spawn, scope, and join operating-system threads      | `std::thread`                                   |
| [Threadpools](threadpool.md)                                | Reuse workers for independent jobs                   | `threadpool`                                    |
| [Data Parallelism](data_parallelism.md)                     | Split CPU-bound iterator work across cores           | `rayon`                                         |
| [Message Passing](message_passing.md)                       | Send owned values between threads                    | `std::sync::mpsc`, `crossbeam-channel`, `flume` |
| [Shared State](shared_state.md)                             | Coordinate access to shared values                   | `Mutex`, `RwLock`                               |
| [Atomics](atomics.md)                                       | Coordinate simple shared values without a mutex      | `std::sync::atomic`, `arc-swap`                 |
| [`crossbeam`](crossbeam.md)                                 | Use scoped threads and lower-level concurrency tools | `crossbeam`                                     |
| [Concurrent Data Structures](concurrent_data_structures.md) | Share maps and queues across threads                 | `dashmap`, `papaya`, `crossbeam-queue`          |
| [`Send` and `Sync`](send_sync.md)                           | Understand safe transfer and sharing between threads | `std::marker`                                   |
| [Actors](_actors.md)                                        | Isolate state behind message-driven workers          | `ractor`, `stakker`                             |

## Explicit Threads

Create, scope, and join threads; use [`std::thread`][c~std::thread~docs]↗ and [`std::thread::scope`][c~std::thread::scope~docs]↗ when borrowing data from the parent scope.

{{#include explicit_threads.incl.md}}

## Threadpools

Reuse a bounded set of workers for queued jobs with [`threadpool`][c~threadpool~docs]↗.

{{#include threadpool.incl.md}}

## Multithreading with the `crossbeam` Crate

Use scoped threads, channels, and other primitives from the [`crossbeam`][c~crossbeam~docs]↗ family.

{{#include crossbeam.incl.md}}

## Message Passing and Channels

Transfer messages between producers and consumers with standard-library or third-party channels.

{{#include message_passing.incl.md}}

## Shared State

Protect shared state with locks or atomics when ownership transfer through a channel is not a fit.

{{#include shared_state.incl.md}}

## Concurrent Data Structures

Choose concurrent maps and queues when many threads need to access the same collection.

{{#include concurrent_data_structures.incl.md}}

## Data Parallelism

Parallelize CPU-bound iterator work with [`rayon`][c~rayon~docs]↗{{hi:rayon}}.

{{#include data_parallelism.incl.md}}

## `Send` and `Sync`

Understand the marker traits that describe values safe to transfer or share between threads.

{{#include send_sync.incl.md}}

## References

- [![Rust concurrency book][book~rust~concurrency~badge]][book~rust~concurrency]↗.
- [Rust Atomics][book~rust-atomics]↗.

## Related Topics

- [[async | Async]].
- [[async_and_blocking | Async and Blocking]].
- [[async_channels | Async Channels]].
- [[concurrent_data_structures | Concurrent Data Structures]].
- [[message_passing | Message Passing]].
- [[explicit_threads | Explicit Threads]].
- [[processor | Processor]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
