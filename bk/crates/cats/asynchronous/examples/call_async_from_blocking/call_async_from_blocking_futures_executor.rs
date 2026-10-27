#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Calls `async` code from blocking code with `futures::executor::block_on`.

/// Prints a greeting.
async fn do_something() {
    println!("hello, world!");
}

/// Drives [`do_something`] to completion with `block_on`.
fn main() {
    let future = do_something();
    // Futures are lazy - nothing is happening
    // until driven to completion by `.await`, `block_on`...

    // `block_on` blocks the current thread until the provided future has
    // run to completion. Other executors provide more complex
    // behavior, like scheduling multiple futures onto the same
    // thread. See `Tokio`.
    futures::executor::block_on(future);
    // `future` is run and "hello, world!" is printed.
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the call async from blocking futures executor example through its
    /// entry point.
    #[test]
    fn test_call_async_from_blocking_futures_executor_example_entry_point() {
        main();
    }
}
