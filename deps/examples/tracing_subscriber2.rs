use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "myproj=debug,axum=debug,tower_http=debug,mongodb=debug".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
