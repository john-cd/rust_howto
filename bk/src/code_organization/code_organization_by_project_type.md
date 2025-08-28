
# Code Organization by Project Type and Size

{{#include code_organization_by_project_type.incl.md}}

## Organize the Code of a Simple Library {#simple-library-code-organization}

A simple library crate may consist of its crate root (e.g. the [`lib.rs`][book~rust~ch07-01-packages-and-crates]↗{{hi:lib.rs}} file) and several modules in separate files.

```txt
- src
  - lib.rs
  - module1.rs
  - module2.rs
  - ...
Cargo.toml
```

where `lib.rs` includes the modules with [`mod`][book~rust-reference~modules]↗{{hi:mod}} statements:

```rust,noplayground
pub mod module1; // Public modules, so that the contents may be accessible from outside the crate.
pub mod module2;

// ...
```

You may also keep the modules private and reexport specific items with a [`pub use`][book~rust-reference~use-declarations-visibility]↗{{hi:pub use}} declaration:

```rust,noplayground
mod module1;
mod module2;

pub use module1::public_function;
pub use module2::Struct1;
```

It is common for `lib.rs` to only contain `mod` and `pub use` statements, and nothing else.

In Rust, there are no requirements to store a single [`struct`][book~rust-reference~structs]↗{{hi:struct}} or [`enum`][book~rust-reference~enum]↗{{hi:enum}} and associated code per file.

On the contrary, it is idiomatic for a module to contain multiple functions, `struct` or `enum` declarations, and `impl` blocks with related functionality. For example, you may write all configuration-related items in the `config` module.

## Organize the Code of a Complex Library {#complex-library-code-organization}

As needs grow, you may create nest modules:

```txt
- src
  - lib.rs
  - module1/         # First-level module.
    - mod.rs
    - submodule1.rs  # Nested submodule.
    - submodule2.rs  # Nested submodule.
  - module2.rs
  - ...
Cargo.toml
```

where [`mod.rs`][book~rust-reference~modules-search]↗{{hi:mod.rs}} contains:

```rust,noplayground
pub mod submodule1;
pub mod submodule2;

// ...
```

Code elsewhere can then refer to public items in the submodules:

```rust,noplayground
// In `module2.rs`:
use super::module1::submodule1;
use super::module1::submodule2::Struct1;

fn a_func() {
  submodule1::public_function();

  let _struct = Struct1;
}
```

Nested folders are also possible:

```txt
- src
  - lib.rs
  - module1/
    - mod.rs
    - submodule1/
      - mod.rs
  - module2.rs
  - ...
```

## Less Common Code Organizations {#less-common-code-organization}

Less commonly, you may see:

```txt
- src
  - lib.rs
  - module1.rs
  - module1/
    - submodule1.rs
    - submodule2.rs
  - module2.rs
  - ...
```

where the module is a file and its submodules are under a folder named after the module.

Finally, `module1` can also be defined inline in `lib.rs` - especially if the module only contains `mod` / `pub use` statements.

```rust,noplayground
// In `lib.rs`:
mod module1 {
  pub mod submodule1;
  pub mod submodule2;
}
```

## Flatten the Module Hierarchy Using Reexports {#flatten-module-hierarchy}

Instead of working with deeply-nested modules, it is convenient to keep submodules private and reexport items of interest instead.

For example, you may write in `module1`:

```rust,noplayground
// Private submodules.
mod submodule1;
mod submodule2;

// Reexport a public function and struct.
pub use submodule1::public_function;
pub use submodule2::Struct1;
```

This flattens the module hierarchy and hides implementation details.
Code in a parent module then refers to:

```rust,noplayground
// The function and struct appear as if they were defined in `module1`.
use module1::public_function;
use module1::Struct1;
```

See the [[use_keyword | `use` keyword]] chapter.

## Create a Prelude for Commonly Used Items of Your Library {#prelude}

Library crates with complex public APIs or deeply nested modules often define a "prelude", a public module that reexports their most commonly used items.

A single glob import e.g., `use crate_name::prelude::*;` brings all these common items into scope in one fell swoop and removes the need for repetitive `use` declarations:

```rust,noplayground
// `src/lib.rs`:
pub mod prelude {
    // Reexport commonly used types, traits, or functions:
    pub use super::a_module::SuperCommonType;
    pub use super::a_module::super_common_function;

    // You may also reexport commonly used items from a dependency of your library:
    pub use my_dependency::some_module::CommonType;
}

pub mod a_module {
    pub fn super_common_function() { /* ... */ }
    pub struct SuperCommonType {
      // ...
    }
}

// In the client:
use crate_name::prelude::*;
// No need to import common items individually.
```

The standard library includes a number of preludes. For example, adding [`use std::io::prelude::*;`][c~std::io::prelude~docs]↗{{hi:std::io::prelude}} at the top of I/O heavy modules imports common I/O traits in one line.

There is also a "Rust prelude", things that Rust automatically imports into every Rust program without even the need for an explicit [`use`][book~rust-reference~use-declarations]↗{{hi:use}} declaration. Here is an excerpt:

```rust,noplayground
pub use std::option::Option::{self, Some, None};
pub use std::fmt::{self, Debug, Display};
```

## Organize the Code of a Binary Crate {#binary-crate-organization}

Similarly, a simple binary crate may consist of a crate root (e.g. `main.rs`{{hi:main.rs}}) and several modules in separate files and/or folders.

```txt
- src
  - main.rs
  - module1.rs
  - module2
    - mod.rs
    - submodule1.rs
    - submodule2.rs
Cargo.toml
```

Commonly, `main.rs` is kept minimal and refers to a library crate in the same `src` folder. Indeed, a Rust package can contain both a `src/main.rs` binary crate root as well as a `src/lib.rs` library crate root, and both crates will have the package name by default.

```txt
- src
  - lib.rs     # Library crate root.
  - main.rs    # Binary crate root. Imports the library crate.
  - module1.rs # Module of the library.
  - ...
- tests
- examples
Cargo.toml
```

In that case, `main.rs` imports the public contents of the library crate via `use crate_name::module_name;` and only contain a short `main()` function.

This code organization exposes a public library crate API, which has the advantages of being reusable and more easily testable (via integration tests in the [`tests`][book~rust~tests-directory]↗{{hi:tests directory}} folder). However, if the crate is published on [`crates.io`][crates.io~website]↗{{hi:crates.io}}, you must make sure to update your crate version according to Cargo's SemVer (semantic versioning) rules, every time you change the (now public) API. Your code will also break if you decide to rename your crate.

A variation of this code organization puts the command-line argument parsing (or UI) code in modules under `main.rs` and keeps the non-user-interface code in the associated library crate.

```rust,noplayground
mod cli; // Command-line argument parsing.

use crate_name::lib_module::*; // Business logic in the library crate.

fn main() {
  // ...
}
```

## Organize Large Projects Using a Workspace {#large-projects}

If your project is large, you may want to split it into several crates, which you then depend on in your main project. For example, you may create a `xyz-core` crate, a `xyz-derive` crate for procedural macros that let you `#[derive(...)]` traits defined in your core crate, a `xyz-utils` crate, and a `xyz` main crate that binds all subcrates together.

Each crate, of course, should be further split into modules and submodules as needed.

You will most often create a 'Cargo workspace' to tie together your project's crates. A 'workspace' is a set of 'packages' developed in tandem that share the same [`Cargo.lock`][c~cargo~cargo.lock]↗{{hi:Cargo.lock}} and output (e.g. [`target`][book~cargo~build-cache]↗) directory - and therefore share the same dependencies. A package is a bundle of one or more crates with `Cargo.toml`{{hi:Cargo.toml}} file that describes how to build those crates. A package include at least one crate, as many binary crates as you like, but at most only one library crate. Note that the concept of a 'package' is often conflated with that of a 'crate' and the latter word is often used to describe the former. Practically, a package is a subfolder of your workspace that contains a `Cargo.toml` file.

A typical organization may look as follows:

```txt
# The root folder of your Workspace.
- lib1/ # First package subfolder (a library crate).
  - src/
    - lib.rs
    - ...
  - tests/ # Optional integration tests and examples.
  - examples/
  - Cargo.toml
- lib2/ # Second package (a library crate + several optional binary crates).
  - src/
    - lib.rs
    - bin
      - tool1.rs # Optional binaries, perhaps tools for e.g. database migration.
      - tool2.rs
  - ...
  - Cargo.toml
- main_lib
  - src/
    - lib.rs  # Often, a main library crate that reexports individual libraries.
    - main.rs # Optional binary that uses the library, e.g. a CLI tool.
  - ...
- ...
- target/    # Shared output directory.
- Cargo.toml # Workspace Cargo.toml that references the packages.
- Cargo.lock # Shared lock file (and dependencies).
```

You may use feature flags in the main library crate's [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} to selectively build subcrates and their dependencies:

```toml
[dependencies]
lib1 = { path = "../lib1" }
lib2 = { path = "../lib2", optional = true }

[features]
feature2 = ["dep:lib2"]
```

In projects that mix multiple technologies (a web project or a [`mdbook`][book~mdbook]↗{{hi:mdbook}} that combines markdown and Rust code, like this book), it is common to create a "crates" subdirectory that contains all Rust packages.

The main [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} file in the root of the workspace should contain a 'workspace' section that references its packages:

```toml
[workspace]
members = [ "lib1", "lib2", "main_lib" ]
```

Confusingly, a workspace [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}} can also include a 'root package' in addition to member crates. That lets you place the code of the main library or executable in e.g. a [`src`][book~cargo~project-layout]↗{{hi:src folder}} folder directly under the workspace root.

## Related Topics {#related-topics .skip}

- [[package_layout | Package Layout]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
