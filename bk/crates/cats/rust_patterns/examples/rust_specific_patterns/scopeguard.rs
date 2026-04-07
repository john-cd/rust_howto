#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates the `scopeguard` crate for RAII-style cleanup.
//!
//! A scope guard runs a given closure when it goes out of scope, even if the
//! code panics (as long as `panic` doesn't abort). This is useful for resource
//! management, such as releasing locks, reverting state changes, or logging on
//! exit.

use scopeguard::defer;

/// Use the `defer!` macro to run an operation at scope exit.
fn example_defer() {
    println!("Before deferred operation.");

    defer! {
        // This runs at the end of the scope, even if a panic occurs.
        println!("Deferred: scope exited.");
    };

    println!("After deferred operation (runs before deferred block).");
    // Scope exits here; the deferred closure runs.
}

/// Use `scopeguard::guard` when the scope guard closure needs access to a
/// value that is also used outside the guard.
fn example_guard() {
    let value = 0;

    {
        // `guard` wraps `value` and calls the closure with it on drop.
        let mut guard = scopeguard::guard(value, |v| {
            println!("Guard dropped. Final value seen by guard: {v}");
        });

        // The guard dereferences to the inner value.
        *guard += 10;
        println!("Inside guard scope, value = {}", *guard);
        // Guard is dropped here and the closure runs.
    }

    println!("Outside guard scope, original value = {value}");
}

fn main() {
    example_defer();
    println!();
    example_guard();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
