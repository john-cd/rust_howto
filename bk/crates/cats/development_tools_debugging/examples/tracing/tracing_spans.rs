#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Creating and entering spans with `tracing`.

use tracing::Level;
use tracing::span;

/// Creates a span, enters it and emits events inside.
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
        );
    }
    tracing::info!("Dropping the guard exits the span.");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the span example runs.
    /// Runs the tracing spans example through its entry point.
    #[test]
    fn test_tracing_spans_example_entry_point() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
        main();
    }
}
