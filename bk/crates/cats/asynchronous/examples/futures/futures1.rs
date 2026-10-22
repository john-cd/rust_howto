//! Races two futures with `select!` from the `futures` crate.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
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

#[tokio::main]
async fn main() {
    race_tasks().await;
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
