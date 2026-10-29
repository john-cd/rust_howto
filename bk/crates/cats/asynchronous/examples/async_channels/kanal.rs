// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The `kanal` crate is used for inter-thread communication, similar to Rust's
//! standard `std::sync::mpsc` module but with additional features and
//! capabilities. It provides channels for sending messages between threads.

use kanal::AsyncReceiver;
use kanal::AsyncSender;
use tokio::task;

/// The `kanal` crate is used for inter-thread communication, similar to Rust's
/// standard `std::sync::mpsc` module but with additional features and
/// capabilities. It provides channels for sending messages between threads.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create an async bounded channel with a capacity of 0.
    // This means that the sender will block until a receiver is ready to
    // receive.
    let (tx, rx): (AsyncSender<i32>, AsyncReceiver<i32>) =
        kanal::bounded_async(0);

    // Spawn two producer tasks.
    // `tx.clone()` is used to create a new sender
    // that can be moved into the task.
    let producer1 = task::spawn(producer(tx.clone(), 1));
    let producer2 = task::spawn(producer(tx, 2));

    // Spawn a consumer task that will receive messages from the channel.
    let consumer = task::spawn(consumer(rx));

    // Wait for all tasks to finish.
    producer1.await??;
    producer2.await??;
    consumer.await??;

    Ok(())
}

/// The producer function sends 5 messages to the channel.
async fn producer(tx: AsyncSender<i32>, id: i32) -> anyhow::Result<()> {
    for i in 0..5 {
        tx.send(i).await?;
        println!("Producer {id} sent: {i}");
    }
    Ok(())
}

/// The consumer function receives messages from the channel.
/// It continues to receive messages until the channel is closed.
async fn consumer(rx: AsyncReceiver<i32>) -> anyhow::Result<()> {
    while let Ok(value) = rx.recv().await {
        println!("Consumer received: {value}");
    }
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the kanal example through its entry point.
    #[test]
    fn test_kanal_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    #[tokio::test]
    async fn every_produced_value_is_consumed() {
        let (tx, rx) = kanal::bounded_async::<i32>(0);
        let p1 = tokio::spawn(producer(tx.clone(), 1));
        let p2 = tokio::spawn(producer(tx, 2));
        let collector = tokio::spawn(async move {
            let mut got = Vec::new();
            while let Ok(v) = rx.recv().await {
                got.push(v);
            }
            got
        });
        p1.await.unwrap().unwrap();
        p2.await.unwrap().unwrap();
        let mut got = collector.await.unwrap();
        got.sort_unstable();
        assert_eq!(got, [0, 0, 1, 1, 2, 2, 3, 3, 4, 4]);
    }

    #[tokio::test]
    async fn producer_fails_when_the_receiver_is_gone() {
        let (tx, rx) = kanal::bounded_async::<i32>(0);
        drop(rx);
        assert!(producer(tx, 1).await.is_err());
    }

    #[tokio::test]
    async fn consumer_ends_when_the_sender_is_gone() {
        let (tx, rx) = kanal::bounded_async::<i32>(1);
        tx.send(1).await.unwrap();
        drop(tx);
        consumer(rx).await.expect("the consumer drains and stops");
    }
}
