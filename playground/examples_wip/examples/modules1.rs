// TODO Reviewed ??-??-2026
#![expect(dead_code, reason = "scratch example with unused items")]

/// A module with a public struct.
mod a {
    /// A struct.
    pub struct A;
}

/// A module that refers to its sibling with `super`.
mod c {
    // We can construct relative paths that begin in the parent module,
    // rather than the current module or the crate root, by using `super`
    // at the start of the path. `super` refers to the parent module.
    use super::a; // The `a` module is now in scope.

    /// Creates an `A` and prints a message.
    pub fn do_something_else() {
        //
        let _ = a::A;
        println!("Do something else.");
    }
}

/// A module with a public function.
mod d {
    /// Does nothing.
    pub fn let_try_this() {}
}
// Absolute paths start with the literal `crate`.
// You can try:
// use crate::d;

/// A module that contains the public module `f`.
mod e {
    /// A public module, re-exported at the root.
    pub mod f {
        /// Prints a message.
        pub fn try_that() {
            println!("Try that.");
        }
    }
}

// `pub use` re-exports the `f` module from the root module, thus external code
// can use the path `<crate_name>::f::try_that()` instead of
// `<crate_name>::e::f::try_that()`.
pub use e::f;

/// The main function of the program.
fn main() {
    c::do_something_else();
    // You can of course access the item made public by `pub use` from your
    // module
    f::try_that();
}
