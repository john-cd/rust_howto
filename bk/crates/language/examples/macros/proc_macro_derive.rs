#![allow(unused)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! A derive procedural macro that implements a trait.

use proc_macros::DebugPrint;

// Define the custom trait that the macro will implement.
/// A trait that prints a debug representation.
pub trait DebugPrint {
    /// Prints a debug representation of `self`.
    fn debug_print(&self);
}

// Apply our custom derive macro.
// This will automatically `impl DebugPrint for` the struct.
// We also derive the standard `Debug` trait.
/// A user, with a derived [`DebugPrint`] implementation.
#[derive(DebugPrint, Debug)]
struct User {
    /// Unique identifier of the user.
    id: u64,
    /// Name of the user.
    name: String,
    /// Whether the user is active.
    active: bool,
}

/// Creates a user and prints it using the derived trait.
fn main() {
    let u = User {
        id: 1,
        name: "Alice".to_owned(),
        active: true,
    };
    u.debug_print();
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
