// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Collects the results of multiple futures with `FuturesUnordered`.

use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;

/// Represents one independently completing task.
async fn complete_task(name: &'static str) -> &'static str {
    name
}

/// Runs three independent futures and collects each result as it completes.
async fn collect_results() -> Vec<&'static str> {
    let mut tasks = FuturesUnordered::new();
    tasks.push(complete_task("database"));
    tasks.push(complete_task("cache"));
    tasks.push(complete_task("configuration"));

    let mut results = Vec::new();
    while let Some(result) = tasks.next().await {
        results.push(result);
    }
    results
}

/// Runs the futures collection example's behavior.
fn run_example() {
    let results = futures_executor::block_on(collect_results());
    println!("Completed tasks: {results:?}");
}

/// Runs the futures collection example.
fn main() {
    run_example();
}

// ANCHOR_END: example

pub fn run() {
    run_example();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies each submitted future is polled to completion.
    #[test]
    fn all_results_are_collected() {
        let mut results = futures_executor::block_on(collect_results());
        results.sort_unstable();

        assert_eq!(results, ["cache", "configuration", "database"]);
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
