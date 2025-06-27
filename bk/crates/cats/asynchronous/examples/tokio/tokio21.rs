#![allow(dead_code)]
// ANCHOR: example
use std::rc::Rc;

use tokio::task;
use tokio::time;

/// Demonstrates using `tokio::task::LocalSet` to run non-Send tasks.
#[tokio::main]
async fn main() {
    // Data that is not thread-safe:
    let nonsend_data = Rc::new("world");

    // A set of tasks to be executed on the same thread:
    let local = task::LocalSet::new();

    let nonsend_data2 = nonsend_data.clone();
    local.spawn_local(async move {
        // ...
        println!("hello {nonsend_data2}")
    });

    local.spawn_local(async move {
        time::sleep(time::Duration::from_millis(100)).await;
        println!("goodbye {nonsend_data}")
    });

    // ...

    local.await;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
