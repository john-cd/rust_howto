// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Logging an error with the `log` crate.

/// Simulates executing a database query.
/// This function always returns an error for demonstration purposes.
fn execute_query(_query: &str) -> Result<(), &'static str> {
    Err("I'm afraid I can't do that")
}

/// Runs a query that fails and logs the error.
fn main() {
    env_logger::init();

    let response = execute_query("DROP TABLE students");
    if let Err(err) = response {
        log::error!("Failed to execute query: {err}");
    }
    println!(
        "log_error example: set RUST_LOG=error to see the error log above"
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the log error example through its entry point.
    #[test]
    fn test_log_error_example_entry_point() {
        main();
    }

    #[test]
    fn execute_query_always_fails_with_the_documented_message() {
        assert_eq!(
            execute_query("DROP TABLE students"),
            Err("I'm afraid I can't do that")
        );
        assert!(execute_query("").is_err());
    }
}
