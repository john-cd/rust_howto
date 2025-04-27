#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `use` keyword to bring items into
//! scope without having to write their full path.

// Import a module from an external crate.
// For code from an external crate, the absolute path begins with the
// crate name. Here, the standard `std` library is the external crate.
use std::array;
// With the following, `HashMap` can be used without prefix in the current
// module.
use std::collections::HashMap;
// Use a glob (the `*`) to bring all public objects within a module (here
// `default`) in scope. Use sparingly.
use std::default::*;
// Use `as` to define aliases, for example in case of name conflict.
use std::io::Result as IoResult;
// The following is equivalent to `use std::io; use std::io::Write;`.
use std::io::{self, Write};
// You can combine multiple `use` lines together with { } as well.
use std::{cmp::Ordering, fmt};

/// Demonstrate
fn use_example() -> IoResult<()> {
    let _arr = array::from_ref(&1);

    // `HashMap` can be used
    let _h: HashMap<String, String> = HashMap::new();

    // Use `std::default::Default` without writing down the whole path,
    // because we imported all public object above.
    let _i: i8 = Default::default();

    // `io`
    Ok::<(), io::Error>(())
}

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
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
