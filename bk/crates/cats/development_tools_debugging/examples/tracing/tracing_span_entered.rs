#![allow(dead_code)]
// ANCHOR: example
use tracing::Level;
use tracing::span;

/// Demonstrates entering and exiting a span using `entered()` and `exit()`.
fn main() {
    // Create a span, entering it using `entered()`, thus consuming it and
    // returning a guard that will exit the span when dropped.
    {
        let _span = span!(Level::INFO, "Some span").entered();

        tracing::info!("Code here is within the span.");
    }

    tracing::info!("Code here is no longer within the span.");

    // Optionally, use `entered` and `exit` to get in and out of the span.
    {
        let span = span!(Level::TRACE, "Another span").entered();
        tracing::info!("Within the span.");

        // Explicitly exit the span, returning it.
        let span = span.exit();
        tracing::info!("No longer within the span.");

        // Re-enter the span.
        let _span = span.entered();
        tracing::info!("Within the span.");
    }
}
// ANCHOR_END: example
use rusty_fork::rusty_fork_test;

// Runs in a separate process.
rusty_fork_test! {
#[test]
    fn test() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
        main();
    }
}
