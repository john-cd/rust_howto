#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates basic usage of the `slog` logging crate.
//! `slog` is a structured, composable logging framework.
//!
//! The `Drain` trait is responsible for handling logging statements (Records)
//! from Loggers: filtering, modifying, formatting and writing the log records
//! into given destination(s).
//!
//! Add to your `Cargo.toml`, as needed:
//! ```toml
//! slog = "2.7" # or latest
//! slog-async = "2.7"
//! slog-term = "2.8"
//! slog-scope = "4.4"
//! ```
use slog::Drain;
use slog::o;

fn main() {
    // Create a terminal decorator.
    let decorator = slog_term::TermDecorator::new().build();
    // Also try: let decorator = slog_term::PlainDecorator::new(file);

    // Create a terminal drain.
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    // Also try: let drain =
    // slog_term::FullFormat::new(decorator).build().fuse();

    // Create an async drain.
    // Offloads processing to another thread.
    let drain = slog_async::Async::new(drain).build().fuse();

    // Create a root logger.
    let root_logger =
        slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));

    // Log some messages.
    // Messages behave like `format!`.
    // ; is used to separate message arguments and key value pairs.
    slog::info!(root_logger, "Logging with slog"; "example" => "simple");
    slog::warn!(root_logger, "A {} message", "warning");
    slog::error!(root_logger, "An error occurred: {msg}", msg = "it failed!");

    // Build a child logger.
    // Child logger inherits all existing key-value pairs from its parent and
    // supplements them with additional ones.
    let child_logger = root_logger.new(o!("key" => "value"));
    perform_some_logging(child_logger);

    // Using slog-scope for global logging:
    let _guard = slog_scope::set_global_logger(root_logger);
    slog_scope::info!("Global logging with slog-scope"; "global" => true);
    perform_global_logging();
    println!(
        "slog example: structured logging with slog (log output above goes to stderr)"
    );
}

/// Example function that performs some logging using a logger.
fn perform_some_logging(logger: slog::Logger) {
    slog::info!(logger, "Performing some work"; "task" => "example task");
    // More work...
}

/// Example function that performs logging using the global logger.
fn perform_global_logging() {
    slog_scope::info!("Performing global work"; "task" => "global task");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
