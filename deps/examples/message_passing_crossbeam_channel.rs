use crossbeam_channel::unbounded;
use crossbeam_channel::{RecvError, TryRecvError};
use std::thread;

fn main() {
    // Create a channel of unbounded capacity.
    let (s1, r1) = unbounded();

    // Alternatively, create a channel that can hold at most n messages at a time.
    // let (s1, r1) = bounded(5);

    // Senders and receivers can be cloned to use them to multiple threads.
    // cloning only creates a new handle to the same sending or receiving side.
    // It does not create a separate stream of messages in any way
    let s2 = s1.clone();

    // Send a message into the channel.
    // Note that the cloned sender is moved into the thread.
    thread::spawn(move || s2.send("Hi!").unwrap());

    // Blocks until receiving the message from the channel.
    assert_eq!(r1.recv(), Ok("Hi!"));

    // Try receiving a message without blocking.
    // The channel is now empty
    assert_eq!(r1.try_recv(), Err(TryRecvError::Empty));

    s1.send("0").unwrap();
    // Receive all remaining messages currently in the channel (non-blocking).
    let v: Vec<_> = r1.try_iter().collect();
    println!("{:?}", v);

    // When all senders or all receivers associated with a channel get dropped, the channel becomes disconnected.
    s1.send("1").unwrap();
    drop(s1);

    // No more messages can be sent...
    // ERROR s1.send("2").unwrap();

    // .. but any remaining messages can still be received.
    println!("{:?}", r1.iter().collect::<Vec<_>>());
    // Note that the call to `collect` would block if the channel were not disconnected.

    // There are no more messages in the channel.
    assert!(r1.is_empty());

    // After disconnection, calling `r1.recv()` does not block
    // Instead, `Err(RecvError)` is returned immediately.
    assert_eq!(r1.recv(), Err(RecvError));
}
