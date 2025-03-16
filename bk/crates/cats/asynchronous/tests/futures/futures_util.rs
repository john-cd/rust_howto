// ANCHOR: example
use std::time::Duration;

use futures::future;
use futures_util::future::FutureExt;
use futures_util::future::TryFutureExt;
use futures_util::pin_mut;
use tokio::time;

// This example showcases various utilities for working with asynchronous code.

// Add these dependencies to your Cargo.toml:
// [dependencies]
// futures = "0.3"
// futures-util = "0.3"
// tokio = { version = "1", features = ["full"] }

// Simulated async function that returns a `Result`:
async fn fetch_data(id: u32) -> Result<String, String> {
    // Simulate network delay
    time::sleep(Duration::from_millis(100)).await;
    if id % 3 == 0 {
        Err(format!("Error fetching data for id {}", id))
    } else {
        Ok(format!("Data for id {}", id))
    }
}

#[tokio::main]
async fn main() {
    println!("\n===== Combining Futures =====");
    // `select` for racing futures,
    // `join_all` for waiting on multiple futures,
    // `try_join` for handling errors in combined futures.

    // `ready` creates a future that is immediately ready with a value.
    let future1 =
        time::sleep(Duration::from_millis(100)).then(|_| future::ready(1));
    let future2 =
        time::sleep(Duration::from_millis(50)).then(|_| future::ready(2));

    // `select` requires `Future` + `Unpin` bounds.
    pin_mut!(future1);
    pin_mut!(future2);

    // Select between multiple futures:
    match future::select(future1, future2).await {
        future::Either::Left((val, _)) => {
            println!("Future 1 completed first with: {}", val)
        }
        future::Either::Right((val, _)) => {
            println!("Future 2 completed first with: {}", val)
        }
    }

    // Joining futures:
    let futures = vec![fetch_data(1), fetch_data(2)];
    // The future returned by `join_all` will drive execution for all of its
    // underlying futures, collecting the results into a destination Vec<T>
    // in the same order as they were provided.
    let results: Vec<Result<String, String>> = future::join_all(futures).await;
    println!("Join results: {:?}", results);

    // `try_join`:
    // If successful, the returned future will finish with a tuple of both
    // results. If unsuccessful, it will complete with the first error
    // encountered.
    let results = future::try_join(fetch_data(1), fetch_data(4)).await;
    println!("try_join results: {:?}", results);

    println!("===== Future Extensions =====");
    // `map` to transform future outputs,
    // `and_then` for chaining asynchronous operations,
    // `or_else` for error handling.

    // Using `map` to transform the output of a Future
    let future = fetch_data(1).map(|result| match result {
        Ok(data) => Ok(format!("Processed: {}", data)),
        Err(e) => Err(e),
    });
    println!("Map result: {:?}", future.await);

    // Using `and_then` for chaining futures
    let future = fetch_data(2).and_then(|data| async move {
        // Simulate additional processing
        time::sleep(Duration::from_millis(50)).await;
        Ok(format!("Enhanced: {}", data))
    });
    println!("AndThen result: {:?}", future.await);

    // Using `or_else` for error handling
    let future = fetch_data(3).or_else(|err| async move {
        println!("Recovering from error: {}", err);
        // Return a fallback value
        Ok::<_, String>(String::from("Fallback data"))
    });
    println!("OrElse result: {:?}", future.await);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
