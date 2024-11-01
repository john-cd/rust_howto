#![allow(unused_imports, dead_code)]
// ANCHOR: example

// For code from an external crate, the absolute path begins with the
// crate name Here, the standard `std` library
use std::collections::HashMap;
// Glob - all public objects in `collections` are now in scope
use std::collections::*;
// With the following, `File` without prefix is available in the scope
use std::fs::File;
// The following is equivalent to `use std::io; use std::io::Write;`
use std::io::{self, Write};
// Combine multiple `use` lines together with { }
use std::{cmp::Ordering, fmt};

mod utils {
    pub fn insert_use() {}
}
// Absolute path - for code from the current crate, it starts with the
// literal `crate`.
use crate::modules::utils::insert_use;

mod a {
    pub mod b {}
}
// A relative path starts from the current module and uses `self`,
// `super`, or an identifier in the current module.
use self::a::b;

mod c {
    // We can construct relative paths that begin in the parent module,
    // rather than the current module or the crate root, by using `super`
    // at the start of the path.
    use super::a;
}

use std::fmt::Result;
use std::io::Result as IoResult; // Alias

mod front_of_house {
    pub mod hosting {}
}
// Reexporting - `pub use` re-exports the `hosting` module from the
// root module, thus external code can use the path
// `<crate>::hosting::add_to_waitlist()` instead of
// `crate::front_of_house::hosting::add_to_waitlist()`.
pub use front_of_house::hosting;

fn main() {}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
