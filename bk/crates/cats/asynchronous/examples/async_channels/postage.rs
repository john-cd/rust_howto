// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Sends each message to the receivers subscribed when it is sent.

use anyhow::Result;
use postage::broadcast;
use postage::prelude::Sink as _;
use postage::prelude::Stream as _;
use tokio::task;

/// Sends two labeled messages, returning an error if the channel closes.
async fn broadcaster(id: usize, mut tx: broadcast::Sender<String>) -> Result<()> {
    for i in 0..2 {
        tx.send(format!("Broadcaster {id}'s message {i}")).await?;
    }
    Ok(())
}

/// Drains the receiver until every sender has been dropped.
async fn receive_all(mut rx: broadcast::Receiver<String>) -> Vec<String> {
    let mut messages = Vec::new();
    while let Some(message) = rx.recv().await {
        messages.push(message);
    }
    messages
}

/// Sends an initial batch to A and B, then sends one message to A, B, and C.
///
/// # Errors
///
/// Returns a send error if the channel closes or a join error if a task fails.
async fn exchange() -> Result<[Vec<String>; 3]> {
    let (mut tx, rx) = broadcast::channel(10);
    let receiver_b = task::spawn(receive_all(rx.clone()));
    let receiver_a = task::spawn(receive_all(rx));

    let (first, second) = tokio::join!(
        task::spawn(broadcaster(0, tx.clone())),
        task::spawn(broadcaster(1, tx.clone())),
    );
    first??;
    second??;

    // C subscribes after both producers finish, so it only sees the last message.
    let receiver_c = task::spawn(receive_all(tx.subscribe()));
    tx.send("Last message".to_owned()).await?;
    drop(tx);

    let (a, b, c) = tokio::join!(receiver_a, receiver_b, receiver_c);
    Ok([a?, b?, c?])
}

/// Prints each subscriber's messages after the exchange completes.
///
/// # Errors
///
/// Returns any send or join error from the exchange.
#[tokio::main]
async fn main() -> Result<()> {
    for (subscriber, messages) in ["A", "B", "C"].into_iter().zip(exchange().await?) {
        for message in messages {
            println!("{subscriber} received {message}");
        }
    }
    Ok(())
}
// ANCHOR_END: example

/// Runs the broadcast example.
///
/// # Errors
///
/// Returns any send or join error from the exchange.
pub fn run() -> Result<()> {
    main()
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the postage example through its entry point.
    #[test]
    fn test_postage_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Confirms the early and late subscribers drain the messages intended for them.
    #[tokio::test]
    async fn subscribers_drain_their_batches() -> anyhow::Result<()> {
        let [mut a, mut b, c] = exchange().await?;
        assert_eq!(a.pop().as_deref(), Some("Last message"));
        assert_eq!(b.pop().as_deref(), Some("Last message"));
        let expected = [
            "Broadcaster 0's message 0",
            "Broadcaster 0's message 1",
            "Broadcaster 1's message 0",
            "Broadcaster 1's message 1",
        ];
        a.sort_unstable();
        b.sort_unstable();
        assert_eq!(a, expected);
        assert_eq!(b, expected);
        assert_eq!(c, ["Last message"]);
        Ok(())
    }

    /// Delivers each broadcast message to every subscriber.
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

    /// Ends the receiver stream after all senders are dropped.
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
