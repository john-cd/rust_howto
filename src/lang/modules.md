# Modules

Crates can contain modules.

Declaring modules: In the crate root file (`main.rs` or `lib.rs`), you can declare new modules; say, you declare a “garden” module with `mod garden;` (or `pub mod garden;` for public); The compiler will look for the module’s code in these places:
- Inline, within curly brackets that replace the semicolon following mod garden
- In the file src/garden.rs
- In the file src/garden/mod.rs (older style)

In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
- Inline, directly following mod vegetables, within curly brackets instead of the semicolon
- In the file src/garden/vegetables.rs
- In the file src/garden/vegetables/mod.rs (older style)

In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default. 
Items can access other items in the same module, even when private.

Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.

[Visibility rules]( https://doc.rust-lang.org/rust-by-example/mod/visibility.html ) 

[A clear explanation of Rust’s module system]( https://www.sheshbabu.com/posts/rust-module-system/ )


## Use keyword

Create a shortcut to a path with the `use` keyword once, and then use the shorter name everywhere else in the scope.

[Use]( https://doc.rust-lang.org/rust-by-example/mod/use.html )


```rust,ignore
use std::fs::File;              // `File` without prefix is now available in the scope  
use std::collections::HashMap;  // for code from an external crate, the absolute path begins with the crate name - here, the standard `std` library 

use std::collections::*;        // Glob - all public objects

use itertools::Itertools;
use syntax::ast;

use std::{cmp::Ordering, io};   // combining multiple use lines together
use std::io::{self, Write};

use crate::utils::insert_use;   // absolute path - for code from the current crate, it starts with the literal crate.

use self::auto_import;          // A relative path starts from the current module and uses self, super, or an identifier in the current module.

use super::AssistContext;       // We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using super at the start of the path.

use std::fmt::Result;
use std::io::Result as IoResult;        // Alias

pub use crate::front_of_house::hosting; // Reexporting -  Now that this pub use has re-exported the hosting module from the root module, external code can now use the path <crate>::hosting::add_to_waitlist() instead.
```

Idiomatic - bringing the function’s parent module into scope, not the function itself: 

```rust,ignore
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path.

```rust
use std::collections::HashMap;

let mut map: HashMap<u32, String>  = HashMap::new();
```
