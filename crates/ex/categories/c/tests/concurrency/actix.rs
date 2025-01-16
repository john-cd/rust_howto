// ANCHOR: example
use std::time::Duration;

use actix::prelude::*;

// Define the messages
struct Ping;
impl Message for Ping {
    type Result = ();
}

// Define the actor
struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;
}

// Implement the handler for the Ping message
impl Handler<Ping> for MyActor {
    type Result = ();

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
fn test() {
    main();
}
// [P1](https://github.com/john-cd/rust_howto/issues/682)
