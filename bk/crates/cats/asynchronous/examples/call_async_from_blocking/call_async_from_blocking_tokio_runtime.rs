// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calls `async` code from blocking code by building a Tokio runtime by hand.

/// Spawns background tasks on a one-thread runtime, works on the main thread
/// meanwhile, then waits for the tasks.
fn main() {
    // Create a new multi-threaded runtime. The runtime is used to
    // execute asynchronous tasks.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        // Set the number of worker threads. In this case, we only
        // need one.
        .worker_threads(1)
        .enable_all()
        .build()
        .expect("the call async from blocking tokio runtime example invariant should hold");

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        handles.push(runtime.spawn(my_bg_task(i)));
    }

    // Do something time-consuming while the async background tasks
    // execute.
    std::thread::sleep(std::time::Duration::from_millis(100));
    println!("Finished time-consuming task.");

    // Wait for all of them to complete.
    for handle in handles {
        // The `spawn` method returns a `JoinHandle`. A `JoinHandle` is
        // a future, so we can wait for it using `block_on`.
        runtime.block_on(handle).expect("the call async from blocking tokio runtime example invariant should hold");
    }
}

// Example async code to execute
/// A background task that prints its number `i`.
async fn my_bg_task(i: u64) {
    // By subtracting, the tasks with larger values of i sleep for a
    // shorter duration.
    let millis = 200 - 20 * i;
    println!("Task {i} sleeping for {millis} ms.");

    tokio::time::sleep(tokio::time::Duration::from_millis(millis)).await;

    println!("Task {i} stopping.");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the call async from blocking tokio runtime example through its
    /// entry point.
    #[test]
    fn test_call_async_from_blocking_tokio_runtime_example_entry_point() {
        main();
    }

    #[test]
    fn background_task_runs_on_the_runtime()
    -> Result<(), Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .start_paused(true)
            .build()?;
        runtime.block_on(my_bg_task(9));
        Ok(())
    }

    #[test]
    fn spawned_tasks_finish_even_with_one_worker()
    -> Result<(), Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?;
        let handle = runtime.spawn(async { 7 });
        assert_eq!(runtime.block_on(handle)?, 7);
        Ok(())
    }
}
