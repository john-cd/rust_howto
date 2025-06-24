#![allow(dead_code)]
// ANCHOR: example
use std::time::Duration;

use actix::prelude::*;

// Define a message named `Ping`.
// Messages are used to communicate with actors.
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

fn main() {
    let system = System::new();

    // Start the actor
    let addr = MyActor.start();

    // Send a Ping message to the actor
    addr.do_send(Ping);

    // Stop the system after a delay
    system.run().unwrap();
    std::thread::sleep(Duration::from_secs(1));
}
// ANCHOR_END: example

#[test]
#[ignore = "Needs review"]
fn test() {
    main();
}
// [finish; test fails: `spawn_local` called from outside of a `task::LocalSet` or LocalRuntime](https://github.com/john-cd/rust_howto/issues/682)
