// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates the use of `crossbeam_channel`'s `tick` and
//! `after` functions to create channels that deliver messages periodically and
//! after a certain duration, respectively.

use std::time::Duration;
use std::time::Instant;

use crossbeam_channel::after;
use crossbeam_channel::select;
use crossbeam_channel::tick;

/// This example demonstrates the use of `crossbeam_channel`'s `tick` and
/// `after` functions to create channels that deliver messages periodically and
/// after a certain duration, respectively.
fn main() {
    let start = Instant::now();
    // Create a channel that delivers messages periodically every 5
    // milliseconds.
    let ticker = tick(Duration::from_millis(5));

    // Create a channel that delivers a single message after 20 milliseconds.
    // This acts as a timeout mechanism.
    let timeout = after(Duration::from_millis(20));
    let timeout_fired = loop {
        // `select` wait until any one of the channels becomes ready and
        // execute it.
        select! {
            recv(ticker) -> _ => println!("elapsed: {:?}", start.elapsed()),
            recv(timeout) -> _ => break true,
            // or use: default(Duration::from_millis(1000)) => break,
        }
    };
    assert!(timeout_fired);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the message passing crossbeam channel after tick example through
    /// its entry point.
    #[test]
    fn test_message_passing_crossbeam_channel_after_tick_example_entry_point() {
        main();
    }
}
