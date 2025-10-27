#![allow(dead_code)]
// ANCHOR: example
// The `tokio_graceful_shutdown` crate provides utility functions to perform a
// graceful shutdown on tokio-rs based services.
//
// - Listening for shutdown requests from within subsystems
// - Manual shutdown initiation from within subsystems
// - Automatic shutdown on SIGINT/SIGTERM/Ctrl+C, Subsystem failure, Subsystem
//   panic
// - Clean shutdown procedure with timeout and error propagation
// - Subsystem nesting
// - Partial shutdown of a selected subsystem tree

use tokio::time::Duration;
use tokio_graceful_shutdown::SubsystemBuilder;
use tokio_graceful_shutdown::SubsystemHandle;
use tokio_graceful_shutdown::Toplevel;

/// Counts down from 3 to 1, logging each number.
async fn countdown() {
    for i in (1..=3).rev() {
        tracing::info!("Shutting down in {i}");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// An example subsystem that either counts down or waits for a shutdown
/// request.
async fn countdown_subsystem(
    subsys: &mut SubsystemHandle,
) -> anyhow::Result<()> {
    tracing::info!("Starting countdown ...");
    tokio::select! {
        // `on_shutdown_requested` waits for the shutdown mode to be triggered.
        // Most often, it will be used in `tokio::select` statements to cancel other code as soon as the shutdown is requested.
        _ = subsys.on_shutdown_requested() => {
            tracing::info!("Countdown cancelled.");
        },
        _ = countdown() => {
            subsys.request_shutdown(); // Triggers a shutdown of the entire subsystem tree.
        }
    };
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init logging.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // Setup and execute the subsystem tree:
    // The `Toplevel` object represents the root object of the subsystem tree.
    Toplevel::new(async |s: &mut SubsystemHandle| {
        // Register and start a new subsystem.
        // The provided `SubsystemHandle` object enables the subsystem to start nested subsystems,
        // to react to shutdown requests or to initiate a shutdown.
        s.start(SubsystemBuilder::new("Countdown", countdown_subsystem));
    })
    // Signals the `Toplevel` object to listen for SIGINT / SIGTERM / Ctrl + C and and initiate a shutdown thereafter:
    .catch_signals()
    // Collects all the return values of the subsystems, determines the global error state:
    .handle_shutdown_requests(Duration::from_millis(400))
    .await
    .map_err(Into::into)
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// TODO expand
