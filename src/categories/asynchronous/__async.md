# Async

{{i:Asynchronous programming}}, or {{i:async}} for short, is a {{i:concurrent}} programming model supported by an increasing number of programming languages. It lets you run a large number of concurrent tasks, while preserving much of the look and feel of ordinary synchronous programming, through the {{i:async/await}} syntax

[Are we async yet?][are-we-async-yet?-website]⮳

[Asynchronous Programming in Rust (book)][book-asynchronous-programming-in-rust]⮳

## Basic Example

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/async.rs}}
```

As any form of {{i:cooperative multitasking}}, a {{i:future}} that spends a long time without reaching an [`{{i:await}}`][book-rust-reference-await]⮳ "blocks the thread", which may prevent other tasks from running.

## Differences with other languages

Rust's implementation of [`{{i:async}}`][book-rust-reference-async]⮳ differs from most languages in a few ways:

- Rust's [`{{i:async}}`][book-rust-reference-async]⮳ operations are lazy. {{i:Futures}} are inert in Rust and only make progress only when polled. The executor calls the [`{{i:poll}}`][c-std::task::Poll]⮳ method repeatedly to execute futures.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/async2.rs}}
```

- Dropping a future stops it from making further progress.
- Async is zero-cost in Rust. You can use [`{{i:async}}`][book-rust-reference-async]⮳ without {{i:heap allocations}} and dynamic dispatch. This also lets you use async in constrained environments, such as embedded systems.
- No built-in runtime is provided by Rust itself. Instead, runtimes are provided by community-maintained crates.
- Both single- and {{i:multithreaded runtimes}} are available.

## Which crate provides what?

- The [`{{i:async}}`][book-rust-reference-async]⮳ / [`{{i:await}}`][book-rust-reference-await]⮳ syntactic sugar is supported directly by the Rust compiler.
- The most fundamental traits, types, and functions, such as the [`{{i:Future}}`][c-std::future::Future]⮳ trait, are provided by the standard library.
- Many utility types, macros and functions are provided by the [`{{i:futures}}`][c-futures]⮳ crate. They can be used in any async Rust application.
- Execution of async code, IO and task spawning are provided by "async runtimes", such as [`{{i:Tokio}}`][c-tokio]⮳ and `{{i:async-std}}`. Most async applications, and some async crates, depend on a specific runtime.

## Async runtimes

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

In most cases, prefer the [`{{i:Tokio}}`](./tokio.md) runtime - see [The State of Async Rust: Runtimes][blog-state-of-async-rust]⮳.

Alternatives to the Tokio async ecosystem include:

- [![async-std][c-async-std-badge]][c-async-std]  [![async-std-crates-io][c-async-std-crates-io-badge]][c-async-std-crates-io]⮳: async version of the Rust standard library. No longer maintained?
- [![smol][c-smol-badge]][c-smol]  [Smol][c-smol-crates-io]⮳
- [![embassy][c-embassy-badge]][c-embassy]  [Embassy][c-embassy-website]⮳ [![embassy-github][c-embassy-github-badge]][c-embassy-github] for embedded systems.
- [![mio][c-mio-badge]][c-mio]  [Mio][c-mio-crates-io]⮳ is a fast, low-level I/O library for Rust focusing on non-blocking APIs and event notification for building high performance I/O apps with as little overhead as possible over the OS abstractions. It is part of the Tokio ecosystem.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
