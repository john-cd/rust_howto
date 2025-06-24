#![allow(dead_code)]
// ANCHOR: example
// Define a trait with a method:
trait Message {
    fn send_message(&self);
}

// Create a struct that will implement our trait.
struct Worker {
    id: u32,
    msg: String,
}

// Implement the trait:
impl Message for Worker {
    fn send_message(&self) {
        println!("Worker {}: {}", self.id, self.msg);
    }
}

// A function that takes a trait object with an autotrait:
fn process_message(messenger: Box<dyn Message + Send>) {
    messenger.send_message();
}

fn main() {
    let worker = Worker {
        id: 1,
        msg: "Hello".to_string(),
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

    handle.join().unwrap();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
