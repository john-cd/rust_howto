use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[test]
fn test() {
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
