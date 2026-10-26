#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Channels from the `flume` crate: bounded and unbounded, sync and async, and
//! `select`.

use std::thread;

use flume::Selector;
use flume::bounded;
use flume::unbounded;

// Example of using the `flume` crate for message passing.
//
// Features:
// - Unbounded, bounded and rendezvous queues.
// - Drop-in replacement for `std::sync::mpsc`.
// - Additional features like MPMC support and send timeouts/deadlines.
// - `Sender` and `Receiver` both implement `Send + Sync + Clone`.
// - Asynchronous support, including mix and match with sync code.
// - `select`-like interface.
// - No `unsafe` code.

/// Sends messages through a bounded `flume` channel and selects between
/// channels.
fn main() {
    // A bounded channel applies back-pressure when its two slots are full.
    let (tx, rx) = bounded(2);
    let producer = thread::spawn(move || {
        for value in 1..=5 {
            tx.send(value).expect("receiver remains alive");
        }
    });

    // Iteration ends when the producer drops its sender.
    let mut received: Vec<_> = rx.iter().collect();
    producer.join().expect("producer should finish");
    received.sort();
    assert_eq!(received, [1, 2, 3, 4, 5]);
    println!("Received: {received:?}");

    assert_eq!(select(), (true, 42));
}

/// Selects between two ready channels and returns both received values.
fn select() -> (bool, i32) {
    let (tx0, rx0) = unbounded();
    let (tx1, rx1) = unbounded();

    let producer = thread::spawn(move || {
        tx0.send(true).expect("receiver remains alive");
        tx1.send(42).expect("receiver remains alive");
    });

    enum Selected {
        Boolean(bool),
        Number(i32),
    }

    // `Selector` waits for whichever receive operation becomes ready first.
    let selected = Selector::new()
        .recv(&rx0, |value| {
            Selected::Boolean(value.expect("sender remains alive"))
        })
        .recv(&rx1, |value| {
            Selected::Number(value.expect("sender remains alive"))
        })
        .wait();
    let result = match selected {
        Selected::Boolean(value) => {
            (value, rx1.recv().expect("second value was sent"))
        }
        Selected::Number(value) => {
            (rx0.recv().expect("first value was sent"), value)
        }
    };
    producer.join().expect("producer should finish");
    result
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
