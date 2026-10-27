#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! An actor with the `ractor` crate: a greeter that counts the greetings it
//! handles.

#![allow(unexpected_cfgs)]

use anyhow::Context;
use ractor::Actor;
use ractor::ActorProcessingErr;
use ractor::ActorRef;
use tokio::sync::oneshot;

// `ractor` is a pure-Rust actor framework, inspired from Erlang's gen_server.
// <https://slawlor.github.io/ractor>

// `ractor` gives a set of generic primitives and helps automate the actor
// supervision tree and management of actors along with traditional actor
// message processing logic. It supports both the `tokio` and `async-std`
// runtime.

// Add to your `Cargo.toml`:
// ```toml
// [dependencies]
// ractor = "0.16"
// ```

// Message type that the actor accepts:
/// A message that asks the greeter to greet someone and return a reply.
struct Greet {
    /// Name to include in the greeting.
    name: String,
    /// One-use channel for the greeting reply.
    reply_to: oneshot::Sender<String>,
}

#[cfg(feature = "cluster")]
impl ractor::Message for Greet {}

// Main actor struct:
/// An actor that greets and counts.
struct Greeter;

// Inner state of the actor:
/// The state of a [`Greeter`].
struct GreeterState {
    /// Number of greetings handled so far.
    count: u64,
}

impl ractor::Actor for Greeter {
    // Startup initialization args:
    type Arguments = ();
    // Actor's message type:
    type Msg = Greet;
    // Optional internal state:
    type State = GreeterState;

    // Invoked when an actor is being started by the system:
    // creates the state, starts internal processing...
    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _args: (),
    ) -> Result<Self::State, ActorProcessingErr> {
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
        let reply =
            format!("Hello, {}! (greeting #{})", message.name, state.count);
        // A caller may cancel a request after sending it; the actor can still
        // process the message and stop normally.
        let _ = message.reply_to.send(reply);

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
        println!("Greeter handled {} greeting(s).", state.count);
        Ok(())
    }
}

/// Sends one greeting request and returns the actor's reply.
///
/// # Errors
/// Returns an error if the actor cannot start, receive its message, reply, or
/// join successfully.
async fn run_actor() -> anyhow::Result<String> {
    let (actor, handle) = Actor::spawn(Some("Greeter".to_owned()), Greeter, ())
        .await
        .context("starting the greeter actor")?;

    let (reply_to, reply_from) = oneshot::channel();
    actor
        .send_message(Greet {
            name: "Rust".to_owned(),
            reply_to,
        })
        .context("sending a greeting to the actor")?;

    let reply = reply_from
        .await
        .context("greeter actor stopped before replying")?;
    handle.await.context("joining the greeter actor task")?;
    Ok(reply)
}

/// Runs the asynchronous Ractor example on a Tokio runtime.
///
/// # Errors
/// Returns an error if the actor cannot complete its message exchange.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{}", run_actor().await?);
    Ok(())
}

// ANCHOR_END: example

/// Runs the Ractor actor example.
///
/// # Errors
/// Returns an error if the actor cannot complete its message exchange.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the greeter replies with the requested name.
    #[tokio::test]
    async fn greeter_replies_to_message() -> anyhow::Result<()> {
        assert_eq!(run_actor().await?, "Hello, Rust! (greeting #1)");
        Ok(())
    }
}
