#![allow(unused_imports, dead_code)]

use std::collections::HashMap;
use std::fs::File; // `File` without prefix is now available in the scope // For code from an external crate, the absolute path begins with the crate name - here, the standard `std` library

use std::collections::*; // Glob - all public objects

use std::io::{self, Write};
use std::{cmp::Ordering, fmt}; // Combining multiple use lines together

mod utils {
    pub fn insert_use() {}
}
use crate::utils::insert_use; // Absolute path - for code from the current crate, it starts with the literal crate.

mod a {
    pub mod b {}
}
use self::a::b; // A relative path starts from the current module and uses self, super, or an identifier in the current module.

mod c {
    use super::a; // We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using super at the start of the path.
}

use std::fmt::Result;
use std::io::Result as IoResult; // Alias

mod front_of_house {
    pub mod hosting {}
}
pub use crate::front_of_house::hosting; // Reexporting -  Now that this pub use has re-exported the hosting module from the root module, external code can now use the path <crate>::hosting::add_to_waitlist() instead.

fn main() {}
