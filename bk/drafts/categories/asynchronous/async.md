<!-- TODO Reviewed ??-??-2026 -->

# Async

{{#include async.incl.md}}

Asynchronous programming{{hi:Asynchronous programming}}, or async{{hi:async}} for short, lets a program make progress on other work while an operation waits, often for input/output. Rust's `async` / `await`{{hi:async/await}} syntax expresses this work as futures{{hi:Futures}}. An executor polls those futures and schedules other ready tasks when one is waiting.

Async is cooperative: a task gives the executor a chance to run other work when it reaches an `.await` that cannot complete yet. Async can handle many waiting tasks with a small number of threads, but it does not make blocking operations non-blocking or guarantee parallel execution. Async tasks running on multiple threads still need synchronization when they share mutable state.

Key constructs in Rust:

- [`async`][book~rust-reference~async]↗{{hi:async}} / [`await`][book~rust-reference~await]↗{{hi:await}} keywords.
- [`std::future::Future`][c~std::future::Future~docs]↗{{hi:std::future::Future}}.

## Basic Example {#basic-example}

[![std][c~std~docs~badge]][c~std~docs] [![tokio][c~tokio~docs~badge]][c~tokio~docs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async/async1.rs:example}}
```

Like other cooperative multitasking{{hi:Cooperative multitasking}} systems, Rust async cannot preempt a task that runs for a long time without reaching an [`await`][book~rust-reference~await]↗{{hi:await}}. Such work blocks the executor thread and can prevent other tasks from running.

## Futures Are Lazy {#diff-other-languages}

[![std][c~std~docs~badge]][c~std~docs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

Calling an `async fn` creates a future; it does not run the function body. The future makes progress only when an executor polls it, typically through `.await` or by spawning it on a runtime.

Dropping a future stops that future from making further progress. This is one reason to keep track of which task owns a future and whether it has been spawned independently.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async/async2.rs:example}}
```

An `async fn` is compiled to a state machine that implements [`std::future::Future`][c~std::future::Future~docs]↗{{hi:std::future::Future}}. The future can be stored inline without a heap allocation or dynamic dispatch; boxing, trait objects, and runtime task storage may add them. The Rust standard library defines the future interfaces, but does not provide an executor, asynchronous I/O, or a runtime.

## Async Runtime and Crates {#which-crate-provides-what}

[![std][c~std~docs~badge]][c~std~docs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

- The compiler implements `async` / `await`, and the standard library provides the `Future` trait and task interfaces.
- The [`futures`][c~futures~docs]↗{{hi:futures}} crate offers utility traits, types, and functions that are not tied to one runtime.
- A runtime polls futures and commonly supplies task spawning, timers, and asynchronous I/O. Runtime APIs and I/O types can be runtime-specific, so check what a library requires before choosing one.

## Spawn Tasks {#spawn-tasks}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

Awaiting a future runs it as part of the current task. A runtime can also spawn a task so it can make progress independently while the parent task does other work. In Tokio, `spawn` returns a handle that can be awaited to retrieve the task's result.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async/async3.rs:example}}
```

## Choose a Runtime {#async-runtimes}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}} [![async-std][c~async-std~docs~badge]][c~async-std~docs]{{hi:async-std}} [![smol][c~smol~docs~badge]][c~smol~docs]{{hi:smol}} [![embassy][c~embassy~docs~badge]][c~embassy~docs]{{hi:embassy}} [![mio][c~mio~docs~badge]][c~mio~docs]{{hi:mio}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

Choose a runtime based on the libraries and platform your application needs. Tokio has a broad ecosystem and is a common default for general-purpose services; start with the [Tokio tutorial][c~tokio~tutorial~docs]↗. For smaller applications, [`smol`][c~smol~docs]↗{{hi:smol}} is a compact alternative. [`Embassy`][c~embassy~docs]↗{{hi:Embassy}} targets embedded systems. See [The State of Async Rust: Runtimes][blog~state-of-async-rust]↗ for additional context.

Other relevant projects include:

- [![async-std][c~async-std~docs~badge]][c~async-std~docs]{{hi:async-std}} [![async-std~crates.io][c~async-std~crates.io~badge]][c~async-std~crates.io]↗: the maintainers [announced the project's discontinuation][async-std~discontinued] and recommend [`smol`][c~smol~docs]↗ as an alternative. Check each dependency's runtime support when migrating.
- [![mio][c~mio~docs~badge]][c~mio~docs]{{hi:mio}} [Mio][c~mio~crates.io]↗ is a low-level, non-blocking I/O and event-notification library. It exposes readiness events; it does not execute futures.

### Execute Tasks with `smol` {#smol}

This example runs two asynchronous tasks and awaits both results:

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/smol.rs:example}}
```

### Poll for I/O Events with `mio` {#mio}

[`mio`][c~mio~docs]↗{{hi:mio}} provides low-level polling for non-blocking I/O. This example uses a `Waker` to wake a `Poll` instance and handle the resulting event; an application supplies its own event loop and future executor.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/mio.rs:example}}
```

## References {#references .skip}

- [Are we async yet?][are-we-async-yet?~website]↗.
- [Asynchronous Programming in Rust (book)][book~asynchronous-programming-in-rust]↗.
- [Communicating between sync and async code][communicating-between-sync-and-async-code~website]↗.
- [async-what-is-blocking][blog~async-what-is-blocking]↗.

## Related Topics {#related-topics .skip}

- [[concurrency | Concurrency]].
- [[network-programming | Network Programming]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
