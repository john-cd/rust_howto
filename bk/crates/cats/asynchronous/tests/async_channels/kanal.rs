// ANCHOR: example
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
        println!("Producer {} sent: {}", id, i);
    }
    Ok(())
}

/// The consumer function receives messages from the channel.
/// It continues to receive messages until the channel is closed.
async fn consumer(rx: AsyncReceiver<i32>) -> anyhow::Result<()> {
    while let Ok(value) = rx.recv().await {
        println!("Consumer received: {}", value);
    }
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
