#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates a basic actor implementation pattern.
//!
//! It spawns a worker thread that receives messages over an MPSC channel,
//! prints them, and exits cleanly on a stop signal.
//! An actor is a concurrent entity that can receive and process messages.
//! It's a fundamental building block for concurrent systems.
use std::sync::mpsc;
use std::thread;

enum Message {
    Print(String),
    Stop,
}

fn main() {
    let (sender, receiver) = mpsc::channel::<Message>();

    let actor = thread::spawn(move || {
        while let Ok(message) = receiver.recv() {
            match message {
                Message::Print(text) => println!("actor received: {text}"),
                Message::Stop => break,
            }
        }
    });

    sender
        .send(Message::Print(
            "Hello from an actor-like worker".to_string(),
        ))
        .expect("channel should be open");
    sender.send(Message::Stop).expect("channel should be open");

    actor.join().expect("actor thread should finish cleanly");
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
    // [finish](https://github.com/john-cd/rust_howto/issues/1011)
}
