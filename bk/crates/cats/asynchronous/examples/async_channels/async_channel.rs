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
//! - **Bounded Channels**: Backpressure is enforced by setting a fixed capacity limit. When the
//!   buffer is full, `send().await` suspends execution until space becomes available.
//! - **Multiple Producers and Consumers**: Both `Sender` and `Receiver` types implement `Clone`.
//!   Cloning a sender or receiver creates another handle to the same underlying channel.
//! - **Channel Closure**: When all `Sender` instances are dropped (or explicitly closed), receivers
//!   will process remaining buffered messages and then return `Err(RecvError)` or
//!   `TryRecvError::Closed`.

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

/// Spawns the specified number of producer tasks.
fn spawn_producers(tx: &Sender<String>, count: usize) -> Vec<task::JoinHandle<()>> {
    let mut tasks = Vec::with_capacity(count);
    for i in 0..count {
        let producer_tx = tx.clone();
        tasks.push(task::spawn(producer(i, producer_tx)));
    }
    tasks
}

/// Spawns the specified number of consumer tasks.
fn spawn_consumers(rx: &Receiver<String>, count: usize) -> Vec<task::JoinHandle<()>> {
    let mut tasks = Vec::with_capacity(count);
    for i in 0..count {
        let consumer_rx = rx.clone();
        tasks.push(task::spawn(consumer(i, consumer_rx)));
    }
    tasks
}

/// Coordinates the full lifecycle of producers and consumers on a bounded channel.
async fn run_channel_pipeline(
    capacity: usize,
    num_producers: usize,
    num_consumers: usize,
) -> anyhow::Result<()> {
    let (tx, rx) = bounded(capacity);
    assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));

    let producer_tasks = spawn_producers(&tx, num_producers);
    let consumer_tasks = spawn_consumers(&rx, num_consumers);

    for task in producer_tasks {
        task.await?;
    }
    println!(
        "The current number of messages in the channel is {}",
        tx.len()
    );

    // Close channel to signal consumers that no further messages will arrive.
    drop(tx);
    assert!(rx.is_closed());

    for task in consumer_tasks {
        task.await?;
    }
    assert!(rx.is_empty());
    Ok(())
}

/// Spawns producers and consumers that share one channel and waits for them to
/// finish.
#[tokio::main]
async fn main() {
    if let Err(err) = run_channel_pipeline(2, 3, 2).await {
        eprintln!("Pipeline execution failed: {err}");
    }
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

    /// Verifies that sending to a closed channel fails promptly.
    #[tokio::test]
    async fn producer_terminates_when_channel_is_closed() {
        let (tx, rx) = bounded::<String>(1);
        drop(rx);
        assert!(tx.is_closed());
        let result = tx.send("hello".to_owned()).await;
        assert!(result.is_err());
    }

    /// Verifies that consuming from an empty closed channel yields Closed error.
    #[tokio::test]
    async fn consumer_observes_closure_on_empty_channel() {
        let (tx, rx) = bounded::<String>(2);
        drop(tx);
        assert_eq!(rx.try_recv(), Err(TryRecvError::Closed));
        assert!(rx.recv().await.is_err());
    }

    /// Verifies that consumer receives buffered items before observing channel closure.
    #[tokio::test]
    async fn consumer_drains_remaining_messages_after_close() {
        let (tx, rx) = bounded(5);
        tx.send("first".to_owned()).await.unwrap();
        tx.send("second".to_owned()).await.unwrap();
        drop(tx);

        assert_eq!(rx.recv().await, Ok("first".to_owned()));
        assert_eq!(rx.recv().await, Ok("second".to_owned()));
        assert_eq!(rx.try_recv(), Err(TryRecvError::Closed));
    }

    /// Verifies that the channel pipeline completes successfully with single producer and consumer.
    #[tokio::test]
    async fn pipeline_runs_to_completion() -> anyhow::Result<()> {
        run_channel_pipeline(1, 1, 1).await
    }
}
