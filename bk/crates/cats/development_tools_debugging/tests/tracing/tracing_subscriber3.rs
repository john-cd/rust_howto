// ANCHOR: example
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

fn main() {
    // Create a formatting layer that does not include the target.
    let fmt_layer = fmt::layer().with_target(false);
    // Create a filter layer that reads the filter from the environment
    // variable, or defaults to "info".
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    // Initialize tracing.
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
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
// [review NOW](https://github.com/john-cd/rust_howto/issues/161)
