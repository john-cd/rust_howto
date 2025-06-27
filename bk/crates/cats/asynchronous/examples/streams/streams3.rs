#![allow(dead_code)]
// ANCHOR: example
use std::time::Duration;

use futures_util::stream;
use futures_util::stream::StreamExt;
// You may also need:
// use futures_util::stream::TryStreamExt;
use tokio::time;

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
        Err(format!("Error fetching data for id {id}"))
    } else {
        Ok(format!("Data for id {id}"))
    }
}

#[tokio::main]
async fn main() {
    println!("\n===== Stream Utilities =====");

    // Create a stream of futures:
    // `iter` converts an `Iterator` into a `Stream`
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
        Err("Error 1".to_string()),
        Ok("Item 3"),
        Err("Error 2".to_string()),
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

#[test]
fn test() {
    main();
}
