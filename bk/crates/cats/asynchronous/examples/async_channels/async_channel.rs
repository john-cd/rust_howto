// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Multi-producer, multi-consumer (MPMC) async channel communication.
//!
//! ### Overview
//! The `async_channel` crate provides multi-producer, multi-consumer channels
//! where multiple tasks can send messages (`Sender`) and multiple tasks can
//! receive messages (`Receiver`).
//!
//! ### Key Concepts
//! - **Bounded Channels**: Backpressure is enforced by setting a fixed capacity
//!   limit. When the buffer is full, `send().await` suspends execution until
//!   space becomes available.
//! - **Multiple Producers and Consumers**: Both `Sender` and `Receiver` types
//!   implement `Clone`. Cloning a sender or receiver creates another handle to
//!   the same underlying channel.
//! - **Channel Closure**: When all `Sender` instances are dropped (or
//!   explicitly closed), receivers will process remaining buffered messages and
//!   then return `Err(RecvError)` or `TryRecvError::Closed`.

use async_channel::Receiver;
use async_channel::Sender;
use async_channel::TryRecvError;
use async_channel::bounded;
use rand::RngExt as _;
use tokio::task;
use tokio::time::Duration;

/// This function simulates a producer that sends messages to a channel.
///
/// # Arguments
/// * `id` - The ID of the producer.
/// * `tx` - The sender side of the channel used to send messages.
async fn producer(id: usize, tx: Sender<String>) {
    // Create messages in a loop, sending them to the channel.
    for i in 0..5 {
        let msg = format!("Producer {id}: Message {i}");
        // Sends messages to the channel.
        // If the channel is full, this method awaits
        // until there is space for a message.
        if let Err(err) = tx.send(msg).await {
            // The channel is closed.
            eprintln!("Failed to send message: {err}");
            break;
        }
        // Simulate work:
        let sleep_duration = rand::rng().random_range(10..50);
        tokio::time::sleep(Duration::from_millis(sleep_duration)).await;
    }
}

/// This function simulates a consumer that receives messages from a channel.
///
/// # Arguments
///
/// * `id` - The ID of the consumer.
/// * `rx` - The receiver side of the channel used to receive messages.
async fn consumer(id: usize, rx: Receiver<String>) {
    // Receives a message from the channel.
    // If the channel is empty, awaits until there is a message.
    // If the channel is closed, receives a message or returns an error if there
    // are no more messages.
    while let Ok(msg) = rx.recv().await {
        println!("Consumer {id}: Received {msg}");
        // Simulate processing:
        let sleep_duration = rand::rng().random_range(30..100);
        tokio::time::sleep(Duration::from_millis(sleep_duration)).await;
    }
    assert_eq!(rx.try_recv(), Err(TryRecvError::Closed));
}

/// Spawns producers and consumers that share one channel and waits for them to
/// finish.
#[tokio::main]
async fn main() {
    let (tx, rx) = bounded(2); // Create a bounded channel with a capacity of 2.
    // You may also use an unbounded queue.

    assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));

    // Create 3 producer tasks:
    let mut producer_tasks = vec![];
    for i in 0..3 {
        let tx = tx.clone();
        producer_tasks.push(task::spawn(producer(i, tx)));
    }
    assert_eq!(tx.sender_count(), 4);

    // Create 2 consumer tasks:
    let mut consumer_tasks = vec![];
    for i in 0..2 {
        let rx = rx.clone();
        consumer_tasks.push(task::spawn(consumer(i, rx)));
    }
    assert_eq!(rx.receiver_count(), 3);

    for task in producer_tasks {
        let _ = task.await;
    }
    println!(
        "The current number of messages in the channel is {}",
        tx.len()
    );

    // Close the channel to signal consumers that no more messages will be sent:
    drop(tx);
    // or tx.close();
    assert!(rx.is_closed());

    for task in consumer_tasks {
        let _ = task.await;
    }
    // The channel is empty:
    assert!(rx.is_empty());
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the async channel example through its entry point.
    #[test]
    fn test_async_channel_example_entry_point() {
        main();
    }
}
