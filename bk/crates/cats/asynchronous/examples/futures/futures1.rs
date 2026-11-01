// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Races two futures with `select!` from the `futures` crate.
//!
//! `select!` waits for several futures and returns when the first one
//! completes, dropping the others. Typical uses are racing a request against a
//! timeout, or listening for work and a shutdown signal at once.

use futures::{
    future::FutureExt as _, // Provides the `.fuse()` method for futures.
    pin_mut,
    select,
};

// Define two asynchronous tasks.
/// An example task.
async fn task_one() {
    // ...
}

/// Another example task.
async fn task_two() {
    // ...
}

// Define an asynchronous function that races two tasks.
/// Runs both tasks and reports which one completes first.
async fn race_tasks() {
    // Fuse the two tasks so that they can be used with `select!`.
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    // Pin the futures to the stack using `pin_mut!`.
    pin_mut!(t1, t2);

    // Use `select!` to race the two tasks.
    select! {
        () = t1 => println!("task one completed first"),
        () = t2 => println!("task two completed first"),
    }
}

/// Runs the futures1 example.
#[tokio::main]
async fn main() {
    race_tasks().await;
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the futures1 example through its entry point.
    #[test]
    fn test_futures1_example_entry_point() {
        main();
    }

    #[tokio::test]
    async fn select_completes_when_a_task_is_ready() {
        race_tasks().await;
        let a = std::future::ready(1).fuse();
        let b = std::future::pending::<i32>().fuse();
        pin_mut!(a, b);
        let winner = select! {
            x = a => x,
            y = b => y,
        };
        assert_eq!(winner, 1);
    }
}
