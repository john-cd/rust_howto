// ANCHOR: example
use tracing_subscriber::fmt;

/// This example demonstrates configuring a custom event formatter for the `fmt`
/// subscriber.
fn main() {
    // Configure a custom event formatter
    let format = fmt::format()
        .with_level(false) // Don't include levels in the formatted output
        .with_target(false) // Don't include targets
        .with_thread_ids(true) // Include the thread ID of the current thread
        .with_thread_names(true) // Include the name of the current thread
        .compact(); // Use the `Compact` formatting style.

    // Create a `fmt` subscriber that uses our custom event format, and
    // set it as the default.
    tracing_subscriber::fmt().event_format(format).init();
}
// ANCHOR_END: example

use rusty_fork::rusty_fork_test;

// Runs in a separate process
rusty_fork_test! {
    #[test]
    fn test() {
        main();
    }
}
// [review NOW](https://github.com/john-cd/rust_howto/issues/162)
