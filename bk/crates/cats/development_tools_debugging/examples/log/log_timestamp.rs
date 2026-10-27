#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to use `env_logger` to format log messages
//! with timestamps.

use std::io::Write as _;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

/// This example demonstrates how to use `env_logger` to format log messages
/// with timestamps.
fn main() {
    // Create a new logger builder.
    Builder::new()
        // Set the format for log messages.
        .format(|buf, record| {
            writeln!(
                // Write the formatted log message to the buffer.
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        // Set the log level filter to Info.
        .filter(None, LevelFilter::Info)
        .init();

    log::warn!("warn");
    log::info!("info");
    log::debug!("debug");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the log timestamp example through its entry point.
    #[test]
    fn test_log_timestamp_example_entry_point() {
        main();
    }
}
