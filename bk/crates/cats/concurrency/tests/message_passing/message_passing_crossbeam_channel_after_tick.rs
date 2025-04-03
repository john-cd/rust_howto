// ANCHOR: example
use std::time::Duration;
use std::time::Instant;

use crossbeam_channel::after;
use crossbeam_channel::select;
use crossbeam_channel::tick;

/// This example demonstrates the use of `crossbeam_channel`'s `tick` and `after` functions
/// to create channels that deliver messages periodically and after a certain duration, respectively.
fn main() {
    let start = Instant::now();
    // Create a channel that delivers messages periodically every 50 milliseconds.
    let ticker = tick(Duration::from_millis(50));

    // Create a channel that delivers a single message after 1 second.
    // This acts as a timeout mechanism.
    let timeout = after(Duration::from_secs(1));

    loop {
        // `select` wait until any one of the channels becomes ready and
        // execute it.
        select! {
            recv(ticker) -> _ => println!("elapsed: {:?}", start.elapsed()),
            recv(timeout) -> _ => break,
            // or use: default(Duration::from_millis(1000)) => break,
        }
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
