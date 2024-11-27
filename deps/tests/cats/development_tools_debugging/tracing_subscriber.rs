// ANCHOR: example
fn main() {
    // Filter events at runtime using the value
    // of the RUST_LOG environment variable:
    // for example, RUST_LOG=debug,my_crate=trace
    tracing_subscriber::fmt::init();
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
// TODO P1
