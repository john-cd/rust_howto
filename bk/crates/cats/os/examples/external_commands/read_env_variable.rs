#![cfg(target_family = "unix")]
// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to read an environment variable.
//! If the environment variable is not set, a default value is used.
//!
//! Environment variables are named text values the operating system passes to
//! each program, for settings such as a home directory or a path list. A
//! variable may be missing or contain invalid text, so reading it returns a
//! `Result`.

use std::env;

/// Reads the `PATH` environment variable, or a default value if the variable is
/// not set.
fn main() {
    let path = env::var("PATH").unwrap_or("".to_owned());
    println!("Path: {path}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the read env variable example through its entry point.
    #[test]
    fn test_read_env_variable_example_entry_point() {
        main();
    }

    /// `PATH` is set and not empty in the test environment.
    #[test]
    fn path_is_set() {
        let path = env::var("PATH").unwrap_or("".to_owned());
        assert!(!path.is_empty());
    }

    /// A variable that is not set falls back to the default value.
    #[test]
    fn unset_variable_uses_default() {
        let value =
            env::var("RUST_HOWTO_SURELY_UNSET").unwrap_or("".to_owned());
        assert_eq!(value, "");
    }
}
