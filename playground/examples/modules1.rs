#![allow(dead_code)]

mod a {
    pub struct A;
}

mod c {
    // We can construct relative paths that begin in the parent module,
    // rather than the current module or the crate root, by using `super`
    // at the start of the path. `super` refers to the parent module.
    use super::a; // The `a` module is now in scope.

    pub fn do_something_else() {
        //
        let _a = a::A;
        println!("Do something else.");
    }
}

mod d {
    pub fn let_try_this() {}
}
// Absolute paths start with the literal `crate`.
// You can try:
// use crate::d;

mod e {
    pub mod f {
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
