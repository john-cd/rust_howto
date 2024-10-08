# Asynchronous programming

Asynchronous programming, or {{i:async}} for short, is a concurrent programming model supported by an increasing number of programming languages. It lets you run a large number of {{i:concurrent tasks}}, while preserving much of the look and feel of ordinary synchronous programming, through the [`async`][book-rust-reference-async]⮳ / [`await`][book-rust-reference-await]⮳ syntax. It helps you deal with events independently of the main program flow, using techniques like {{i:futures}}, {{i:promises}}, waiting, or eventing.

- Ability to make progress on multiple tasks, even if they don't execute at the exact same time.
- Mechanism: _cooperative_ multitasking - tasks yield control, allowing other tasks to run.
- Involves context switching on a single thread or, most often, among a few threads (the pool of which is opaquely managed by the async runtime).
- Achieves {{i:non-blocking I/O operations}} to improve responsiveness and efficiency.
- Lower overhead compared to multithreading.
- Multi-threaded async programming also requires careful synchronization to prevent data races.

Key constructs in Rust:

- [`async`][book-rust-reference-async]⮳ / [`await`][book-rust-reference-await]⮳ keywords
- [`Future`][std::future::Future]⮳

Here are the topics we’ll cover:

{{#include index.incl.md}}

[Are we async yet?][are-we-async-yet?]⮳

[Asynchronous Programming in Rust (book)][book-asynchronous-programming-in-rust]⮳

## Basic Example

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/async.rs}}
```

As any form of {{i:cooperative multitasking}}, a future that spends a long time without reaching an [`await`][book-rust-reference-await]⮳ "blocks the thread", which may prevent other tasks from running.

## Differences with other languages

Rust's implementation of [`async`][book-rust-reference-async]⮳ differs from most languages in a few ways:

- Rust's [`async`][book-rust-reference-async]⮳ operations are lazy. Futures are inert in Rust and only make progress only when polled. The executor calls the [`poll`][std::task::Poll]⮳ method repeatedly to execute futures.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/async2.rs}}
```

- Dropping a {{i:future stops}} it from making further progress.
- Async is zero-cost in Rust. You can use [`async`][book-rust-reference-async]⮳ without heap allocations and dynamic dispatch. This also lets you use async in constrained environments, such as embedded systems.
- No built-in runtime is provided by Rust itself. Instead, runtimes are provided by community-maintained crates.
- Both single- and multithreaded runtimes are available.

## Which crate provides what?

- The [`async`][book-rust-reference-async]⮳/[`await`][book-rust-reference-await]⮳ syntactic sugar is supported directly by the Rust compiler.
- The most fundamental traits, types, and functions, such as the [`Future`][std::future::Future]⮳ trait, are provided by the standard library.
- Many utility types, macros and functions are provided by the [`futures`][futures]⮳ crate. They can be used in any async Rust application.
- Execution of async code, IO and task spawning are provided by "async runtimes", such as [`Tokio`][tokio]⮳ and `async-std`. Most async applications, and some async crates, depend on a specific runtime.

## Async runtimes

In most cases, prefer the [Tokio](tokio.md) {{i:runtime}} - see [The State of Async Rust: Runtimes][blog-state-of-async-rust]⮳.

Alternatives to the Tokio async ecosystem include:

- [![async-std][async-std-badge]][async-std]  [async-std][async-std-crate]⮳: async version of the Rust standard library. No longer maintained?
- [![smol][smol-badge]][smol]  [Smol][smol-crate]⮳
- [![embassy][embassy-badge]][embassy]  [Embassy][embassy-website]⮳ [![embassy-github][embassy-github-badge]][embassy-github] for embedded systems.
- [![mio][mio-badge]][mio]  [Mio][mio-crate]⮳ is a fast, low-level I/O library for Rust focusing on non-blocking APIs and event notification for building high performance I/O apps with as little overhead as possible over the OS abstractions. It is part of the Tokio ecosystem.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
