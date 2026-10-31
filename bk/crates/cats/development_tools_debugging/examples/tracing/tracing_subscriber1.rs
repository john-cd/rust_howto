// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates how to configure the tracing subscriber
//! to change the logging level.
//!
//! In `tracing`, a subscriber decides what happens to the events and spans.
//! `tracing_subscriber::fmt` prints them as text, with level filtering from an
//! environment variable.

/// Demonstrates how to configure the tracing subscriber
/// to change the logging level.
fn main() {
    // Change the default logging level:
    tracing_subscriber::fmt()
        // Sets the maximum verbosity level.
        .with_max_level(tracing::Level::DEBUG)
        // Sets this subscriber to be the global trace collector for this
        // application.
        .init();
    // You may use `try_init()` instead.

    tracing::debug!("tracing configured!");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isolated;

    /// Runs the tracing subscriber1 example through its entry point.
    #[test]
    fn test_tracing_subscriber1_example_entry_point() {
        isolated::run(
            module_path!(),
            "test_tracing_subscriber1_example_entry_point",
            || {
                main();
            },
        );
    }

    // output-only: the example installs the process-wide subscriber, which can
    // be set once per process, so its output cannot be captured here.
}
