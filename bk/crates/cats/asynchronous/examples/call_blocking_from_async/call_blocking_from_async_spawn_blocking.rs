// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to call a blocking function from an
//! asynchronous context using `tokio::task::spawn_blocking`.

/// This example demonstrates how to call a blocking function from an
/// asynchronous context using `tokio::task::spawn_blocking`.
///
/// In asynchronous programming, blocking operations can halt the progress of
/// the entire asynchronous runtime. To avoid this,
/// `tokio::task::spawn_blocking` allows us to offload blocking tasks to a
/// separate thread pool.
#[tokio::main]
async fn main() {
    // This is running on the Tokio runtime. We should avoid blocking here.

    // `spawn_blocking` moves the closure to a separate thread pool where
    // blocking is acceptable.
    let blocking_task = tokio::task::spawn_blocking(|| {
        println!("Inside spawn_blocking");
    });

    // Await the completion of the blocking task.
    blocking_task.await.expect("the call blocking from async spawn blocking example invariant should hold");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the call blocking from async spawn blocking example through its
    /// entry point.
    #[test]
    fn test_call_blocking_from_async_spawn_blocking_example_entry_point() {
        main();
    }
}
