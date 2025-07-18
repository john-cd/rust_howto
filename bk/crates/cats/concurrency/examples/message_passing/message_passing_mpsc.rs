#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates message passing using multiple producers and a
//! single consumer. It showcases the use of `mpsc::channel` for creating a
//! channel, `tx.send` for sending messages, and `rx.recv` for receiving
//! messages.
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![String::from("hi"), String::from("hi again")];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![String::from("more"), String::from("messages")];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    while let Ok(msg) = rx.recv() {
        println!("{msg}");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
