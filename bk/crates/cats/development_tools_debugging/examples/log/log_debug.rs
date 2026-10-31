// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Emitting a debug message with the `log` crate.
//!
//! Log levels rank messages: `error`, `warn`, `info`, `debug` and `trace`, from
//! most to least important. A logger filters out levels below its threshold, so
//! debug output costs almost nothing when it is off.

/// Executes a database query and logs it at the debug level.
fn execute_query(query: &str) {
    log::debug!("Executing query: {query}");
}

/// Initializes `env_logger` and runs a query, which logs a debug message.
fn main() {
    // `env_logger` is a simple logger that can be configured via environment
    // variables. Example: `RUST_LOG=info ./app`.
    env_logger::init();

    execute_query("DROP TABLE students");
    println!(
        "log_debug example: set RUST_LOG=debug to see the debug log above"
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the log debug example through its entry point.
    #[test]
    fn test_log_debug_example_entry_point() {
        main();
    }

    // output-only: the example only emits log records, and the process-wide
    // logger can be installed once, so the records cannot be captured here.
}
