// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Graceful shutdown of nested subsystems with `tokio_graceful_shutdown`.
//!
//! A graceful shutdown stops accepting new work, lets in-flight work finish and
//! releases resources, instead of killing the process. Subsystems are started
//! as a tree and stopped together on Ctrl-C or SIGTERM, with a timeout in case
//! one hangs.

// The `tokio_graceful_shutdown` crate provides utilities for coordinating
// graceful shutdown in Tokio services.
//
// - Listen for shutdown requests from within subsystems.
// - Initiate shutdown manually from a subsystem.
// - Handle SIGINT/SIGTERM/Ctrl+C, subsystem failures, and panics.
// - Apply a clean shutdown procedure with timeout and error propagation.
// - Nest subsystems.
// - Shut down a selected subtree.

use tokio::time::Duration;
use tokio_graceful_shutdown::ErrorAction;
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
        () = subsys.on_shutdown_requested() => {
            tracing::info!("Countdown cancelled.");
        },
        () = countdown() => {
            subsys.request_shutdown(); // Triggers a shutdown of the entire subsystem tree.
        }
    };
    Ok(())
}

/// A subsystem that starts a countdown and ends when it is done.
async fn nested_subsystem(subsys: &mut SubsystemHandle) -> anyhow::Result<()> {
    tracing::info!("Nested subsystem started.");
    subsys.on_shutdown_requested().await;
    tracing::info!("Nested subsystem stopped.");
    Ok(())
}

/// A subsystem that panics, to show how the shutdown handles it.
async fn panic_subsystem(_subsys: &mut SubsystemHandle) -> anyhow::Result<()> {
    tracing::info!("Panic subsystem started.");
    tokio::time::sleep(Duration::from_millis(50)).await;
    panic!("Panic subsystem panicked!")
}

/// A subsystem that starts the other subsystems as its children.
async fn parent_subsystem(subsys: &mut SubsystemHandle) -> anyhow::Result<()> {
    tracing::info!("Parent subsystem started.");
    let nested =
        subsys.start(SubsystemBuilder::new("Nested", nested_subsystem));
    let _panic_sub =
        subsys.start(SubsystemBuilder::new("Panic", panic_subsystem));

    // Shut down only the nested subsystem while the parent continues running.
    tokio::time::sleep(Duration::from_millis(100)).await;
    tracing::info!("Shutting down nested subsystem...");
    nested.change_failure_action(ErrorAction::CatchAndLocalShutdown);
    nested.change_panic_action(ErrorAction::CatchAndLocalShutdown);
    nested.initiate_shutdown();
    let _ = nested.join().await;
    tracing::info!("Nested subsystem shut down.");

    subsys.on_shutdown_requested().await;
    tracing::info!("Parent subsystem stopped.");
    Ok(())
}

/// Starts the subsystems under a shutdown handler and waits for them.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // Set up and run the subsystem tree.
    // The `Toplevel` object represents the root object of the subsystem tree.
    let _ = Toplevel::new(async |s: &mut SubsystemHandle| {
        // Register and start a new subsystem.
        // The provided `SubsystemHandle` object enables the subsystem to start
        // nested subsystems, to react to shutdown requests or to
        // initiate a shutdown.
        s.start(SubsystemBuilder::new("Countdown", countdown_subsystem));
        s.start(SubsystemBuilder::new("Parent", parent_subsystem));
    })
    // Listen for SIGINT, SIGTERM, or Ctrl+C and start shutdown when one
    // arrives.
    .catch_signals()
    // Collect subsystem results and determine the overall error state.
    .handle_shutdown_requests(Duration::from_millis(400))
    .await;
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the tokio graceful shutdown example through its entry point.
    #[test]
    fn test_tokio_graceful_shutdown_example_entry_point() -> anyhow::Result<()>
    {
        main()?;
        Ok(())
    }

    #[tokio::test(start_paused = true)]
    async fn countdown_takes_three_ticks() {
        let start = tokio::time::Instant::now();
        countdown().await;
        assert_eq!(start.elapsed(), Duration::from_millis(300));
    }
}
