#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates message passing using multiple producers and a
//! single consumer. It showcases the use of `mpsc::channel` for creating a
//! channel, `tx.send` for sending messages, and `rx.recv` for receiving
//! messages.

use std::sync::mpsc;
use std::thread;
/// Sends messages from several producer threads to a single consumer through an
/// `mpsc` channel.
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    let first = thread::spawn(move || {
        for value in ["hi", "hi again"] {
            tx.send(String::from(value)).expect(
                "the message passing mpsc example invariant should hold",
            );
        }
    });

    let second = thread::spawn(move || {
        for value in ["more", "messages"] {
            tx2.send(String::from(value)).expect(
                "the message passing mpsc example invariant should hold",
            );
        }
    });

    // Both producer-owned senders are dropped when their threads finish, so
    // iteration ends after all four messages have arrived.
    let mut received: Vec<_> = rx.into_iter().collect();
    first.join().expect("first producer should finish");
    second.join().expect("second producer should finish");
    received.sort();

    let mut expected = vec!["hi", "hi again", "more", "messages"];
    expected.sort();
    assert_eq!(received, expected);
    println!("{received:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the message passing mpsc example through its entry point.
    #[test]
    fn test_message_passing_mpsc_example_entry_point() {
        main();
    }
}
