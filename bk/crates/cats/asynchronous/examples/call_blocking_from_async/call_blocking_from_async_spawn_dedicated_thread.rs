// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Runs CPU-bound work from `async` code on a dedicated thread.

/// Sums `nums` on a dedicated thread and returns the result through a channel.
async fn parallel_sum(nums: Vec<i32>) -> i32 {
    let (tx, rx) = tokio::sync::oneshot::channel();

    // Spawn a task on a dedicate thread.
    std::thread::spawn(move || {
        // Perform an expensive computation on this thread...
        let sum = nums.into_iter().sum();

        // Send the result back to the main async task.
        // The underscore is used to ignore the result of the send operation.
        // If the receiver is dropped before the sender sends a value, the send
        // operation will return an error.
        let _ = tx.send(sum);
    });

    // Wait for the result from the dedicated thread.
    rx.await.expect("Panic in rayon::spawn")
}

/// Sums a vector on a dedicated thread and prints the result.
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

    /// Runs the call blocking from async spawn dedicated thread example through
    /// its entry point.
    #[test]
    fn test_call_blocking_from_async_spawn_dedicated_thread_example_entry_point()
     {
        main();
    }

    #[tokio::test]
    async fn parallel_sum_adds_all_numbers() {
        assert_eq!(parallel_sum(vec![1; 1024 * 1024]).await, 1024 * 1024);
        assert_eq!(parallel_sum(vec![]).await, 0);
        assert_eq!(parallel_sum(vec![-3, 3, 10]).await, 10);
    }
}
