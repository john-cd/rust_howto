// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A broadcast channel from the `postage` crate: several receivers get every
//! message.
//!
//! A broadcast channel delivers every message to every receiver, like a notice
//! board, instead of to just one of them. It suits events that several parts of
//! a program react to, such as a shutdown signal.

use postage::broadcast;
use postage::prelude::Sink as _;
use postage::prelude::Stream as _;
use tokio::task;
use tokio::time::Duration;

/// This function simulates a broadcaster that sends messages to a broadcast
/// channel. The broadcaster sends two messages, each with a unique identifier,
/// and then sleeps for a short duration.
async fn broadcaster(id: usize, mut tx: broadcast::Sender<String>) {
    for i in 0..2 {
        let msg = format!("Broadcaster {id}'s message {i}");
        if let Err(err) = tx.send(msg.clone()).await {
            // `send` returns Err(SendError(value))
            // if the sink rejected the message.
            eprintln!("Failed to send message: {err}");
            break;
        }
        println!("Sent: {msg}");
        // Simulate work:
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

/// This function simulates a receiver that receives messages from a broadcast
/// channel. The receiver continuously receives messages and prints them until
/// the channel is closed.
async fn receiver(name: &'static str, mut rx: broadcast::Receiver<String>) {
    while let Some(msg) = rx.recv().await {
        println!("{name} receive {msg}");
    }
}

/// Spawns broadcasters and receivers on one channel and waits for them to
/// finish.
#[tokio::main]
async fn main() {
    // Create a broadcast channel with a capacity of 10 messages.
    // The broadcast channel provides reliable broadcast delivery between
    // multiple senders and multiple receivers. The channel has a fixed
    // capacity, and senders are suspended if the buffer is filled.
    let (tx, rx) = broadcast::channel(10);

    let mut broadcaster_tasks = vec![];
    for i in 0..2 {
        let tx = tx.clone();
        broadcaster_tasks.push(task::spawn(broadcaster(i, tx)));
    }

    // Create a couple of receivers:
    let rx2 = rx.clone();
    task::spawn(receiver("A", rx));
    task::spawn(receiver("B", rx2));

    // Let the broadcasters and receivers run for a short duration:
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Subscribe to the channel.
    // The receiver will observe all messages sent _after the call to
    // subscribe_. Messages currently in the buffer will not be received.
    let rx3 = tx.subscribe();
    task::spawn(receiver("C", rx3));
    // Send a last message to the channel.
    let mut tx2 = tx.clone();
    tx2.send("Last message".into()).await.ok();

    // Wait for all the receivers to print:
    tokio::time::sleep(Duration::from_millis(25)).await;
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the postage example through its entry point.
    #[test]
    fn test_postage_example_entry_point() {
        main();
    }

    #[tokio::test]
    async fn every_subscriber_receives_the_broadcast_messages()
    -> Result<(), Box<dyn std::error::Error>> {
        let (tx, mut rx1) = broadcast::channel(10);
        let mut rx2 = rx1.clone();
        let mut tx = tx;
        tx.send("one".to_owned()).await?;
        tx.send("two".to_owned()).await?;
        for rx in [&mut rx1, &mut rx2] {
            assert_eq!(rx.recv().await.as_deref(), Some("one"));
            assert_eq!(rx.recv().await.as_deref(), Some("two"));
        }
        Ok(())
    }

    #[tokio::test]
    async fn receiver_stream_ends_when_all_senders_are_dropped()
    -> Result<(), Box<dyn std::error::Error>> {
        let (mut tx, mut rx) = broadcast::channel(2);
        tx.send("only".to_owned()).await?;
        drop(tx);
        assert_eq!(rx.recv().await.as_deref(), Some("only"));
        assert_eq!(rx.recv().await, None);
        Ok(())
    }
}
