// ANCHOR: example
use env_logger::Target;

/// This example demonstrates how to configure the `env_logger` to output logs
/// to `Stdout`.
fn main() {
    // Create a new `env_logger` builder.
    env_logger::Builder::new().target(Target::Stdout).init();

    // Log an error message.
    log::error!("This error has been printed to Stdout");
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
