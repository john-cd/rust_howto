# Async

Asynchronous programming, or async for short, is a concurrent programming model supported by an increasing number of programming languages. It lets you run a large number of concurrent tasks, while preserving much of the look and feel of ordinary synchronous programming, through the async/await syntax

[Are we async yet?]( https://areweasyncyet.rs/ )

[Asynchronous Programming in Rust (book)]( https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html )

## Basic Example

```rust,ignore
// Most often, we will use async functions.
// Rust transforms the `async fn` at compile time into a state machine that _implicitly_ returns a `Future`.
// A future represents an asynchronous computation that might not have finished yet.
async fn first_task() -> SomeStruct { /* ... */ }

async fn second_task_1(&s: SomeStruct ) { /* ... */ }

// `async fn` is really syntaxic sugar for a function...
fn second_task_2() -> impl Future<Output = ()> {
    // ...that contains an `async` block.
    async {
        ()
    }   // returns `Future<Output = ()>`
}

async fn do_something() {
    // Use `.await` to start executing the future.
    let s = first_task().await;
    // `await` yields control back to the executor, which may decide to do other work if the task is not ready, then come back here.

    // `join!` is like `.await` but can wait for multiple futures concurrently, returning when all branches complete.
    let f1 = second_task_1(&s);
    let f2 = second_task_2();
    futures::join!(f1, f2);     // or tokio::join!
}

// We replace `fn main()` by `async fn main()` and declare which executor runtime we'll use - in this case, Tokio.
// The runtime crate must be added to `Cargo.toml`: `tokio = { version = "1", features = ["full"] }`
// Technically, the #[tokio::main] attribute is a macro that transforms it into a synchronous fn main() that initializes a runtime instance and executes the async main function.
#[tokio::main]
async fn main() {
    do_something().await; // note: `await` must be called or nothing is executing - Futures are lazy
}
```

As any form of cooperative multitasking, a future that spends a long time without reaching an `await` "blocks the thread", which may prevent other tasks from running.

## Differences with other languages

Rust's implementation of `async` differs from most languages in a few ways:

- Rust's `async` operations are lazy. Futures are inert in Rust and only make progress only when polled. The executor calls the `poll` method repeatedly to execute futures.

```rust,ignore
async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();

    // This println! comes first
    println!("hello");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}
// Prints:
// hello
// world
// Example from https://tokio.rs/tokio/tutorial/hello-tokio
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

In most cases, prefer the [Tokio](../topics/tokio.md) runtime - see [The State of Async Rust: Runtimes]( https://corrode.dev/blog/async/ ).

Alternatives to the Tokio async ecosystem include:

- [async-std]( https://crates.io/crates/async-std ): async version of the Rust standard library. No longer maintained?
- [Smol]( https://crates.io/crates/smol )
- [Embassy]( https://embassy.dev/ ) for embedded systems.
- [Mio]( https://crates.io/crates/mio ) is a fast, low-level I/O library for Rust focusing on non-blocking APIs and event notification for building high performance I/O apps with as little overhead as possible over the OS abstractions. It is part of the Tokio ecosystem.
