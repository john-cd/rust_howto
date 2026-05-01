#![allow(dead_code)]
// ANCHOR: example
use futures::{
    future::FutureExt, // Provides the `.fuse()` method for futures.
    pin_mut,
    select,
};

// Define two asynchronous tasks.
async fn task_one() {
    // ...
}
async fn task_two() {
    // ...
}

// Define an asynchronous function that races two tasks.
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

#[test]
fn test() {
    main();
}
