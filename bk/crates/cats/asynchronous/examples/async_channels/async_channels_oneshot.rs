#![allow(dead_code)]
// ANCHOR: example
use tokio::sync::oneshot;

/// Simulate some computation that takes a `u32` and returns a `String`.
async fn some_computation(input: u32) -> String {
    format!("The result of computation is {input}")
}

/// Demonstrates the use of a `oneshot` channel for single-value communication.
async fn one_shot() {
    // Create a `oneshot` channel.
    // `tx` is the sending end, and `rx` is the receiving end.
    let (tx, rx) = oneshot::channel();

    // Spawn a new asynchronous task.
    tokio::spawn(async move {
        // Perform some computation.
        let res = some_computation(0).await;
        // Send the result through the channel.
        // `unwrap()` is used here for simplicity, but in a real application,
        // you should handle the potential error (e.g., if the receiver is
        // dropped).
        tx.send(res).unwrap();
        // Alternatively, return the value via the joinhandle returned
        // by `spawn`
    });

    // Do other work while the computation is happening in the background.
    // ...

    // Wait for the computation result.
    // `await` will block until a value is received or the channel is closed.
    // `unwrap()` is used here for simplicity, but in a real application,
    // you should handle the potential error (e.g., if the sender is dropped).
    let res = rx.await.unwrap();
    println!("{res}");
}

#[tokio::main]
async fn main() {
    one_shot().await;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
