// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates a small asynchronous workload using `smol`.

use std::time::Duration;

use smol::Timer;

/// Runs several asynchronous tasks on the `smol` runtime and waits for their
/// results.
fn main() {
    // Run the async block on the smol runtime.
    smol::block_on(async {
        // Spawn a background task that waits and then returns a value.
        let task1 = smol::spawn(async {
            Timer::after(Duration::from_millis(20)).await;
            println!("Task 1 complete");
            1
        });

        // Spawn a second task with a shorter delay.
        let task2 = smol::spawn(async {
            Timer::after(Duration::from_millis(10)).await;
            println!("Task 2 complete");
            2
        });

        // Wait for both tasks to complete and combine their results.
        let result = task1.await + task2.await;
        println!("Total result: {result}");
    });
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the smol example through its entry point.
    #[test]
    fn test_smol_example_entry_point() {
        main();
    }

    #[test]
    fn spawned_tasks_return_their_values() {
        let total = smol::block_on(async {
            let a = smol::spawn(async {
                Timer::after(Duration::from_millis(5)).await;
                1
            });
            let b = smol::spawn(async { 2 });
            a.await + b.await
        });
        assert_eq!(total, 3);
    }

    #[test]
    fn timer_waits_at_least_the_requested_duration() {
        let start = std::time::Instant::now();
        smol::block_on(Timer::after(Duration::from_millis(20)));
        assert!(start.elapsed() >= Duration::from_millis(20));
    }
}
