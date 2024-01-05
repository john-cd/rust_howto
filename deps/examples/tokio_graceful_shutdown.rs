use tokio::time::{sleep, Duration};
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle, Toplevel};

async fn countdown() {
    for i in (1..=5).rev() {
        tracing::info!("Shutting down in: {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}

async fn countdown_subsystem(subsys: SubsystemHandle) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Init logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // Setup and execute subsystem tree
    Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("Countdown", countdown_subsystem));
    })
    .catch_signals() // signals the Toplevel object to listen for SIGINT/SIGTERM/Ctrl+C
    .handle_shutdown_requests(Duration::from_millis(1000)) // collects all the return values of the subsystems, determines the global error state
    .await
    .map_err(|e| e.into())
}
