#![allow(dead_code)]
// ANCHOR: example
use std::thread;
use std::time::Duration;

use flume::Receiver;
use flume::RecvTimeoutError;
use flume::Sender;
use flume::bounded;
use flume::unbounded;

// Example of using the `flume` crate for message passing.
//
// Features:
// - Unbounded, bounded and rendezvous queues
// - Drop-in replacement for `std::sync::mpsc`
// - Additional features like MPMC support and send timeouts/deadlines
// - `Sender` and `Receiver` both implement `Send + Sync + Clone`
// - Asynchronous support, including mix and match with sync code
// - `select`-like interface
// - No `unsafe` code

fn main() {
    // Create a bounded channel:
    let (tx, rx): (Sender<i32>, Receiver<i32>) = bounded(2);

    let tx2 = tx.clone();
    // Spawn a producer thread
    let producer_handle = thread::spawn(move || {
        for i in 1..6 {
            // If there is no space left for new messages, calls to
            // `Sender::send` will block
            // (unblocking once a receiver has made space).
            tx2.send(i).expect("All receivers have been dropped");
            println!("Produced: {}", i);
            // Send a value into the channel, returning an error if all
            // receivers have been dropped or the timeout has expired.
            match tx2.send_timeout(i + 10, Duration::from_millis(5)) {
                Ok(_) => println!("Produced: {}", i + 10),
                Err(e) => eprintln!("{}", e),
            }
        }
    });

    let rx2 = rx.clone();
    // Spawn a consumer thread
    let consumer_handle = thread::spawn(move || {
        // Receive values from the channel in a loop.
        loop {
            match rx2.recv_timeout(Duration::from_millis(10)) {
                Ok(value) => println!("Consumed: {}", value),
                e @ Err(RecvTimeoutError::Timeout) => {
                    eprintln!("The timeout has expired: {:?}.", e)
                }
                Err(RecvTimeoutError::Disconnected) => {
                    eprintln!(
                        "All senders are dropped and there are no values left in the channel."
                    );
                    break;
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Wait for producer thread to finish
    match producer_handle.join() {
        Ok(_) => println!("Producer thread finished."),
        Err(e) => eprintln!("Producer thread error: {:?}", e),
    }
    // Drop the original sender as well to signal to the consumer
    // that no more values will be sent.
    drop(tx);
    // All senders for this channel have been dropped.
    assert!(rx.is_disconnected());

    match consumer_handle.join() {
        Ok(_) => println!("Consumer thread finished."),
        Err(e) => eprintln!("Consumer thread error: {:?}", e),
    }

    select();
}

/// Requires the "select" feature to be enabled.
fn select() {
    let (tx0, rx0) = unbounded();
    let (tx1, rx1) = unbounded();

    std::thread::spawn(move || {
        tx0.send(true).unwrap();
        tx1.send(42).unwrap();
    });

    // `Selector` allows a thread to wait
    // upon the result of more than one operation at once.
    flume::Selector::new()
        .recv(&rx0, |b| println!("Received {:?}", b))
        .recv(&rx1, |n| println!("Received {:?}", n))
        .wait();
}
// ANCHOR_END: example

#[test]
fn test() {
    main()
}
