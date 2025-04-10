// ANCHOR: example
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::fmt::time::UtcTime;

/// This example demonstrates customizing the formatting of log lines for spans
/// and events. See https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/index.html#customizing-formatters
fn main() {
    // Configure a custom event formatter.
    let format: Format<_, _> = fmt::format() // Returns the default configuration for an event formatter.
        .with_ansi(true)  // Enable ANSI terminal colors for formatted output.
        .with_file(true)  // Displays the event's source code file path.
        .with_line_number(true) // Displays the source code line number.
        // or: .with_source_location(true)
        .with_level(false) // Don't include event levels e.g. Error, Debug, etc.
        .with_target(false) // Don't include event targets.
        .with_thread_ids(true) // Include the thread ID of the current thread.
        .with_thread_names(true) // Include the name of the current thread.
        .without_time() // Do not emit timestamps with log messages.
        .with_timer(UtcTime::rfc_3339()) // Use the given timer for log message timestamps.
        .compact(); // Use the `Compact` formatting style.
    // Consider also: `.json()` or `.pretty()`

    // The default logging format, `Full`, includes all fields in each event and
    // its containing spans. The `Compact` logging format is intended to produce
    // shorter log lines; it displays each event's fields, along with fields
    // from the current span context, but other information is abbreviated.

    // Create a subscriber that uses our custom event format, and
    // set it as the default.
    // - `fmt()` is essentially shorthand for `SubscriberBuilder::default()`.
    // - `init()` installs this Subscriber as the global default.
    tracing_subscriber::fmt().event_format(format).init();

    // Test it out.
    tracing::error!("tracing configured!");
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
// FIXME cover same functions on fmt()
