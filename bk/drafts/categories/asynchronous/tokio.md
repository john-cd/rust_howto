<!-- TODO Reviewed ??-??-2026 -->

# Tokio

{{#include tokio.incl.md}}

Tokio is an asynchronous runtime for Rust. It schedules tasks and provides asynchronous I/O, timers, synchronization primitives, and utilities for coordinating work. An `async fn` creates a future; it runs only when an executor polls it inside a runtime that supplies any required services.

## Runtime Basics {#basics}

[![tokio~website][c~tokio~website~badge]][c~tokio~website] [![tokio][c~tokio~docs~badge]][c~tokio~docs] [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io] [![tokio~repo][c~tokio~repo~badge]][c~tokio~repo] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs]{{hi:tokio}}{{hi:Async}}{{hi:Futures}}{{hi:Io}}{{hi:Non-blocking}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

For an application whose entry point can be asynchronous, use `#[tokio::main]` to create and run a runtime. For a synchronous application, create or own a runtime at the boundary and call `Runtime::block_on` there. Avoid creating a runtime for each operation, and do not call `block_on` from inside an async task.

Tokio offers a multi-threaded runtime for general applications and a current-thread runtime for applications that need a single executor thread. The current-thread example shows how to build that runtime directly:

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/tokio/tokio2.rs:example}}
```

If `tokio` is configured with default features disabled, enable only the runtime, macros, I/O, timer, synchronization, and signal features that the application uses. Runtime setup is described in the [Tokio tutorial][c~tokio~tutorial~docs]↗.

## Join Futures on One Task {#join}

The `tokio::join!` macro polls several futures as part of the current task and waits for all of them. This is concurrency, not parallel execution: the branches do not run at the same time on separate threads. A synchronous blocking call in one branch prevents the task from polling the others. Use `try_join!` when an error should stop waiting for the remaining branches.

Use `select!` when the task should continue with whichever future completes first, such as a request or a shutdown signal. Dropping the losing future cancels it; check that the operation is safe to cancel before using this pattern.

## Spawn Independent Tasks {#spawning}

Use `tokio::spawn` when work should be scheduled as an independent task. It returns a `JoinHandle`; awaiting it yields the task's result or a `JoinError` if the task panicked or was cancelled. Await handles when results or failures matter. A spawned task must own its captured data, and the multi-threaded runtime requires spawned futures to be `Send`.

Tasks are lightweight, but spawning unbounded work can still exhaust memory or overload a service. Limit concurrency and queue capacity when accepting work faster than it can be processed.

## Run Tasks That Are Not `Send` {#localset}

A future that holds thread-bound state such as `Rc<T>` cannot be moved between runtime threads. `LocalSet` runs such futures on one thread. Prefer ordinary `Send` tasks when possible; reserve a local task set for APIs that require thread affinity.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/tokio/tokio21.rs:example}}
```

## Use Tokio I/O, Timers, and Cancellation {#io}

Prefer Tokio's asynchronous I/O APIs for sockets, files, and timers so a waiting operation yields instead of occupying a runtime worker. Add timeouts and cancellation paths around operations that can wait indefinitely. Use `tokio::select!` to combine an operation with a deadline or shutdown signal, and propagate I/O and task errors instead of discarding them.

CPU-heavy work and blocking APIs need a different execution boundary. See [Mixing Async and Blocking Code][ex~asynchronous~call-async-from-blocking] for `spawn_blocking`, Rayon, dedicated threads, and calls from synchronous code.

## Graceful Shutdown {#graceful-shutdown}

A graceful shutdown stops accepting new work, signals active tasks, gives them an opportunity to finish or clean up, and then exits within a defined deadline. Make each long-running task observe the shutdown signal. Dropping a runtime is not a substitute for coordinating cleanup.

The following example uses `tokio-graceful-shutdown` to organize nested services and handle operating-system shutdown signals:

```rust,editable,noplayground
{{#include ../../../crates/cats/asynchronous/examples/tokio/tokio_graceful_shutdown.rs:example}}
```

## Useful Links {#useful-links .skip}

- [`tokio.rs`][c~tokio~website]↗.
- [Tokio examples][c~tokio~examples~docs]↗.
- [Tokio glossary][c~tokio~glossary~website]↗.
- [Tokio tutorial][c~tokio~tutorial~docs]↗.
- [Mini-Redis example][c~tokio~mini-redis~repo]↗.
- [Rust Tokio template][rust-tokio-template~repo]↗.

## Related Topics {#related-topics .skip}

- [[async_and_blocking | Mixing Async and Blocking Code]].
- [[async_channels | Channels for Use in Async Code]].
- [[concurrency | Concurrency]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  tokio: review
</div>
