#![allow(dead_code)]
#![cfg(target_family = "unix")]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! This example demonstrates how to read an environment variable.
//! If the environment variable is not set, a default value is used.

use std::env;

/// Reads the `PATH` environment variable, or a default value if the variable is
/// not set.
fn main() {
    let path = env::var("PATH").unwrap_or("".to_owned());
    println!("Path: {path}");
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
