#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to create a custom logger.
//!
//! The custom logger will print log messages to the console.

use log::Level;
use log::LevelFilter;
use log::Metadata;
use log::Record;
use log::SetLoggerError;

/// The custom logger.
static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

/// A logger that prints the records to the console.
struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

/// Installs the custom logger and emits log messages at several levels.
fn main() -> Result<(), SetLoggerError> {
    log::set_logger(&CONSOLE_LOGGER)?;
    log::set_max_level(LevelFilter::Info);

    log::info!("hello log");
    log::warn!("warning");
    log::error!("oops");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), SetLoggerError> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the log custom logger example through its entry point.
    #[test]
    fn test_log_custom_logger_example_entry_point() {
        main().expect("example entry point should succeed");
    }
}
