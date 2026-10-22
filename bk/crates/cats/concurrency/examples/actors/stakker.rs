//! An actor with the `stakker` crate: a counter that processes messages.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
use stakker::*;

// Define the messages that the actor can process.
// `Increment` increases the counter, while `GetValue` prints it.
/// A message that a [`Counter`] can process.
#[derive(Clone, Debug)]
enum MyMessage {
    /// Adds the given number to the count.
    Increment(u32),
    /// Prints the current count.
    GetValue,
}

/// An actor that holds a running total.
struct Counter {
    /// The running total.
    count: u32,
}

impl Counter {
    // Handle a single message and update or report state.
    /// Handles a single message by updating or reporting the count.
    fn handle_message(&mut self, message: MyMessage) {
        match message {
            MyMessage::Increment(amount) => {
                // Increase the internal counter by the provided amount.
                self.count += amount;
            }
            MyMessage::GetValue => {
                // Print the current value of the counter.
                println!("Current value: {}", self.count);
            }
        }
    }
}

impl Actor for Counter {
    type Msg = MyMessage;

    // This method is invoked by the Stakker runtime when an actor receives a
    // message.
    fn recv(
        &mut self,
        _ctx: &mut Context<Self::Msg>,
        msg: Self::Msg,
        _sender: Sender,
    ) {
        self.handle_message(msg);
    }
}

/// Spawns a [`Counter`], sends it messages and runs the system.
fn main() {
    // Create the actor system.
    let mut system = Stakker::new();

    // Spawn a `Counter` actor with an initial count of 0.
    let counter = system.spawn(Counter { count: 0 }, |counter, message| {
        counter.handle_message(message)
    });

    // Send a few messages to the actor.
    // These are processed asynchronously by the actor system.
    counter.send(MyMessage::Increment(5));
    counter.send(MyMessage::Increment(10));
    counter.send(MyMessage::GetValue);
    counter.send(MyMessage::Increment(2));
    counter.send(MyMessage::GetValue);

    // Run the actor system until no work remains.
    system.run(());
}
// ANCHOR_END: example

// TODO add to a chapter on actors with stakker
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
