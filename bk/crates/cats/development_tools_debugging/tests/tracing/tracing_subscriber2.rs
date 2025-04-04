// ANCHOR: example
//! This example demonstrates how to configure tracing with `tracing-subscriber`.
//! It sets up a subscriber that logs to the console and filters logs based on the `RUST_LOG` environment variable.
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout)) // Logs to the console
        .with(tracing_subscriber::filter::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "myproj=debug,axum=debug,tower_http=debug,mongodb=debug".into()
            }),
        ))
        .init();
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
// [review; does it print? NOW](https://github.com/john-cd/rust_howto/issues/160)
