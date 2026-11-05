// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the use of crossbeam channels for message passing between
//! threads.
//!
//! A channel lets threads pass values to each other instead of sharing memory.
//! Crossbeam channels can have many producers and many consumers, and can be
//! bounded so that senders wait when the queue is full (backpressure).

use std::thread;

use crossbeam_channel::Receiver;
use crossbeam_channel::RecvError;
use crossbeam_channel::Sender;
use crossbeam_channel::TryRecvError;
use crossbeam_channel::unbounded;

/// Spawns a background thread to send a message across a cloned channel sender.
fn send_on_worker_thread<T: Send + 'static>(sender: Sender<T>, message: T) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        sender
            .send(message)
            .expect("the message passing crossbeam channel example invariant should hold");
    })
}

/// Drains all currently available messages without blocking.
fn drain_available<T>(receiver: &Receiver<T>) -> Vec<T> {
    receiver.try_iter().collect()
}

/// Collects all messages from a disconnected channel until drained.
fn collect_remaining<T>(receiver: &Receiver<T>) -> Vec<T> {
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
    let producer = send_on_worker_thread(s2, "Hi!");

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
    let v: Vec<_> = drain_available(&r1);
    assert_eq!(v, ["0"]);
    println!("{v:?}");

    // When all senders or all receivers associated with a channel get
    // dropped, the channel becomes disconnected.
    s1.send("1")
        .expect("the message passing crossbeam channel example invariant should hold");
    drop(s1);

    // No more messages can be sent...
    // ERROR s1.send("2").unwrap();

    // ...but any remaining messages can still be received.
    let remaining: Vec<_> = collect_remaining(&r1);
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

    /// Verifies that draining an empty channel yields an empty vector.
    #[test]
    fn drain_available_on_empty_channel_returns_empty_vec() {
        let (_s, r) = unbounded::<i32>();
        assert!(drain_available(&r).is_empty());
    }

    /// Verifies non-blocking drain collects multiple pending items.
    #[test]
    fn drain_available_collects_all_buffered_items() {
        let (s, r) = unbounded();
        s.send(10).unwrap();
        s.send(20).unwrap();
        s.send(30).unwrap();
        assert_eq!(drain_available(&r), vec![10, 20, 30]);
        assert_eq!(r.try_recv(), Err(TryRecvError::Empty));
    }

    /// Verifies that worker thread message sender works with join handle.
    #[test]
    fn worker_thread_sends_payload_successfully() {
        let (s, r) = unbounded();
        let handle = send_on_worker_thread(s, 42);
        handle.join().unwrap();
        assert_eq!(r.recv(), Ok(42));
    }

    /// Verifies disconnected channel error behavior for send and receive.
    #[test]
    fn disconnected_channel_returns_errors() {
        let (s, r) = unbounded::<&str>();
        drop(r);
        assert!(s.send("fail").is_err());

        let (s2, r2) = unbounded::<&str>();
        drop(s2);
        assert_eq!(r2.recv(), Err(RecvError));
    }
}
