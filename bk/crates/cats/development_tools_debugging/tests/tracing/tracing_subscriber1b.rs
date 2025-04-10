// ANCHOR: example
use std::fs::File;

use tracing::instrument;
use tracing_subscriber::fmt::format::FmtSpan;

/// Demonstrates how to log in JSON format.
/// This is intended for production use with systems where structured logs are
/// consumed as JSON by analysis and viewing tools.
fn main() {
    let file = File::create("temp/my_log.json").unwrap();

    // Build a custom subscriber.
    let json_subscriber = tracing_subscriber::fmt()
        .json() // Log in JSON format.
        .with_writer(std::sync::Mutex::new(file)) // Log to the file.
        // Adjust the following configuration as needed.
        .with_max_level(tracing::Level::TRACE) // Log everything.
        .flatten_event(false) // Do not flatten event metadata.
        .with_current_span(true) // Include the current span in formatted events.
        .with_span_list(true) // Include a list (from root to leaf) of all currently entered spans in formatted events.
        .with_span_events(FmtSpan::FULL) // Events will be logged whenever a span is created, entered, exited, or closed.
        .with_thread_ids(true) // Include the thread ID of the current thread.
        .with_thread_names(true) // Include the thread name of the current thread.
        .with_ansi(false) // Disable ANSI terminal escape codes for colors and other text formatting.
        .finish();

    // Register it as the global default. Equivalent to
    // `json_subscriber.init()`.
    tracing::subscriber::set_global_default(json_subscriber)
        .expect("Setting tracing default failed.");

    tracing::debug!("Tracing configured!");

    // Calling this instrumented function will log span events, since
    // `with_span_events` is enabled above.
    a_function();
}

#[instrument]
fn a_function() {
    tracing::debug!("In a_function.");
}
// ANCHOR_END: example

use rusty_fork::rusty_fork_test;

// Runs in a separate process.
rusty_fork_test! {
    #[test]
    fn test() {
        main();
    }
}
