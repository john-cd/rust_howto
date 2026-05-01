// ANCHOR: example
//! A scope guard will run a given closure when it goes out of scope, even if
//! the code panics (as long as `panic` doesn't abort).
//!
//! This is useful for resource management, like closing files, releasing locks,
//! or cleaning up allocated memory.

use std::fs::File;
use std::io::Error;
use std::io::Write;

use scopeguard::defer;

/// Use the `defer!` macro to run an operation at scope exit, either regular
/// scope exit or during unwinding from a panic.
fn defer() {
    defer! {
        println!("Executing the deferred operation.");
        // Under the covers, defer! creates a `ScopeGuard` and passes its body as a closure.
    };

    println!("Executing the function.");

    // The scope exits and the ScopeGuard closure executes here.
}

// Also consider `defer_on_success!` (runs on successful scope exit) and
// `defer_on_unwind!` (runs on panic).

/// Use `scopeguard::guard` if the scope guard closure needs to access an outer
/// value that is also mutated outside of the scope guard.
fn process_file(filename: &str) -> Result<(), Error> {
    println!("Create file: {filename}");
    let file = File::create(filename)?;

    let mut file_guard = scopeguard::guard(file, |f| {
        // Ensure that all in-memory file data reaches the filesystem.
        let _ = f.sync_all();
        println!("Closing file: {filename}");
    });

    println!("Write to file.");
    // The guard works like a smart pointer, so the inner value can be accessed
    // by (mutable) reference.
    file_guard.write_all(b"test")?;

    // `file_guard` is dropped (due to the scope exit) and the file is flushed.
    Ok(())
}

fn main() -> Result<(), Error> {
    defer();
    process_file("temp/my_document.txt")?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
