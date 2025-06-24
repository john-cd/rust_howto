#![allow(dead_code)]
// ANCHOR: example
/// Demonstrates how to configure the tracing subscriber
/// to change the logging level.
fn main() {
    // Change the default logging level:
    tracing_subscriber::fmt()
        // Sets the maximum verbosity level.
        .with_max_level(tracing::Level::DEBUG)
        // Sets this subscriber to be the global trace collector for this application.
        .init();
    // You may use `try_init()` instead.

    tracing::debug!("tracing configured!");
}
// ANCHOR_END: example

use rusty_fork::rusty_fork_test;

// Runs in a separate process.
rusty_fork_test! {
    #[test]
    fn test() {
        main();
    }
}
