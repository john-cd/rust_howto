// ANCHOR: example
use std::time::Duration;
use std::time::Instant;

use crossbeam_channel::after;
use crossbeam_channel::select;
use crossbeam_channel::tick;

fn main() {
    let start = Instant::now();
    // Channel that delivers messages periodically.
    let ticker = tick(Duration::from_millis(50));
    // Channel that delivers a single message
    // after a certain duration of time.
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
