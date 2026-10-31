// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates how to write logs to multiple destinations (in this instance,
//! stdout and a file) simultaneously.
//!
//! A subscriber can print events as JSON, so that log systems such as
//! Elasticsearch or Loki can index each field instead of parsing text.

use std::io;

use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt as _;

/// Demonstrates how to write logs to multiple destinations (in this instance,
/// stdout and a file) simultaneously.
fn main() {
    let dir = tempfile::tempdir().expect("Failed to create tempdir");

    // Hourly rotating file appender that writes to
    // /{dir}/example.log.YYYY-MM-DD-HH:
    let file_appender = tracing_appender::rolling::hourly(dir, "example.log");

    // Non-Blocking rolling file appender:
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let collector = tracing_subscriber::registry()
        .with(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::TRACE.into()),
        )
        .with(fmt::layer().with_writer(io::stdout))
        .with(fmt::layer().with_writer(non_blocking));
    tracing::subscriber::set_global_default(collector)
        .expect("Unable to set a global collector");

    // Test it.
    tracing::debug!("tracing configured!");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the tracing subscriber2b example through its entry point.
    #[test]
    fn test_tracing_subscriber2b_example_entry_point() {
        main();
    }

    // output-only: the example installs the process-wide subscriber, which can
    // be set once per process, so its output cannot be captured here.
}
