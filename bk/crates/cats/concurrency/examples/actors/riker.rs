// ANCHOR: example
//! This example demonstrates a simple `riker` actor that receives a message.

use riker::actors::*;

#[derive(Clone, Debug)]
struct Ping;

#[actor(PingPong)]
struct PingPongActor;

impl Actor for PingPongActor {
    type Msg = PingPongMsg;

    fn recv(&mut self, _ctx: &Context<Self::Msg>, msg: Self::Msg, _sender: Sender) {
        match msg {
            PingPongMsg::Ping => println!("Received Ping message"),
            PingPongMsg::Pong => println!("Received Pong message"),
        }
    }
}

fn main() {
    // Build and start the actor system.
    let sys = SystemBuilder::new().name("ping-pong").create().unwrap();

    // Create an actor instance in the system.
    let actor = sys.actor_of::<PingPongActor>("ping-pong-actor").unwrap();

    // Send a Ping message to the actor.
    actor.tell(PingPongMsg::Ping, None);

    // Sleep briefly to allow the actor to process the message before the program exits.
    std::thread::sleep(std::time::Duration::from_millis(100));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO add to a chapter on actors with riker
