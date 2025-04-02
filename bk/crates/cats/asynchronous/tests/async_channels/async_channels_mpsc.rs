// ANCHOR: example
use tokio::sync::mpsc;

/// Simulates some computation that takes a `u32` and returns a `String`.
async fn some_computation(input: u32) -> String {
    format!("The result of computation is {}", input)
}

/// Demonstrates a multi-producer, single-consumer (MPSC) channel.
/// Multiple producers send data to a single receiver.
pub async fn multi_producer_single_receiver() {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 1..=10 {
            let res = some_computation(i).await;
            tx.send(res).await.unwrap();
        }
    });

    while let Some(res) = rx.recv().await {
        println!("{}", res);
    }
}

#[tokio::main]
async fn main() {
    multi_producer_single_receiver().await;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
