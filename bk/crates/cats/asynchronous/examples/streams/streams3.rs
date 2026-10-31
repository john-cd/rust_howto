// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Processing a `Stream` of fallible asynchronous fetches with `futures_util`.
//!
//! A stream of fallible fetches yields `Result` items. You decide what an error
//! means: stop at the first failure, skip it, or collect all results and
//! failures to report later.

use std::time::Duration;

use futures_util::stream;
use futures_util::stream::StreamExt as _;
// You may also need:
// use futures_util::stream::TryStreamExt;
use tokio::time;

// Add these dependencies to your Cargo.toml:
// ```toml
// [dependencies]
// futures = "0.3"
// futures-util = "0.3"
// tokio = { version = "1", features = ["full"] }
// ```

// Simulated async function that returns a `Result`:
/// Simulates fetching the data of `id`; fails for some ids.
async fn fetch_data(id: u32) -> Result<String, String> {
    // Simulate network delay:
    time::sleep(Duration::from_millis(100)).await;
    if id.is_multiple_of(3) {
        Err(format!("Error fetching data for id {id}"))
    } else {
        Ok(format!("Data for id {id}"))
    }
}

/// Streams the results of several fetches and handles the errors.
#[tokio::main]
async fn main() {
    println!("\n===== Stream Utilities =====");

    // Create a stream of futures:
    // `iter` converts an `Iterator` into a `Stream`.
    let stream = stream::iter(vec![1, 2, 3, 4, 5]).map(fetch_data);

    // Process stream with `for_each_concurrent`, which
    // runs this stream to completion, executing the provided asynchronous
    // closure for each element on the stream concurrently as elements become
    // available. This is similar to `StreamExt::for_each`, but the futures
    // produced by the closure are run concurrently (but not in parallel - this
    // combinator does not introduce any threads).
    stream
        .for_each_concurrent(2, |future| async {
            match future.await {
                Ok(data) => println!("Processed concurrently: {data}"),
                Err(e) => println!("Error: {e}"),
            }
        })
        .await;

    println!("\n===== Stream Transformations =====");

    // Create a stream with successful and error results.
    let stream = stream::iter(vec![
        Ok::<_, String>("Item 1"),
        Ok("Item 2"),
        Err("Error 1".to_owned()),
        Ok("Item 3"),
        Err("Error 2".to_owned()),
    ]);

    // Using `filter_map` on a stream.
    // Filters the values produced by this stream
    // while simultaneously mapping them to a different type.
    let filtered: Vec<_> = stream
        .filter_map(|res| async move {
            match res {
                Ok(item) => Some(format!("Filtered: {item}")),
                Err(_) => None, // Filter out errors
            }
        })
        .collect()
        .await;

    println!("Filtered stream results: {filtered:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the streams3 example through its entry point.
    #[test]
    fn test_streams3_example_entry_point() {
        main();
    }

    #[tokio::test(start_paused = true)]
    async fn fetch_data_fails_for_multiples_of_three() {
        assert_eq!(fetch_data(2).await, Ok("Data for id 2".to_owned()));
        assert!(fetch_data(6).await.is_err());
    }

    #[tokio::test(start_paused = true)]
    async fn concurrent_processing_handles_every_item()
    -> Result<(), Box<dyn std::error::Error>> {
        let results = std::sync::Mutex::new(Vec::new());
        stream::iter(vec![1, 2, 3, 4, 5])
            .map(fetch_data)
            .for_each_concurrent(2, |f| async {
                let result = f.await;
                if let Ok(mut r) = results.lock() {
                    r.push(result);
                }
            })
            .await;
        let results = results.into_inner().map_err(|e| e.to_string())?;
        assert_eq!(results.len(), 5);
        assert_eq!(results.iter().filter(|r| r.is_err()).count(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn filter_map_drops_the_errors() {
        let items = stream::iter(vec![Ok("a"), Err("e".to_owned()), Ok("b")]);
        let kept: Vec<String> = items
            .filter_map(|r| async move { r.ok().map(str::to_uppercase) })
            .collect()
            .await;
        assert_eq!(kept, ["A", "B"]);
    }
}
