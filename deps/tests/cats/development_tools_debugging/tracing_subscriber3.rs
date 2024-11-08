// ANCHOR: example
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

fn main() {
    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

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
