// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates basic `mio` event polling with a `Waker`.

use std::time::Duration;

use mio::Events;
use mio::Poll;
use mio::Token;
use mio::Waker;

/// The token that identifies the events sent by the waker.
const WAKE_TOKEN: Token = Token(0);

/// Wakes up a polling instance and handles the event.
///
/// # Errors
/// Returns an error if polling or waking the instance fails.
fn main() -> std::io::Result<()> {
    // Create a new polling instance and event storage.
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(8);

    // Construct a waker that can notify the poll instance.
    let waker = Waker::new(poll.registry(), WAKE_TOKEN)?;

    // Signal the poller using the waker. In real code this could happen
    // from another thread or asynchronous context.
    waker.wake()?;

    // Wait for events for up to 100ms.
    poll.poll(&mut events, Some(Duration::from_millis(100)))?;

    // Inspect each event, looking for the wake token.
    for event in &events {
        if event.token() == WAKE_TOKEN {
            println!("Received wake event: {event:?}");
        }
    }

    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the mio example through its entry point.
    #[test]
    fn test_mio_example_entry_point() -> std::io::Result<()> {
        main()
    }

    #[test]
    fn waker_produces_a_readable_event() -> std::io::Result<()> {
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(8);
        let waker = Waker::new(poll.registry(), WAKE_TOKEN)?;
        waker.wake()?;
        poll.poll(&mut events, Some(Duration::from_millis(500)))?;
        let tokens: Vec<Token> =
            events.iter().map(mio::event::Event::token).collect();
        assert_eq!(tokens, [WAKE_TOKEN]);
        Ok(())
    }

    #[test]
    fn poll_times_out_without_events() -> std::io::Result<()> {
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(8);
        poll.poll(&mut events, Some(Duration::from_millis(10)))?;
        assert!(events.is_empty());
        Ok(())
    }
}
