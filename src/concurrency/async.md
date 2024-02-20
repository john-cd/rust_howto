# Async

Asynchronous programming, or async for short, is a concurrent programming model supported by an increasing number of programming languages. It lets you run a large number of concurrent tasks, while preserving much of the look and feel of ordinary synchronous programming, through the async/await syntax

[Are we async yet?][are-we-async-yet?]⮳

[Asynchronous Programming in Rust (book)][book-asynchronous-programming-in-rust]⮳

## Basic Example

```rust,editable,mdbook-runnable
{{#include ../../deps/tests/async.rs}}
```

As any form of cooperative multitasking, a future that spends a long time without reaching an `await` "blocks the thread", which may prevent other tasks from running.

## Differences with other languages

Rust's implementation of `async` differs from most languages in a few ways:

- Rust's `async` operations are lazy. Futures are inert in Rust and only make progress only when polled. The executor calls the `poll` method repeatedly to execute futures.

```rust,editable,mdbook-runnable
{{#include ../../deps/tests/async2.rs}}
```

- Dropping a future stops it from making further progress.
- Async is zero-cost in Rust. You can use `async` without heap allocations and dynamic dispatch. This also lets you use async in constrained environments, such as embedded systems.
- No built-in runtime is provided by Rust itself. Instead, runtimes are provided by community-maintained crates.
- Both single- and multithreaded runtimes are available.

## Which crate provides what?

- The `async`/`await` syntaxic sugar is supported directly by the Rust compiler.
- The most fundamental traits, types, and functions, such as the `Future` trait, are provided by the standard library.
- Many utility types, macros and functions are provided by the `futures` crate. They can be used in any async Rust application.
- Execution of async code, IO and task spawning are provided by "async runtimes", such as `Tokio` and `async-std`. Most async applications, and some async crates, depend on a specific runtime.

## Async runtimes

In most cases, prefer the [Tokio](../async/tokio.md) runtime - see [The State of Async Rust: Runtimes][blog-state-of-async-rust]⮳.

Alternatives to the Tokio async ecosystem include:

- [![async-std-badge]][async-std] [async-std][async-std-crate]⮳: async version of the Rust standard library. No longer maintained?
- [![smol-badge]][smol] [Smol][smol-crate]⮳
- [![embassy-badge]][embassy] [Embassy][embassy-website]⮳ [![embassy-github][embassy-github-badge]][embassy-github] for embedded systems.
- [![mio-badge]][mio] [Mio][mio-crate]⮳ is a fast, low-level I/O library for Rust focusing on non-blocking APIs and event notification for building high performance I/O apps with as little overhead as possible over the OS abstractions. It is part of the Tokio ecosystem.

{{#include ../refs/link-refs.md}}
