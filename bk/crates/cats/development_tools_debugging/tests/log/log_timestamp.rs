// ANCHOR: example
use std::io::Write;

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
            writeln!( // Write the formatted log message to the buffer.
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

use rusty_fork::rusty_fork_test;

// Runs in a separate process
rusty_fork_test! {
    #[test]
    fn test() {
        main();
    }
}
