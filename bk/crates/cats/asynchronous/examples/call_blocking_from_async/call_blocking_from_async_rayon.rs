#![allow(dead_code)]
// ANCHOR: example
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

#[tokio::main]
async fn main() {
    let nums = vec![1; 1024 * 1024];
    println!("{}", parallel_sum(nums).await);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
