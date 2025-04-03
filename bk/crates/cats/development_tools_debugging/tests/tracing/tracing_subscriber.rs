// ANCHOR: example
fn main() {
    // Initialize the tracing subscriber with a default format.
    //
    // This sets up the tracing infrastructure to output log messages
    // to the standard output. The format of these messages is determined
    // by the default settings of the `fmt` module.
    //
    // The `init()` function is responsible for setting the global
    // default subscriber.

    // Filter events at runtime using the value
    // of the RUST_LOG environment variable:
    // for example, `RUST_LOG=debug,my_crate=trace`.
    tracing_subscriber::fmt::init();

    // The above is equivalent to:
    // ```
    // tracing_subscriber::fmt()
    //     .with_env_filter(EnvFilter::from_default_env())
    //     .init();
    // ```
    // It also filters events at runtime using the value of the
    // `RUST_LOG` environment variable (e.g., `RUST_LOG=debug,my_crate=trace`).

    tracing::info!("tracing configured!");
    println!("Done.");
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
// [review;  println!; set to noplayground; default level?](https://github.com/john-cd/rust_howto/issues/159)
