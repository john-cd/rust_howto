// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to configure the `env_logger` to output logs
//! to `Stdout`.

use env_logger::Target;

/// This example demonstrates how to configure the `env_logger` to output logs
/// to `Stdout`.
fn main() {
    // Create a new `env_logger` builder.
    env_logger::Builder::new().target(Target::Stdout).init();

    // Log an error message.
    log::error!("This error has been printed to Stdout");
    println!("log_stdout example: configured env_logger to write to stdout");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the log stdout example through its entry point.
    #[test]
    fn test_log_stdout_example_entry_point() {
        main();
    }

    // output-only: the example only emits log records, and the process-wide
    // logger can be installed once, so the records cannot be captured here.
}
