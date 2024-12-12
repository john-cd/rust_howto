#![allow(unused_imports, dead_code)]
// ANCHOR: example

// With the following, `File` without prefix is available in the scope
// For code from an external crate, the absolute path begins with the
// crate name - here, the standard `std` library
use std::collections::HashMap;
// Glob - all public objects in `collections` are now in scope
// Use sparingly
use std::collections::*;
// Use `as` to define aliases, for example in case of name conflict
use std::fmt::Result;
use std::fs::File;
use std::io::Result as IoResult;
// The following is equivalent to `use std::io; use std::io::Write;`
use std::io::{self, Write};
// You can combine multiple `use` lines together with { } as well
use std::{cmp::Ordering, fmt};

mod a {
    pub mod b {
        pub fn fn_in_b() {
            println!("in b!");
        }
    }
    pub struct A;
}
// For internal code, a relative path starts from the current module and uses
// `self`, or an identifier in the current module.
use self::a::b;
// b is now in scope

// Try the simpler version:
// use a::b;

fn do_something() {
    b::fn_in_b();
}

mod c {
    // We can construct relative paths that begin in the parent module,
    // rather than the current module or the crate root, by using `super`
    // at the start of the path.
    use super::a;
    // a is now in scope

    pub fn do_something_else() {
        let _a = a::A;
        println!("do_something_else");
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
            println!("try_that");
        }
    }
}
// `pub use` re-exports the `f` module from the
// root module, thus external code can use the path
// `<crate_name>::f::try_that()` instead of
// `<crate_name>::e::f::try_that()`.
pub use e::f;

fn main() {
    do_something();
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
