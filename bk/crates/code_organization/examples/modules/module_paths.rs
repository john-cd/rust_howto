// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Resolve paths between modules and re-export a public module.

/// A module containing an item used by a sibling module.
mod a {
    /// A unit struct declared in module `a`.
    pub struct A;
}

/// A sibling module that refers to `a` through its parent.
mod c {
    use super::a;

    /// Creates an `a::A` through the sibling module path.
    pub fn do_something() {
        let _ = a::A;
        println!("Created a::A through super::a.");
    }
}

/// A private module that contains a public nested module.
mod e {
    /// A public nested module.
    pub mod f {
        /// Prints a message from the nested module.
        pub fn try_that() {
            println!("Called f::try_that through the re-export.");
        }
    }
}

// Re-export `e::f` so callers can name it from the crate root.
pub use e::f;

/// Calls a sibling module and the re-exported nested module.
fn main() {
    c::do_something();
    f::try_that();
}
// ANCHOR_END: example

/// Runs the module path example.
pub fn run() {
    main();
}
