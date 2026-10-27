#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Trait objects with auto traits such as `Send`.

// Define a trait with a method:
/// A thing that can send a message.
trait Message {
    /// Sends the message.
    fn send_message(&self);
}

// Create a struct that will implement our trait.
/// A worker that sends a message.
struct Worker {
    /// Identifier of the worker.
    id: u32,
    /// The message to send.
    msg: String,
}

// Implement the trait:
impl Message for Worker {
    fn send_message(&self) {
        println!("Worker {}: {}", self.id, self.msg);
    }
}

// A function that takes a trait object with an autotrait:
/// Sends the message of a trait object that is `Send`.
fn process_message(messenger: Box<dyn Message + Send>) {
    messenger.send_message();
}

/// Sends a message through a `Box<dyn Message + Send>`.
fn main() {
    let worker = Worker {
        id: 1,
        msg: "Hello".to_owned(),
    };

    // Create a boxed trait object.
    // The `Send` autotrait is automatically implemented for `Worker`
    // because all of its fields are `Send`.
    let messenger: Box<dyn Message + Send + 'static> = Box::new(worker);

    // Use the trait object in a separate thread.
    // This is possible because the trait object is `Send`.
    let handle = std::thread::spawn(move || {
        process_message(messenger);
    });

    handle.join().expect("worker thread should not panic");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the dyn autotraits example through its entry point.
    #[test]
    fn test_dyn_autotraits_example_entry_point() {
        main();
    }
}
