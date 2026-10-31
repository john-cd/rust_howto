// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A Tokio multi-producer, single-consumer (`mpsc`) channel.
//!
//! A channel passes values between tasks so that they need not share memory.
//! `mpsc` means multi-producer, single-consumer: many senders, one receiver. A
//! bounded channel makes senders wait when it is full (backpressure), so a fast
//! producer cannot overwhelm a slow consumer.

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

    #[tokio::test]
    async fn some_computation_formats_the_input() {
        assert_eq!(some_computation(7).await, "The result of computation is 7");
    }

    #[tokio::test]
    async fn channel_delivers_messages_in_order_and_closes() {
        let (tx, mut rx) = mpsc::channel(4);
        tokio::spawn(async move {
            for i in 1..=3 {
                tx.send(some_computation(i).await)
                    .await
                    .expect("receiver open");
            }
        });
        let mut got = Vec::new();
        while let Some(res) = rx.recv().await {
            got.push(res);
        }
        assert_eq!(
            got,
            [
                "The result of computation is 1",
                "The result of computation is 2",
                "The result of computation is 3"
            ]
        );
    }

    #[tokio::test]
    async fn send_fails_once_the_receiver_is_dropped() {
        let (tx, rx) = mpsc::channel::<u32>(1);
        drop(rx);
        assert!(tx.send(1).await.is_err());
    }
}
