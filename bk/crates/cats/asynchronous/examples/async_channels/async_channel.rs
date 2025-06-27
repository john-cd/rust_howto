#![allow(dead_code)]
// ANCHOR: example
use async_channel::Receiver;
use async_channel::Sender;
use async_channel::TryRecvError;
use async_channel::bounded;
use rand::Rng;
use tokio::task;
use tokio::time;
use tokio::time::Duration;

/// `producer` function:
///
/// This function simulates a producer that sends messages to a channel.
///
/// # Arguments
/// * `id` - The ID of the producer.
async fn producer(id: usize, tx: Sender<String>) {
    for i in 0..5 {
        let msg = format!("Producer {id}: Message {i}");
        // Sends messages to the channel.
        // It creates messages in a loop, sends them to the channel.
        // If the channel is full, this method awaits until there is space for a
        // message.
        if let Err(err) = tx.send(msg).await {
            // The channel is closed.
            eprintln!("Failed to send message: {err}");
            break;
        }
        // Simulate work
        let sleep_duration = rand::rng().random_range(10..50);
        time::sleep(Duration::from_millis(sleep_duration)).await;
    }
}

/// `consumer` function:
///
/// This function simulates a consumer that receives messages from a channel.
///
/// # Arguments
///
/// * `id` - The ID of the consumer.
async fn consumer(id: usize, rx: Receiver<String>) {
    // Receives a message from the channel.
    // If the channel is empty, awaits until there is a message.
    // If the channel is closed, receives a message or returns an error if there
    // are no more messages.
    while let Ok(msg) = rx.recv().await {
        println!("Consumer {id}: Received {msg}");
        // Simulate processing
        let sleep_duration = rand::rng().random_range(30..100);
        time::sleep(Duration::from_millis(sleep_duration)).await;
    }
    assert_eq!(rx.try_recv(), Err(TryRecvError::Closed));
}

#[tokio::main]
async fn main() {
    let (tx, rx) = bounded(2); // Create a bounded channel with a capacity of 2
    // You may also use an unbounded queue.
    assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));

    // Create 3 producer tasks
    let mut producer_tasks = vec![];
    for i in 0..3 {
        let tx = tx.clone();
        producer_tasks.push(task::spawn(producer(i, tx)));
    }
    assert_eq!(tx.sender_count(), 4);

    // Create 2 consumer tasks
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

    // Close the channel to signal consumers that no more messages will be sent
    drop(tx);
    // or tx.close();
    assert!(rx.is_closed());

    for task in consumer_tasks {
        let _ = task.await;
    }
    // The channel is empty.
    assert!(rx.is_empty());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
