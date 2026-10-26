#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates how to use the `which` crate to find the location of
//! executables.

use std::env;

use which::which;

/// Looks up the location of executables in the `PATH`.
fn main() {
    // Print the current PATH environment variable.
    println!("PATH = {:?}", env::var_os("PATH"));

    // Try to find the location of common executables.
    match which("rustc") {
        Ok(path) => println!("Found rustc at: {}", path.display()),
        Err(e) => println!("Could not find rustc: {e}"),
    }

    // Try to find a command that likely doesn't exist.
    match which("some-command-that-probably-doesnt-exist") {
        Ok(path) => println!("Found command at: {}", path.display()),
        Err(e) => println!("Could not find command: {e}"),
    }
}

// ANCHOR_END: example

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
