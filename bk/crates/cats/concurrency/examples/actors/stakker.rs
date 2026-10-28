#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! An actor with the `stakker` crate: a counter that processes messages.

use std::sync::mpsc;
use std::time::Instant;

use anyhow::Context;
use stakker::*;

/// An actor that holds a running total.
struct Counter {
    /// The running total.
    count: u32,
}

impl Counter {
    /// Creates a counter actor with an initial value of zero.
    fn init(_cx: CX![]) -> Option<Self> {
        Some(Self { count: 0 })
    }

    /// Adds an amount and returns the updated total to the caller.
    fn increment(&mut self, _cx: CX![], amount: u32, reply_to: Ret<u32>) {
        self.count += amount;
        ret!([reply_to], self.count);
    }
}

/// Sends two increments and returns the replies from the counter actor.
///
/// # Errors
/// Returns an error if the actor does not reply to either increment.
fn run_actor() -> anyhow::Result<(u32, u32)> {
    let mut system = Stakker::new(Instant::now());
    let stakker = &mut system;
    let counter = actor!(stakker, Counter::init(), ret_nop!());
    let (reply_sender, reply_receiver) = mpsc::channel();

    let first_reply_sender = reply_sender.clone();
    call!(
        [counter],
        increment(
            5,
            ret_do!(move |value: Option<u32>| {
                if let Some(value) = value {
                    first_reply_sender.send(value).expect(
                        "the first reply receiver stays open until delivery",
                    );
                }
            })
        )
    );
    call!(
        [counter],
        increment(
            10,
            ret_do!(move |value: Option<u32>| {
                if let Some(value) = value {
                    reply_sender.send(value).expect(
                        "the second reply receiver stays open until delivery",
                    );
                }
            })
        )
    );

    stakker.run(Instant::now(), false);
    let first = reply_receiver.recv().context(
        "counter actor stopped before replying to the first increment",
    )?;
    let second = reply_receiver.recv().context(
        "counter actor stopped before replying to the second increment",
    )?;
    Ok((first, second))
}

/// Runs the Stakker counter example.
///
/// # Errors
/// Returns an error if the actor does not reply to either increment.
fn main() -> anyhow::Result<()> {
    let (first, second) = run_actor()?;
    println!("Totals: {first}, {second}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the Stakker actor example.
///
/// # Errors
/// Returns an error if the actor does not reply to either increment.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that Stakker replies with each updated counter total.
    #[test]
    fn actor_replies_with_updated_totals() -> anyhow::Result<()> {
        assert_eq!(run_actor()?, (5, 15));
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
