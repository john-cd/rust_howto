// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A scope guard will run a given closure when it goes out of scope, even if
//! the code panics (as long as `panic` doesn't abort).
//!
//! This is useful for resource management, like closing files, releasing locks,
//! or cleaning up allocated memory.

use std::fs::File;
use std::io::Error;
use std::io::Write as _;

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

/// Runs a closure at the end of a scope, and cleans up after processing a file.
fn main() -> Result<(), Error> {
    defer();
    process_file("temp/my_document.txt")?;
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the scopeguard example through its entry point.
    #[test]
    fn test_scopeguard_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// The guard runs its closure when it goes out of scope, even on early
    /// return.
    #[test]
    fn guard_runs_on_drop() {
        let ran = std::cell::Cell::new(false);
        {
            let _guard = scopeguard::guard((), |()| ran.set(true));
            assert!(!ran.get());
        }
        assert!(ran.get());
    }

    /// `defer!` runs after the rest of the scope.
    #[test]
    fn defer_runs_last() {
        let order = std::cell::RefCell::new(Vec::new());
        {
            defer! { order.borrow_mut().push("deferred"); }
            order.borrow_mut().push("body");
        }
        assert_eq!(*order.borrow(), ["body", "deferred"]);
    }

    /// The file is written, and an unwritable path is an error.
    #[test]
    fn process_file_writes_and_reports_errors() -> Result<(), Error> {
        let path = std::env::temp_dir().join("rust_howto_scopeguard.txt");
        let name = path.to_str().expect("UTF-8 path");
        process_file(name)?;
        assert_eq!(std::fs::read_to_string(&path)?, "test");
        std::fs::remove_file(&path)?;
        assert!(process_file("/this/dir/does/not/exist/file.txt").is_err());
        Ok(())
    }
}
