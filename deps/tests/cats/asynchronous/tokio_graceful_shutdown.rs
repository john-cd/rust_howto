// ANCHOR: example
use tokio::time::sleep;
use tokio::time::Duration;
use tokio_graceful_shutdown::SubsystemBuilder;
use tokio_graceful_shutdown::SubsystemHandle;
use tokio_graceful_shutdown::Toplevel;

async fn countdown() {
    for i in (1..=3).rev() {
        tracing::info!("Shutting down in: {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}

async fn countdown_subsystem(
    subsys: SubsystemHandle,
) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    // Init logging
    tracing_subscriber::fmt()
        .init();

    // Setup and execute subsystem tree
    Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("Countdown", countdown_subsystem));
    })
    // Signals the Toplevel object to listen for SIGINT/SIGTERM/Ctrl+C
    .catch_signals()
    // Collects all the return values of the subsystems, determines the global error state
    .handle_shutdown_requests(Duration::from_millis(1000))
    .await
    .map_err(|e| e.into())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
