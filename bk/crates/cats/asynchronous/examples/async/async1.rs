#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Basics of `async` functions and `Future`s: calling, awaiting and combining
//! them.

use std::future::Future;

/// An empty value produced and consumed by the example tasks.
struct SomeStruct;

// Most often, we will use async functions.
// Rust transforms the `async fn` at compile time into a state machine
// that _implicitly_ returns a `Future`. A `Future` represents an
// asynchronous computation that might not have finished yet.
/// An `async fn` that returns a [`SomeStruct`].
async fn first_task() -> SomeStruct {
    // ...
    println!("First task");
    SomeStruct
}

/// An `async fn` that borrows the output of [`first_task`].
async fn second_task_1(_s: &SomeStruct) {
    // ...
    println!("Second task, part 1");
}

// `async fn` is really syntactic sugar for a function...
/// Equivalent of an `async fn` written as a function that returns a `Future`.
#[allow(clippy::manual_async_fn)]
fn second_task_2() -> impl Future<Output = ()> {
    // ...that contains an `async` block. An `async` block is a block of code
    // that can be paused and resumed.
    async {
        println!("Second task, part 2");
    }
    // It returns `Future<Output = ()>`.
}

/// Awaits the example tasks in sequence.
async fn do_something() {
    // `.await` polls the future and suspends this task if it is not ready.
    let s = first_task().await;
    // If the future is not ready, the executor can run other work and poll it
    // again later.

    // `join!` is like `.await`, but can wait for multiple futures
    // concurrently, returning when all branches complete.
    let f1 = second_task_1(&s);
    let f2 = second_task_2();
    futures::join!(f1, f2); // You could use `tokio::join!` as well.
}

// We replace `fn main()` by `async fn main()` and declare which
// executor runtime we'll use - in this case, Tokio. The runtime crate
// must be added to `Cargo.toml`:
// ```toml
// `tokio = { version = "1", features = ["full"] }`.
// ```
// The `#[tokio::main]` attribute is a macro that transforms `async fn main()`
// into a synchronous function that initializes a
// async runtime instance.
/// Runs [`do_something`] on the Tokio runtime.
#[tokio::main]
async fn main() {
    do_something().await;
    // Note: `await` must be called or nothing is executing!
    // Futures are lazy.
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
