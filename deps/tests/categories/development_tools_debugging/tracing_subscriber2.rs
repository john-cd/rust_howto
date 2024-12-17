// ANCHOR: example
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
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
// [tracing_subscriber2: does it print? (P0)](https://github.com/john-cd/rust_howto/issues/160)
