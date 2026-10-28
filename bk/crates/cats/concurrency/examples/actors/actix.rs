#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! An `actix` counter actor that replies with its updated total.

use actix::Actor;
use actix::Context as ActorContext;
use actix::Handler;
use actix::Message;
use actix::System;
use anyhow::Context;

/// A message that adds an amount to the counter.
struct Add(u64);

impl Message for Add {
    type Result = u64;
}

/// An actor that owns a running total.
struct Counter {
    /// The total after processing the last message.
    total: u64,
}

impl Actor for Counter {
    type Context = ActorContext<Self>;
}

impl Handler<Add> for Counter {
    type Result = u64;

    /// Adds the message amount and returns the updated total.
    fn handle(
        &mut self,
        message: Add,
        _context: &mut ActorContext<Self>,
    ) -> Self::Result {
        self.total += message.0;
        self.total
    }
}

/// Starts the counter and returns the replies to two messages.
///
/// # Errors
/// Returns an error if a message reply or the Actix system fails.
fn run_actor() -> anyhow::Result<(u64, u64)> {
    let system = System::new();
    let totals = system.block_on(async {
        let counter = Counter { total: 0 }.start();
        let first = counter.send(Add(2)).await.context(
            "counter actor stopped before replying to the first message",
        )?;
        let second = counter.send(Add(3)).await.context(
            "counter actor stopped before replying to the second message",
        )?;
        Ok::<_, anyhow::Error>((first, second))
    })?;

    System::current().stop();
    system.run().context("Actix system failed")?;
    Ok(totals)
}

/// Runs the Actix counter example.
///
/// # Errors
/// Returns an error if a message reply or the Actix system fails.
fn main() -> anyhow::Result<()> {
    let (first, second) = run_actor()?;
    println!("Totals: {first}, {second}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the Actix actor example.
///
/// # Errors
/// Returns an error if a message reply or the Actix system fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that Actix replies with each updated counter total.
    #[test]
    fn actor_replies_with_updated_totals() -> anyhow::Result<()> {
        assert_eq!(run_actor()?, (2, 5));
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
