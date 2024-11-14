# Async

Asynchronous programming{{hi:Asynchronous programming}}, or async{{hi:async}} for short, is a concurrent{{hi:Concurrency}} programming model supported by an increasing number of programming languages. It lets you run a large number of concurrent tasks, while preserving much of the look and feel of ordinary synchronous programming, through the async/await{{hi:async/await}} syntax

[Are we async yet?][are-we-async-yet?-website]⮳

[Asynchronous Programming in Rust (book)][book-asynchronous-programming-in-rust]⮳

## Basic Example {#basic-example}

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

```rust
{{#include ../../../deps/tests/cats/asynchronous/async1.rs:example}}
```

As any form of cooperative multitasking{{hi:Cooperative multitasking}}, a future{{hi:Futures}} that spends a long time without reaching an [`await`][book-rust-reference-await]{{hi:await}}⮳ "blocks the thread", which may prevent other tasks from running.

## Differences with other languages {#diff-other-languages}

Rust's implementation of [`async`][book-rust-reference-async]{{hi:async}}⮳ differs from most languages in a few ways:

- Rust's [`async`][book-rust-reference-async]{{hi:async}}⮳ operations are lazy. Futures{{hi:Futures}} are inert in Rust and only make progress only when polled. The executor calls the [`std::task::Poll`][c-std::task::Poll]{{hi:std::task::Poll}}⮳ method repeatedly to execute futures.

```rust
{{#include ../../../deps/tests/cats/asynchronous/async2.rs:example}}
```

- Dropping a future stops it from making further progress.
- Async is zero-cost in Rust. You can use [`async`][book-rust-reference-async]{{hi:async}}⮳ without heap allocations{{hi:Heap allocations}} and dynamic dispatch. This also lets you use async in constrained environments, such as embedded systems.
- No built-in runtime is provided by Rust itself. Instead, runtimes are provided by community-maintained crates.
- Both single- and multi-threaded runtimes{{hi:Multithreaded runtimes}} are available.

## Which crate provides what? {#which-crate-provides-what}

- The [`async`][book-rust-reference-async]{{hi:async}}⮳ / [`await`][book-rust-reference-await]{{hi:await}}⮳ syntactic sugar is supported directly by the Rust compiler.
- The most fundamental traits, types, and functions, such as the [`std::future::Future`][c-std::future::Future]{{hi:std::future::Future}}⮳ trait, are provided by the standard library.
- Many utility types, macros and functions are provided by the [`futures`][c-futures]{{hi:futures}}⮳ crate. They can be used in any async Rust application.
- Execution of async code, IO and task spawning are provided by "async runtimes", such as [`tokio`][c-tokio]{{hi:tokio}}⮳ and `async_std`{{hi:async-std}}. Most async applications, and some async crates, depend on a specific runtime.

## Async runtimes {#async-runtimes}

[![async-std][c-async_std-badge]][c-async_std]{{hi:async-std}}  [![smol][c-smol-badge]][c-smol]{{hi:smol}}  [![embassy][c-embassy-badge]][c-embassy]{{hi:embassy}}  [![mio][c-mio-badge]][c-mio]{{hi:mio}}  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

In most cases, prefer the [`tokio`][p-tokio]{{hi:tokio}} runtime - see [The State of Async Rust: Runtimes][blog-state-of-async-rust]⮳.

Alternatives to the Tokio async ecosystem include:

- [![async-std][c-async_std-badge]][c-async_std]{{hi:async-std}}  [![async_std-crates.io][c-async_std-crates.io-badge]][c-async_std-crates.io]⮳: async version of the Rust standard library. No longer maintained?
- [![smol][c-smol-badge]][c-smol]{{hi:smol}}  [Smol][c-smol-crates.io]⮳
- [![embassy][c-embassy-badge]][c-embassy]{{hi:embassy}}  [Embassy][c-embassy-website]⮳ [![embassy-github][c-embassy-github-badge]][c-embassy-github] for embedded systems.
- [![mio][c-mio-badge]][c-mio]{{hi:mio}}  [Mio][c-mio-crates.io]⮳ is a fast, low-level I/O library for Rust focusing on non-blocking APIs and event notification for building high performance I/O apps with as little overhead as possible over the OS abstractions. It is part of the Tokio ecosystem.

[p-tokio]: ./tokio.md
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
