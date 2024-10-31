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
}

// TODO
use rusty_fork::rusty_fork_test;

rusty_fork_test! {
#[test]
fn test() {
    main();
}
}
