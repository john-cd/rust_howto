// ANCHOR: example
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
