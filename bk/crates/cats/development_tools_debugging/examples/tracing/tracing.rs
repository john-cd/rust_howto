#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates the basic usage of the `tracing` crate for logging
//! events.
//!
//! It showcases how to use different log levels (trace, debug, info, warn,
//! error) and how to add structured data to log messages.

use tracing::Level;
use tracing::debug;
use tracing::error;
use tracing::event;
use tracing::info;
use tracing::trace;
use tracing::warn;

fn main() {
    // Basic logging of a message at various severity levels.
    event!(Level::INFO, "something happened");
    error!("error!");
    warn!("warning!");
    info!("info!");
    debug!("debug info!");
    trace!("trace info!");

    // Set the level and modify the event's target.
    // A target is a string that categorizes part of the system where the span
    // or event occurred. The tracing macros default to using the module path
    // where the span or event originated as the target, but it may be
    // overridden.
    event!(target: "app_events", Level::TRACE, "something has happened!");

    // Structured fields on events (and spans) are specified using the syntax
    // `field_name = field_value`. Fields are separated by commas.
    // Here, we record an event with two fields:
    event!(
        Level::INFO,
        answer = 42,
        question = "life, the universe, and everything"
    );

    // Unlike other fields, `message`'s shorthand initialization is just
    // the string itself.
    debug!(excitement = "yay!", "hello!");

    // String formatting; shorthand for `user = user`:
    let user = "ferris";
    event!(Level::TRACE, "login: {user}");

    // Records an event with a `struct` field using the `Debug` format.
    // Note the `?`: `my_struct` will be recorded
    // using its `fmt::Debug` implementation.
    let my_struct = S;
    event!(Level::TRACE, greeting = ?my_struct);
}

/// A simple struct for demonstration purposes.
#[derive(Debug)]
struct S;
// ANCHOR_END: example

use rusty_fork::rusty_fork_test;

// Runs in a separate process.
rusty_fork_test! {
    #[test]
    fn test() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
        main();
    }
}
