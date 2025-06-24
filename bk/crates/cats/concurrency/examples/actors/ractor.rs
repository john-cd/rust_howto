#![allow(dead_code)]
// ANCHOR: example
#![allow(unexpected_cfgs)]

use ractor::Actor;
use ractor::ActorProcessingErr;
use ractor::ActorRef;
use ractor::cast;
use tokio::time::Duration;
use tokio::time::sleep;

// `ractor` is a pure-Rust actor framework, inspired from Erlang's gen_server.
// https://slawlor.github.io/ractor/

// `ractor` gives a set of generic primitives and helps automate the actor
// supervision tree and management of actors along with traditional actor
// message processing logic. It supports both the `tokio` and `async-std`
// runtime.

// Add to your `Cargo.toml`:
// [dependencies]
// ractor = "0.14"

// Message type that the actor accepts
#[derive(Debug, Clone)]
struct Greet(String);

#[cfg(feature = "cluster")]
impl ractor::Message for Greet {}

// Main actor struct
struct Greeter;

// Inner state of the actor
struct GreeterState {
    count: u64,
}

impl ractor::Actor for Greeter {
    // Startup initialization args
    type Arguments = ();
    // Actor's message type
    type Msg = Greet;
    // Optional internal state
    type State = GreeterState;

    // Invoked when an actor is being started by the system:
    // creates the state, starts internal processing...
    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        println!("Greeter actor is starting.");

        // Startup the event processing:
        // Actors communicate by passing messages to each other.
        cast!(myself, Greet("Hello!".to_string()))?;
        // Or: myself.send_message(Greet("Hello!".to_string()))?;

        // Create the initial state
        Ok(GreeterState { count: 0 })
    }

    // Handle incoming messages from the event processing loop.
    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        state.count += 1;
        println!("Received a message: {}", message.0);

        // In this case, we stop the actor after just one message:
        myself.stop(Some("Received greeting.".into()));
        Ok(())
    }

    // Invoked after an actor has been stopped to perform final cleanup.
    async fn post_stop(
        &self,
        _myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        println!("Greeter actor is stopping. Count: {}", state.count);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // Create a Greeter actor
    let (_actor, handle) =
        Actor::spawn(Some("Actor1".to_string()), Greeter, ())
            .await
            .expect("Failed to start the actor"); // panic in `pre_start()`

    handle.await.expect("Actor failed to exit properly");

    // Sleep for a bit to let the actor finish any remaining work
    sleep(Duration::from_millis(50)).await;
}

#[test]
fn test() {
    main();
}
// ANCHOR_END: example
