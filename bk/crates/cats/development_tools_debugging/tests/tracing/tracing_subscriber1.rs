// ANCHOR: example
/// Demonstrates how to change the logging level.
fn main() {
    tracing_subscriber::fmt()
        // Sets the maximum verbosity level. In this case, enable everything.
        .with_max_level(tracing::Level::TRACE)
        // Sets this to be the default, global collector for this application.
        .init();

    // Test it.
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
