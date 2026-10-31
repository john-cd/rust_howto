// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Combinators from `futures_util`, applied to asynchronous fetches that may
//! fail.
//!
//! Combinators transform futures and streams without writing state machines by
//! hand, much as iterator adapters do for iterators. Here fetches that can fail
//! are mapped, filtered and collected.

use std::time::Duration;

use futures::future;
use futures_util::future::FutureExt as _;
use futures_util::future::TryFutureExt as _;
use futures_util::pin_mut;
use tokio::time;

// This example demonstrates how to use `futures-util` to work with
// asynchronous code.

// Add these dependencies to your Cargo.toml:
// ```toml
// [dependencies]
// futures = "0.3"
// futures-util = "0.3"
// tokio = { version = "1", features = ["full"] }
// ```

/// Simulated async function that returns a `Result`:
async fn fetch_data(id: u32) -> Result<String, String> {
    // Simulate network delay:
    time::sleep(Duration::from_millis(100)).await;
    if id.is_multiple_of(3) {
        Err(format!("Error fetching data for id {id}"))
    } else {
        Ok(format!("Data for id {id}"))
    }
}

/// Combines the results of several simulated fetches in different ways.
#[tokio::main]
async fn main() {
    println!("\n===== Combining Futures =====");
    // Demonstrates:
    // - `select` for racing futures,
    // - `join_all` for waiting on multiple futures,
    // - `try_join` for handling errors in combined futures.

    // `ready` creates a future that is immediately ready with a value.
    let future1 =
        time::sleep(Duration::from_millis(100)).then(|()| future::ready(1));
    let future2 =
        time::sleep(Duration::from_millis(50)).then(|()| future::ready(2));

    // `select` requires `Future` + `Unpin` bounds.
    pin_mut!(future1);
    pin_mut!(future2);

    // Select between multiple futures:
    match future::select(future1, future2).await {
        future::Either::Left((val, _)) => {
            println!("Future 1 completed first with: {val}");
        }
        future::Either::Right((val, _)) => {
            println!("Future 2 completed first with: {val}");
        }
    }

    // Join futures:
    let futures = vec![fetch_data(1), fetch_data(2)];
    // The future returned by `join_all` will drive execution for all of its
    // underlying futures, collecting the results into a destination `Vec<T>`
    // in the same order as they were provided.
    let results: Vec<Result<String, String>> = future::join_all(futures).await;
    println!("Join results: {results:?}");

    // `try_join`:
    // If successful, the returned future will finish with a tuple of both
    // results. If unsuccessful, it will complete with the first error
    // encountered.
    let results = future::try_join(fetch_data(1), fetch_data(4)).await;
    println!("try_join results: {results:?}");

    println!("===== Future Extensions =====");
    // Demonstrates:
    // - `map` to transform future outputs,
    // - `and_then` for chaining asynchronous operations,
    // - `or_else` for error handling.

    // Use `map` to transform the output of a Future:
    let future = fetch_data(1).map(|result| match result {
        Ok(data) => Ok(format!("Processed: {data}")),
        Err(e) => Err(e),
    });
    println!("Map result: {:?}", future.await);

    // Use `and_then` for chaining futures:
    let future = fetch_data(2).and_then(|data| async move {
        // Simulate additional processing:
        time::sleep(Duration::from_millis(50)).await;
        Ok(format!("Enhanced: {data}"))
    });
    println!("AndThen result: {:?}", future.await);

    // Use `or_else` for error handling:
    let future = fetch_data(3).or_else(|err| async move {
        println!("Recovering from error: {err}");
        // Return a fallback value:
        Ok::<_, String>(String::from("Fallback data"))
    });
    println!("OrElse result: {:?}", future.await);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the futures util example through its entry point.
    #[test]
    fn test_futures_util_example_entry_point() {
        main();
    }

    #[tokio::test(start_paused = true)]
    async fn fetch_data_fails_for_multiples_of_three() {
        assert_eq!(fetch_data(1).await, Ok("Data for id 1".to_owned()));
        assert_eq!(
            fetch_data(3).await,
            Err("Error fetching data for id 3".to_owned())
        );
        assert!(fetch_data(0).await.is_err());
    }

    #[tokio::test(start_paused = true)]
    async fn combinators_transform_and_recover()
    -> Result<(), Box<dyn std::error::Error>> {
        let mapped =
            fetch_data(1).map(|r| r.map(|d| format!("Processed: {d}")));
        assert_eq!(mapped.await?, "Processed: Data for id 1");

        let recovered = fetch_data(3)
            .or_else(|_| async { Ok::<_, String>("Fallback data".to_owned()) });
        assert_eq!(recovered.await?, "Fallback data");

        let both = future::try_join(fetch_data(1), fetch_data(4)).await;
        assert_eq!(
            both?,
            ("Data for id 1".to_owned(), "Data for id 4".to_owned())
        );
        assert!(
            future::try_join(fetch_data(1), fetch_data(3))
                .await
                .is_err()
        );
        Ok(())
    }

    #[tokio::test(start_paused = true)]
    async fn select_returns_the_shorter_sleep_first() {
        let slow =
            time::sleep(Duration::from_millis(100)).then(|()| future::ready(1));
        let fast =
            time::sleep(Duration::from_millis(50)).then(|()| future::ready(2));
        pin_mut!(slow, fast);
        match future::select(slow, fast).await {
            future::Either::Right((val, _)) => assert_eq!(val, 2),
            future::Either::Left(_) => panic!("the shorter sleep should win"),
        }
    }
}
