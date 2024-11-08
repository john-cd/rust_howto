// ANCHOR: example
use tracing_subscriber::fmt;

fn main() {
    // Configure a custom event formatter
    let format = fmt::format()
        .with_level(false) // Don't include levels in formatted output
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
