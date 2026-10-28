// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A Tokio multi-producer, single-consumer (`mpsc`) channel.

use tokio::sync::mpsc;

/// Simulates some computation that takes a `u32` and returns a `String`.
async fn some_computation(input: u32) -> String {
    format!("The result of computation is {input}")
}

/// Demonstrates a multi-producer, single-consumer (MPSC) channel.
/// Multiple producers send data to a single receiver.
pub async fn multi_producer_single_receiver() {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 1..=10 {
            let res = some_computation(i).await;
            tx.send(res).await.expect(
                "the async channels mpsc example invariant should hold",
            );
        }
    });

    while let Some(res) = rx.recv().await {
        println!("{res}");
    }
}

/// Runs the async channels mpsc example.
#[tokio::main]
async fn main() {
    multi_producer_single_receiver().await;
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the async channels mpsc example through its entry point.
    #[test]
    fn test_async_channels_mpsc_example_entry_point() {
        main();
    }
}
