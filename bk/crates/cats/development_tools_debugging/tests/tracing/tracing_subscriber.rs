// ANCHOR: example
fn main() {
    // Filter events at runtime using the value
    // of the RUST_LOG environment variable:
    // for example, RUST_LOG=debug,my_crate=trace
    tracing_subscriber::fmt::init();

    // This is equivalent to:
    // tracing_subscriber::fmt()
    // .with_env_filter(EnvFilter::from_default_env())
    // .init();

    tracing::info!("tracing configured!");
    println!("Done.")
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
// println!
// set to noplayground
// default level?

// [tracing_subscriber:  (P1)](https://github.com/john-cd/rust_howto/issues/159)
