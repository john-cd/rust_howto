// ANCHOR: example
use log::Level;
use log::LevelFilter;
use log::Metadata;
use log::Record;
use log::SetLoggerError;

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

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

fn main() -> Result<(), SetLoggerError> {
    log::set_logger(&CONSOLE_LOGGER)?;
    log::set_max_level(LevelFilter::Info);

    log::info!("hello log");
    log::warn!("warning");
    log::error!("oops");
    Ok(())
}
// ANCHOR_END: example

use rusty_fork::rusty_fork_test;
// Runs in a separate process
rusty_fork_test! {
    #[test]
    fn test() {
        main().unwrap();
    }
}
