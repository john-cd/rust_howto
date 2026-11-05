// TODO Reviewed ??-??-2026.

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
    pub fn do_something() -> &'static str {
        let _ = a::A;
        "Created a::A through super::a."
    }
}

/// A private module that contains a public nested module.
mod e {
    /// A public nested module.
    pub mod f {
        /// Returns a message from the nested module.
        pub fn try_that() -> &'static str {
            "Called f::try_that through the re-export."
        }
    }
}

// Re-export `e::f` so callers can name it from the crate root.
pub use e::f;

/// Calls a sibling module and the re-exported nested module.
fn main() {
    println!("{}", c::do_something());
    println!("{}", f::try_that());
}
// ANCHOR_END: example

/// Runs the module path example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the module path example runs through its entry point.
    #[test]
    fn test_module_paths_example_entry_point() {
        main();
    }

    #[test]
    fn test_sibling_module_path() {
        assert_eq!(c::do_something(), "Created a::A through super::a.");
    }

    #[test]
    fn test_reexported_module_path() {
        // `f` is reachable from the crate root through `pub use`...
        assert_eq!(f::try_that(), "Called f::try_that through the re-export.");
        // ...and through its original path.
        assert_eq!(e::f::try_that(), f::try_that());
    }
}
