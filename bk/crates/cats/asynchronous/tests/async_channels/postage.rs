// ANCHOR: example
use postage::broadcast;
use postage::prelude::Stream;
use postage::sink::Sink;
use tokio::task;
use tokio::time::Duration;

async fn broadcaster(id: usize, mut tx: broadcast::Sender<String>) {
    for i in 0..2 {
        let msg = format!("Broadcaster {}'s message {}", id, i);
        if let Err(err) = tx.send(msg.clone()).await {
            // `send` returns Err(SendError(value))
            // if the sink rejected the message.
            eprintln!("Failed to send message: {}", err);
            break;
        }
        println!("Sent: {}", msg);
        // Simulate work
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

async fn receiver(name: &'static str, mut rx: broadcast::Receiver<String>) {
    while let Some(msg) = rx.recv().await {
        println!("{} receive {}", name, msg);
    }
}

#[tokio::main]
async fn main() {
    // The broadcast channel provides reliable broadcast delivery between
    // multiple senders and multiple receivers. The channel has a fixed
    // capacity, and senders are suspended if the buffer is filled.
    let (tx, rx) = broadcast::channel(10);

    let mut broadcaster_tasks = vec![];
    for i in 0..2 {
        let tx = tx.clone();
        broadcaster_tasks.push(task::spawn(broadcaster(i, tx)));
    }

    // Let's create a couple of receivers:
    let rx2 = rx.clone();
    task::spawn(receiver("A", rx));
    task::spawn(receiver("B", rx2));

    tokio::time::sleep(Duration::from_millis(50)).await;

    // We may also subscribe to the channel.
    // The receiver will observe all messages sent _after the call to
    // subscribe_. Messages currently in the buffer will not be received.
    let rx3 = tx.subscribe();
    task::spawn(receiver("C", rx3));
    let mut tx2 = tx.clone();
    tx2.send("Last message".into()).await.ok();

    // Wait for all the receivers to print
    tokio::time::sleep(Duration::from_millis(25)).await;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [finish; polish postage.rs example / add examples for other queues; logging; stream, sink](https://github.com/john-cd/rust_howto/issues/80)
