// ANCHOR: example
//! This example demonstrates the basic usage of the `tracing` crate for logging events.
//!
//! It showcases how to use different log levels (trace, debug, info, warn, error)
//! and how to add structured data to log messages.

use tracing::Level;
use tracing::debug;
use tracing::error;
use tracing::event;
use tracing::info;
use tracing::trace;
use tracing::warn;

/// A simple struct for demonstration purposes.
#[derive(Debug)]
struct S;

fn main() {
    event!(Level::INFO, "something happened");
    error!("error!");
    warn!("warning!");
    info!("info!");
    debug!("debug info!");
    trace!("trace info!");

    event!(target: "app_events", Level::TRACE, "something has happened!");

    // Records an event with two fields (That also works for spans).
    event!(
        Level::INFO,
        answer = 42,
        question = "life, the universe, and everything"
    );

    // Unlike other fields, `message`'s shorthand initialization is just
    // the string itself.
    debug!(excitement = "yay!", "hello!");

    // Shorthand for `user = user`:
    let user = "ferris";
    event!(Level::TRACE, "login: {}", user);

    // Records an event with a struct field using the debug format.
    // Note the `?`: `my_struct` will be recorded
    // using its `fmt::Debug` implementation.
    let my_struct = S;
    event!(Level::TRACE, greeting = ?my_struct);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
