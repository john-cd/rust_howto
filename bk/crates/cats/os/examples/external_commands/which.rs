#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates how to use the `which` crate to find the location of
//! executables.

use std::env;

use which::which;

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

#[test]
fn test() {
    main();
}
