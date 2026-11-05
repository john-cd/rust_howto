//! TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the use of crossbeam channels for message passing between
//! threads.
//!
//! A channel lets threads pass values to each other instead of sharing memory.
//! Crossbeam channels can have many producers and many consumers, and can be
//! bounded so that senders wait when the queue is full (backpressure).

use std::thread;
use std::thread::JoinHandle;

use crossbeam_channel::Receiver;
use crossbeam_channel::RecvError;
use crossbeam_channel::Sender;
use crossbeam_channel::TryRecvError;
use crossbeam_channel::unbounded;

/// Spawns a thread that sends one message into the channel.
///
/// The sender is moved into the thread. The returned handle lets the caller wait for it.
fn spawn_producer(sender: Sender<&'static str>, message: &'static str) -> JoinHandle<()> {
    thread::spawn(move || {
        sender
            .send(message)
            .expect("the message passing crossbeam channel example invariant should hold");
    })
}

/// Receives every message currently in the channel without blocking.
fn drain_without_blocking(receiver: &Receiver<&'static str>) -> Vec<&'static str> {
    receiver.try_iter().collect()
}

/// Disconnects the channel after sending one last message, then receives what remains.
///
/// Dropping the only sender disconnects the channel, so iterating over the receiver ends instead
/// of blocking forever.
fn send_then_disconnect(
    sender: Sender<&'static str>,
    receiver: &Receiver<&'static str>,
    message: &'static str,
) -> Vec<&'static str> {
    sender
        .send(message)
        .expect("the message passing crossbeam channel example invariant should hold");
    drop(sender);
    receiver.iter().collect()
}

/// Demonstrates the use of crossbeam channels for message passing between
/// threads.
fn main() {
    // Create a channel of unbounded capacity.
    let (s1, r1) = unbounded();

    // Alternatively, create a channel that can hold at most n messages at
    // a time:
    // let (s1, r1) = bounded(5);

    // Senders and receivers can be cloned to use them to multiple
    // threads. Cloning only creates a new handle to the same sending
    // or receiving side. It does not create a separate stream of
    // messages in any way.
    let s2 = s1.clone();

    // Spawn a new thread to send a message into the channel.
    // Note that the cloned sender is moved into the thread.
    let producer = spawn_producer(s2, "Hi!");

    // Blocks until receiving the message from the channel.
    // Receive the message and assert that it's "Hi!".
    assert_eq!(r1.recv(), Ok("Hi!"));
    producer.join().expect("producer should finish");

    // Try receiving a message without blocking.
    // The channel is now empty:
    assert_eq!(r1.try_recv(), Err(TryRecvError::Empty));

    s1.send("0")
        .expect("the message passing crossbeam channel example invariant should hold");
    // Receive all remaining messages currently in the channel
    // (non-blocking).
    let v = drain_without_blocking(&r1);
    assert_eq!(v, ["0"]);
    println!("{v:?}");

    // When all senders or all receivers associated with a channel get
    // dropped, the channel becomes disconnected.
    // Once the sender is dropped, no more messages can be sent...
    // ERROR s1.send("2").unwrap();

    // ...but any remaining messages can still be received.
    let remaining = send_then_disconnect(s1, &r1, "1");
    assert_eq!(remaining, ["1"]);
    println!("{remaining:?}");
    // Note that the call to `collect` would block if the channel were not
    // disconnected.

    // There are no more messages in the channel.
    // Check if the channel is empty.
    assert!(r1.is_empty());

    // After disconnection, calling `r1.recv()` does not block
    // Instead, `Err(RecvError)` is returned immediately.
    assert_eq!(r1.recv(), Err(RecvError));
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the message passing crossbeam channel example through its entry
    /// point.
    #[test]
    fn test_message_passing_crossbeam_channel_example_entry_point() {
        main();
    }

    /// A producer thread delivers its message, and an empty channel does not block.
    #[test]
    fn producer_delivers_message_and_empty_channel_does_not_block() {
        let (sender, receiver) = unbounded();
        spawn_producer(sender, "hello")
            .join()
            .expect("producer should finish");
        assert_eq!(receiver.recv(), Ok("hello"));
        assert!(drain_without_blocking(&receiver).is_empty());
    }

    /// Draining returns queued messages in order.
    #[test]
    fn drain_without_blocking_preserves_order() {
        let (sender, receiver) = unbounded();
        for message in ["a", "b", "c"] {
            sender.send(message).expect("receiver is alive");
        }
        assert_eq!(drain_without_blocking(&receiver), ["a", "b", "c"]);
    }

    /// Disconnecting still lets queued messages be received, then reports disconnection.
    #[test]
    fn send_then_disconnect_returns_remaining_messages() {
        let (sender, receiver) = unbounded();
        sender.send("queued").expect("receiver is alive");
        assert_eq!(
            send_then_disconnect(sender, &receiver, "last"),
            ["queued", "last"]
        );
        assert_eq!(receiver.recv(), Err(RecvError));
        assert_eq!(receiver.try_recv(), Err(TryRecvError::Disconnected));
    }
}
