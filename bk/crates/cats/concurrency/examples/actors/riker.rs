#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! This example demonstrates a simple `riker` actor that receives a message.

use anyhow::Context as AnyhowContext;
use anyhow::Result;
use riker::actors::*;

/// A message that the actor receives.
#[derive(Clone, Debug)]
pub struct Ping;

/// A pong message handled by the actor.
#[derive(Clone, Debug)]
pub struct Pong;

/// An actor that handles ping and pong messages.
#[actor(Ping, Pong)]
#[derive(Default)]
struct PingPongActor;

impl Actor for PingPongActor {
    type Msg = PingPongActorMsg;

    fn recv(
        &mut self,
        ctx: &Context<Self::Msg>,
        msg: Self::Msg,
        sender: Sender,
    ) {
        self.receive(ctx, msg, sender);
    }
}

impl Receive<Ping> for PingPongActor {
    type Msg = PingPongActorMsg;

    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        _msg: Ping,
        _sender: Sender,
    ) {
        println!("Received Ping message");
    }
}

impl Receive<Pong> for PingPongActor {
    type Msg = PingPongActorMsg;

    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        _msg: Pong,
        _sender: Sender,
    ) {
        println!("Received Pong message");
    }
}

/// Starts an actor system, creates an actor and sends it a message.
///
/// # Errors
/// Returns an error if the actor system or actor cannot be created.
fn main() -> Result<()> {
    // Build and start the actor system.
    let sys = SystemBuilder::new()
        .name("ping-pong")
        .create()
        .context("creating the Riker actor system")?;

    // Create an actor instance in the system.
    let actor = sys
        .actor_of::<PingPongActor>("ping-pong-actor")
        .context("creating the ping-pong actor")?;

    // Send a Ping message to the actor.
    actor.tell(Ping, None);
    actor.tell(Pong, None);

    // Sleep briefly to allow the actor to process the message before the
    // program exits.
    std::thread::sleep(std::time::Duration::from_millis(100));
    Ok(())
}
// ANCHOR_END: example

/// Runs the Riker actor example.
///
/// # Errors
/// Returns an error if the actor system or actor cannot be created.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the actor system starts and receives a message.
    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
