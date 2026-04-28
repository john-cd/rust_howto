// ANCHOR: example
//! This example demonstrates a small asynchronous workload using `smol`.

use std::time::Duration;

use smol::Timer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        let result = task1.await? + task2.await?;
        println!("Total result: {result}");
        Ok(())
    })
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        main().unwrap();
    }
}
// TODO add to a chapter on asynchronous programming with smol
