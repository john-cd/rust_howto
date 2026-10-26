#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates bounded concurrent processing with a deterministic stream.

use std::time::Duration;

use anyhow::Result;
use futures::StreamExt as _;
use futures::TryStreamExt as _;
use futures::stream;
use tokio::time::sleep;

/// Processes a fixed list of items with bounded concurrency.
async fn process_items() -> Result<Vec<String>> {
    let items = ["red", "green", "blue", "yellow"];
    let item_count = items.len();
    stream::iter(items.into_iter().enumerate())
        .map(move |(index, value)| async move {
            // Simulate independent asynchronous work without using a network.
            sleep(Duration::from_millis(4 * (item_count - index) as u64)).await;
            Ok::<_, anyhow::Error>(format!("{index}:{value}"))
        })
        .buffer_unordered(2)
        .try_collect()
        .await
}

/// Runs the example and prints the results in input order.
#[tokio::main]
async fn main() -> Result<()> {
    let mut results = process_items().await?;
    results.sort();
    for result in results {
        println!("{result}");
    }
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies stream processing completes offline and returns every item.
    #[tokio::test]
    async fn processes_all_items_without_network_access() -> Result<()> {
        let mut results = process_items().await?;
        results.sort();

        assert_eq!(results, ["0:red", "1:green", "2:blue", "3:yellow"]);
        Ok(())
    }
}
