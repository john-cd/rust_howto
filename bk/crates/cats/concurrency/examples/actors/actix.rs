#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A minimal `actix` actor that handles a `Ping` message.

use actix::prelude::*;

// Define a message named `Ping`.
// Messages are used to communicate with actors.
/// A message sent to [`MyActor`].
struct Ping;

// Implement the `Message` trait for `Ping`.
// This trait is required for types that will be sent as messages to actors.
impl Message for Ping {
    // Define the result type for this message.
    // In this case, the message doesn't return any value, so it's `()`.
    type Result = ();
}

// Define an actor named `MyActor`.
// Actors are the fundamental units of concurrency in Actix.
/// An actor that handles [`Ping`] messages.
struct MyActor;

// Implement the `Actor` trait for `MyActor`.
impl Actor for MyActor {
    // Define the context type for this actor.
    // `Context<Self>` is the standard context type for actors.
    type Context = Context<Self>;
}

// Implement the `Handler` trait for `MyActor` to handle the `Ping` message.
// This trait defines how the actor should respond to a specific message type.
impl Handler<Ping> for MyActor {
    // Define the result type for this handler.
    // In this case, the handler doesn't return any value, so it's `()`.
    type Result = ();

    // Define the handler function for the `Ping` message.
    fn handle(&mut self, _msg: Ping, _ctx: &mut Context<Self>) {
        println!("Received Ping message");
    }
}

/// Starts [`MyActor`], sends it a [`Ping`] and runs the actor system.
fn main() {
    let system = System::new();

    // Actors must be started within a running system, hence `block_on`:
    system.block_on(async {
        // Start the actor:
        let addr = MyActor.start();

        // Send a Ping message to the actor and wait for it to be handled:
        addr.send(Ping).await.unwrap();

        // Ask the system to stop:
        System::current().stop();
    });

    // Run the system until it stops:
    system.run().unwrap();
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
