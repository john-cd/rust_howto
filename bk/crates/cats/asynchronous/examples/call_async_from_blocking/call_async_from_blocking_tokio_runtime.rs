#![allow(dead_code)]
// ANCHOR: example
fn main() {
    // Create a new multi-threaded runtime. The runtime is used to
    // execute asynchronous tasks.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        // Set the number of worker threads. In this case, we only
        // need one.
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        handles.push(runtime.spawn(my_bg_task(i)));
    }

    // Do something time-consuming while the async background tasks
    // execute.
    std::thread::sleep(std::time::Duration::from_millis(750));
    println!("Finished time-consuming task.");

    // Wait for all of them to complete.
    for handle in handles {
        // The `spawn` method returns a `JoinHandle`. A `JoinHandle` is
        // a future, so we can wait for it using `block_on`.
        runtime.block_on(handle).unwrap();
    }
}

// Example async code to execute
async fn my_bg_task(i: u64) {
    // By subtracting, the tasks with larger values of i sleep for a
    // shorter duration.
    let millis = 1000 - 50 * i;
    println!("Task {i} sleeping for {millis} ms.");

    tokio::time::sleep(tokio::time::Duration::from_millis(millis)).await;

    println!("Task {i} stopping.");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
