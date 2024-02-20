async fn parallel_sum(nums: Vec<i32>) -> i32 {
    let (tx, rx) = tokio::sync::oneshot::channel();

    // Spawn a task on a dedicate thread.
    std::thread::spawn(move || {
        // Perform an expensive computation on this thread...
        let sum = nums.into_iter().sum();

        // Send the result back to Tokio.
        let _ = tx.send(sum);
    });

    // Wait for the rayon task.
    rx.await.expect("Panic in rayon::spawn")
}

#[tokio::test]
async fn test() {
    let nums = vec![1; 1024 * 1024];
    println!("{}", parallel_sum(nums).await);
}
