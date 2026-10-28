// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Publishes the latest state with `watch`, and contrasts it with `broadcast`.

use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::watch;

/// An application configuration.
#[derive(Debug, PartialEq)]
struct Config {
    /// The log level.
    level: &'static str,
}

/// Publishes three configurations before the subscribers look at them.
///
/// `watch` keeps only the latest value; `broadcast` queues events and
/// reports a lag when its buffer overflows.
async fn latest_and_first_event() -> (Arc<Config>, Result<u8, RecvError>) {
    let (tx, mut rx) = watch::channel(Arc::new(Config { level: "info" }));
    let (btx, mut brx) = broadcast::channel::<u8>(2);

    for (i, level) in ["warn", "debug", "trace"].into_iter().enumerate() {
        // `send_replace` never fails, even without receivers.
        let _previous = tx.send_replace(Arc::new(Config { level }));
        btx.send(u8::try_from(i).unwrap_or(0)).ok();
    }

    // `changed` returns once the value differs from what this receiver saw.
    rx.changed().await.ok();
    let latest = rx.borrow_and_update().clone();

    // The broadcast receiver missed the first event.
    let first = brx.recv().await;
    (latest, first)
}

/// Publishes the sample state changes and prints each subscriber result.
async fn example() {
    let (latest, first) = latest_and_first_event().await;
    println!("watch saw only {latest:?}; broadcast first recv: {first:?}");
}

/// Runs the example on a current-thread Tokio runtime.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    example().await;
}

// ANCHOR_END: example

/// Runs the example on a current-thread Tokio runtime.
///
/// # Errors
///
/// Returns an error if the Tokio runtime cannot be created.
pub fn run() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(example());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example entry point runs successfully.
    #[test]
    fn main_runs() -> anyhow::Result<()> {
        run()
    }

    /// Verifies `watch` coalesces updates and `broadcast` can lag.
    #[tokio::test]
    async fn watch_coalesces_broadcast_lags() {
        let (latest, first) = latest_and_first_event().await;
        assert_eq!(*latest, Config { level: "trace" });
        assert_eq!(first, Err(RecvError::Lagged(1)));
    }
}
