# Message passing

One increasingly popular approach to ensuring safe concurrency is message passing, where threads communicate by sending each other messages containing data. The Rust standard library provides channels for message passing that are safe to use in concurrent contexts.

Message passing in `async` programming is covered in a separate page: [async channels](async_channels.md)

## Multiple producers, single consumer

```rust,editable
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("hi again"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    while let Ok(msg) = rx.recv() {
        println!("{msg}");
    }
}
```

## Crossbeam_channel

Multi-producer multi-consumer channels for message passing.

```rust,editable,ignore
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
```

Example using specialized channels for tickers and timeout

```rust,editable,ignore
use std::time::{Duration, Instant};
use crossbeam_channel::{after, select, tick};

let start = Instant::now();
// channel that delivers messages periodically.
let ticker = tick(Duration::from_millis(50));
// channel that delivers a single message after a certain duration of time.
let timeout = after(Duration::from_secs(1));

loop {
    // `select` wait until any one of the channels becomes ready and execute it.
    select! {
        recv(ticker) -> _ => println!("elapsed: {:?}", start.elapsed()),
        recv(timeout) -> _ => break,
        // or use: default(Duration::from_millis(1000)) => break,
    }
}
```

## Reference

[Message Passing]( https://doc.rust-lang.org/book/ch16-02-message-passing.html )
