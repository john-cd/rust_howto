#![allow(dead_code)]
// ANCHOR: example
use tracing::Level;
use tracing::span;

fn main() {
    // Create a new span named "my_span" with the TRACE level.
    // Spans are used to represent a period of time in the execution of a
    // program.
    let span = span!(Level::TRACE, "my_span");
    {
        // Current lexical scope.
        let _guard = span.enter();
        println!(
            "`enter` returns a RAII guard, which, when dropped, exits the span."
        );
        tracing::info!(
            "Any trace events that occur here will occur within the span."
        )
    }
    tracing::info!("Dropping the guard exits the span.");
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
