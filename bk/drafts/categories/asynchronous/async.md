# Async

{{#include async.incl.md}}

Asynchronous programming{{hi:Asynchronous programming}}, or async{{hi:async}} for short, is a concurrent{{hi:Concurrency}} programming model supported by an increasing number of programming languages. It lets you run a large number of concurrent tasks, while preserving much of the look and feel of ordinary synchronous programming, through the async/await{{hi:async/await}} syntax. It helps you deal with events independently of the main program flow, using techniques like futures{{hi:Futures}}, promises{{hi:Promises}}, waiting, or eventing.

- Ability to make progress on multiple tasks, even if they don't execute at the exact same time.
- Mechanism: _cooperative_ multitasking - tasks yield control, allowing other tasks to run.
- Involves context switching on a single thread or, most often, among a few threads (the pool of which is opaquely managed by the async runtime).
- Achieves non-blocking I/O operations{{hi:Non-blocking I/O operations}} to improve responsiveness and efficiency.
- Lower overhead compared to multithreading.
- Multi-threaded async programming also requires careful synchronization to prevent data races.

Key constructs in Rust:

- [`async`][book~rust-reference~async]{{hi:async}}⮳ / [`await`][book~rust-reference~await]{{hi:await}}⮳ keywords.
- [`std::future::Future`][c~std::future::Future~docs]{{hi:std::future::Future}}⮳.

## Basic Example {#basic-example}

[![std][c~std~docs~badge]][c~std~docs] [![tokio][c~tokio~docs~badge]][c~tokio~docs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async/async1.rs:example}}
```

As any form of cooperative multitasking{{hi:Cooperative multitasking}}, a future{{hi:Futures}} that spends a long time without reaching an [`await`][book~rust-reference~await]{{hi:await}}⮳ "blocks the thread", which may prevent other tasks from running.

## Differences with Other Languages {#diff-other-languages}

[![std][c~std~docs~badge]][c~std~docs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

Rust's implementation of [`async`][book~rust-reference~async]{{hi:async}}⮳ differs from most languages in a few ways:

- Rust's [`async`][book~rust-reference~async]{{hi:async}}⮳ operations are lazy. Futures{{hi:Futures}} are inert in Rust and only make progress only when polled. The executor calls the [`std::task::Poll`][c~std::task::Poll~docs]{{hi:std::task::Poll}}⮳ method repeatedly to execute futures.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/async/async2.rs:example}}
```

- Dropping a future stops it from making further progress.
- Async is zero-cost in Rust. You can use [`async`][book~rust-reference~async]{{hi:async}}⮳ without heap allocations{{hi:Heap allocations}} and dynamic dispatch. This also lets you use async in constrained environments, such as embedded systems.
- No built-in runtime is provided by Rust itself. Instead, runtimes are provided by community-maintained [crates][p~crates].
- Both single- and multi-threaded runtimes{{hi:Multi-threaded runtimes}} are available.

## Which Crate Provides What? {#which-crate-provides-what}

[![std][c~std~docs~badge]][c~std~docs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

- The [`async`][book~rust-reference~async]{{hi:async}}⮳ / [`await`][book~rust-reference~await]{{hi:await}}⮳ syntactic sugar is supported directly by the Rust compiler.
- The most fundamental [traits][p~traits], types, and [functions][p~functions], such as the [`std::future::Future`][c~std::future::Future~docs]{{hi:std::future::Future}}⮳ trait, are provided by the standard library.
- Many utility types, [macros][p~macros] and [functions][p~functions] are provided by the [`[futures`][c~futures~docs]{{hi:futures}}⮳ crate. They can be used in any async Rust application.
- Execution of async code, IO and task spawning are provided by "async runtimes", such as [`tokio`][c~tokio~docs]{{hi:tokio}}⮳ and [`async_std`][c~async_std~docs]{{hi:async-std}}⮳. Most async applications, and some async crates, depend on a specific runtime.

## Async Runtimes {#async-runtimes}

[![async-std][c~async_std~docs~badge]][c~async_std~docs]{{hi:async-std}} [![smol][c~smol~docs~badge]][c~smol~docs]{{hi:smol}} [![embassy][c~embassy~docs~badge]][c~embassy~docs]{{hi:embassy}} [![mio][c~mio~docs~badge]][c~mio~docs]{{hi:mio}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

In most cases, prefer the [`tokio`][p~tokio]{{hi:tokio}} runtime - see [The State of Async Rust: Runtimes][blog~state-of-async-rust]⮳.

Alternatives to the [Tokio][p~tokio] async ecosystem include:

- [![async-std][c~async_std~docs~badge]][c~async_std~docs]{{hi:async-std}} [![async_std~crates.io][c~async_std~crates.io~badge]][c~async_std~crates.io]⮳: async version of the Rust standard library. No longer maintained?
- [![smol][c~smol~docs~badge]][c~smol~docs]{{hi:smol}} [Smol][c~smol~crates.io]⮳.
- [![embassy][c~embassy~docs~badge]][c~embassy~docs]{{hi:embassy}} [Embassy][c~embassy~website]⮳ [![embassy~github][c~embassy~github~badge]][c~embassy~github] for embedded systems.
- [![mio][c~mio~docs~badge]][c~mio~docs]{{hi:mio}} [Mio][c~mio~crates.io]⮳ is a fast, low-level I/O library for Rust focusing on non-blocking APIs and event notification for building high performance I/O apps with as little overhead as possible over the OS abstractions. It is part of the Tokio ecosystem.

## Related Topics {#skip}

- [[concurrency | Concurrency]].

## References

- [Are we async yet?][are-we-async-yet?~website]⮳.
- [Asynchronous Programming in Rust (book)][book~asynchronous-programming-in-rust]⮳.
- [Communicating between sync and async code][communicating-between-sync-and-async-code~website]⮳.
- [async-what-is-blocking][blog~async-what-is-blocking]⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; review in depth](https://github.com/john-cd/rust_howto/issues/633)
</div>
