// ANCHOR: example
//! This example demonstrates basic `mio` event polling with a `Waker`.

use mio::{Events, Interest, Poll, Token, Waker};
use std::time::Duration;

const WAKE_TOKEN: Token = Token(0);

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
    for event in events.iter() {
        if event.token() == WAKE_TOKEN {
            println!("Received wake event: {:?}", event);
        }
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    main().unwrap();
}
// TODO add to a chapter on asynchronous programming with mio
