# Async

Asynchronous programming, or async for short, is a concurrent programming model supported by an increasing number of programming languages. It lets you run a large number of concurrent tasks, while preserving much of the look and feel of ordinary synchronous programming, through the async/await syntax

[Are we async yet?][are-we-async-yet?]⮳

[Asynchronous Programming in Rust (book)][asynchronous-programming-in-rust-book]⮳

## Basic Example

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/async.rs}}
```

As any form of cooperative multitasking, a future that spends a long time without reaching an `await` "blocks the thread", which may prevent other tasks from running.

## Differences with other languages

Rust's implementation of `async` differs from most languages in a few ways:

- Rust's `async` operations are lazy. Futures are inert in Rust and only make progress only when polled. The executor calls the `poll` method repeatedly to execute futures.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/async2.rs}}
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

In most cases, prefer the [Tokio](../concurrency/tokio.md) runtime - see [The State of Async Rust: Runtimes]( https://corrode.dev/blog/async/ )⮳.

Alternatives to the Tokio async ecosystem include:

- [async-std][async-std-crate]⮳: async version of the Rust standard library. No longer maintained?
- [Smol][smol-crate]⮳
- [Embassy][embassy-web]⮳ for embedded systems.
- [Mio][mio-crate]⮳ is a fast, low-level I/O library for Rust focusing on non-blocking APIs and event notification for building high performance I/O apps with as little overhead as possible over the OS abstractions. It is part of the Tokio ecosystem.

[are-we-async-yet?]: https://areweasyncyet.rs/
[asynchronous-programming-in-rust-book]: https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html

{{#include ../links.md}}
