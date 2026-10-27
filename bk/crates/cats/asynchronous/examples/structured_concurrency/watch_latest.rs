#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Choosing a Tokio channel: `watch` keeps only the latest value (config,
//! state), `broadcast` delivers every event but receivers can lag, and `mpsc`
//! is a work queue where each message goes to one consumer.

use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::mpsc;
use tokio::sync::watch;

/// A configuration published to subscribers.
#[derive(Debug, PartialEq)]
struct Config {
    version: u32,
}

/// `watch`: subscribers see the latest value; intermediate values may be
/// skipped.
async fn watch_latest() {
    let (tx, mut rx) = watch::channel(Arc::new(Config { version: 1 }));

    // `send_replace` never fails, even if there are no receivers, and returns
    // the previous value.
    let old = tx.send_replace(Arc::new(Config { version: 2 }));
    assert_eq!(old.version, 1);
    tx.send_replace(Arc::new(Config { version: 3 }));

    // The subscriber missed version 2 but sees the latest one.
    assert!(rx.changed().await.is_ok());
    assert_eq!(rx.borrow_and_update().version, 3);
}

/// `broadcast`: every event is delivered, unless the receiver falls behind.
async fn broadcast_events() {
    let (tx, mut rx) = broadcast::channel(2);
    for event in 1..=5 {
        // `send` fails only if there are no receivers.
        assert!(tx.send(event).is_ok());
    }
    // The buffer holds the two most recent events; three were dropped.
    assert_eq!(rx.recv().await, Err(RecvError::Lagged(3)));
    assert_eq!(rx.recv().await, Ok(4));
    assert_eq!(rx.recv().await, Ok(5));
}

/// `mpsc`: a work queue; each message is received once.
async fn work_queue() {
    let (tx, mut rx) = mpsc::channel(8);
    for job in 1..=3 {
        assert!(tx.send(job).await.is_ok());
    }
    drop(tx);
    let mut jobs = Vec::new();
    while let Some(job) = rx.recv().await {
        jobs.push(job);
    }
    assert_eq!(jobs, vec![1, 2, 3]);
}

/// Runs the three demonstrations.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    watch_latest().await;
    broadcast_events().await;
    work_queue().await;
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks watch state, broadcasts, and bounded message delivery.
    /// Runs the watch latest example through its entry point.
    #[test]
    fn test_watch_latest_example_entry_point() {
        main();
    }
}
