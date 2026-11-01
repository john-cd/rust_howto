// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Runs CPU-bound work from `async` code on the Rayon thread pool.
//!
//! An async runtime runs many tasks on a few threads, so a task that computes
//! for a long time without awaiting blocks the others. Handing CPU-heavy work
//! to a separate thread pool, such as Rayon's, keeps the runtime responsive.

use rayon::prelude::*;

/// Computes the sum of a vector of numbers in parallel using Rayon.
///
/// This function spawns a Rayon task to perform the sum and uses a `oneshot`
/// channel to return the result to the Tokio runtime.
async fn parallel_sum(nums: Vec<i32>) -> i32 {
    let (tx, rx) = tokio::sync::oneshot::channel();

    // Spawn a task on rayon.
    rayon::spawn(move || {
        // Perform an expensive computation on this thread...

        // ...or compute the sum on multiple rayon threads.
        let sum = nums.par_iter().sum();

        // Send the result back to Tokio.
        let _ = tx.send(sum);
    });

    // Wait for the rayon task.
    rx.await.expect("Panic in rayon::spawn")
}

/// Sums a vector in parallel and prints the result.
#[tokio::main]
async fn main() {
    let nums = vec![1; 1024 * 1024];
    println!("{}", parallel_sum(nums).await);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the call blocking from async rayon example through its entry point.
    #[test]
    fn test_call_blocking_from_async_rayon_example_entry_point() {
        main();
    }

    #[tokio::test]
    async fn parallel_sum_adds_all_numbers() {
        assert_eq!(parallel_sum(vec![1; 1024 * 1024]).await, 1024 * 1024);
        assert_eq!(parallel_sum(vec![]).await, 0);
        assert_eq!(parallel_sum(vec![-3, 3, 10]).await, 10);
    }
}
