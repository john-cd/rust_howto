use tokio::time::Duration;
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle, Toplevel};

async fn countdown() {
    for i in (1..=3).rev() {
        tracing::info!("Shutting down in {i}");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn countdown_subsystem(subsys: &mut SubsystemHandle) -> anyhow::Result<()> {
    tracing::info!("Starting countdown ...");
    tokio::select! {
        _ = subsys.on_shutdown_requested() => {
            tracing::info!("Countdown cancelled.");
        },
        _ = countdown() => {
            subsys.request_shutdown();
        }
    };
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    Toplevel::new(async |s: &mut SubsystemHandle| {
        s.start(SubsystemBuilder::new("Countdown", countdown_subsystem));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_millis(400))
    .await
    .map_err(Into::into)
}
