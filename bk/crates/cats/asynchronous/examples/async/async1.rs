#![allow(dead_code)]
// ANCHOR: example
use std::future::Future;

struct SomeStruct;

/// Most often, we will use async functions.
/// Rust transforms the `async fn` at compile time into a state machine
/// that _implicitly_ returns a `Future`. A `Future` represents an
/// asynchronous computation that might not have finished yet.
async fn first_task() -> SomeStruct {
    // ...
    println!("First task");
    SomeStruct
}

async fn second_task_1(_s: &SomeStruct) {
    // ...
    println!("Second task, part 1");
}

/// `async fn` is really syntactic sugar for a function...
#[allow(clippy::manual_async_fn)]
fn second_task_2() -> impl Future<Output = ()> {
    // ...that contains an `async` block. An `async` block is a block of code
    // that can be paused and resumed.
    async {
        println!("Second task, part 2");
    }
    // It returns `Future<Output = ()>`
}

async fn do_something() {
    // Use `.await` to start executing the future.
    let s = first_task().await;
    // `await` yields control back to the executor, which may decide to do
    // other work if the task is not ready, then come back here.

    // `join!` is like `.await`, but can wait for multiple futures
    // concurrently, returning when all branches complete.
    let f1 = second_task_1(&s);
    let f2 = second_task_2();
    futures::join!(f1, f2); // You could use tokio::join! as well.
}

/// We replace `fn main()` by `async fn main()` and declare which
/// executor runtime we'll use - in this case, Tokio. The runtime crate
/// must be added to `Cargo.toml`:
/// ```toml
/// `tokio = { version = "1", features = ["full"] }`.
/// ```
/// The `#[tokio::main]` attribute is a macro that transforms `async fn main()`
/// into a synchronous function that initializes a
/// async runtime instance.
#[tokio::main]
async fn main() {
    do_something().await;
    // Note: `await` must be called or nothing is executing!
    // Futures are lazy.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
