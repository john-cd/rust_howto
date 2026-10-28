#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Testing code that reads environment variables.
//!
//! The environment is process-global and tests run in parallel threads, so
//! tests that set the same variable race. `serial_test` runs them one at a
//! time; `temp_env` restores the previous value afterwards. (In edition 2024
//! `std::env::set_var` is `unsafe` for exactly this reason.)

/// The log level from `APP_LOG`, defaulting to `info`.
fn log_level() -> String {
    std::env::var("APP_LOG").unwrap_or_else(|_| "info".to_owned())
}

/// The example runs under `cargo test`.
fn main() {}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    /// Reads a temporary log level from the environment.
    #[test]
    #[serial(env)]
    fn reads_the_variable() {
        temp_env::with_var("APP_LOG", Some("debug"), || {
            assert_eq!(log_level(), "debug");
        });
    }

    /// Uses the default level when the environment variable is absent.
    #[test]
    #[serial(env)]
    fn defaults_when_unset() {
        // `None` removes the variable for the duration of the closure.
        temp_env::with_var("APP_LOG", None::<&str>, || {
            assert_eq!(log_level(), "info");
        });
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
// ANCHOR_END: example
