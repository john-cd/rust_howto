# Modules and `use` keyword

{{#include modules.incl.md}}

## Modules {#modules}

[![Rust by example - Modules][book-rust-by-example-mod-badge]][book-rust-by-example-mod]{{hi:mod}}

Modules{{hi:Modules}} are Rust's way of organizing code within a crate (a binary or a library){{hi:crate}}.

They help:

- Group related code: Keep functions, structs, enums, etc., that work together in one place.
- Control visibility: Decide which parts of your code are public (usable outside the module) and which are private implementation details.
- Create namespaces: Avoid naming conflicts by putting items into distinct scopes.

To declare a module, use the `mod` keyword.

A module can be written inline, in the same file than its parent, by using curly brackets: `mod <module_name> { ... }` or it may be defined in other files, by inserting a semicolon after its declaration: `mod <module_name>;`.

In Rust, all items ([functions][p-functions], methods, [structs][p-structs], [enums][p-enums], modules, and constants) are private to their module by default. Items can access other items in the _same_ module, even when private. Use the `pub` keyword before an item's definition (`pub fn`, `pub struct`, etc.) to make it accessible from outside its module.

Items in a parent module can't use the private items{{hi:Private items}} inside child modules, but items in child modules can use the items in their ancestor modules.

### Organize your Code using Modules {#inline-modules}

The following demonstrates the use of (inline) modules, paths to access items within modules, and visibility rules.

```rust,editable
{{#include ../../crates/language/tests/modules/modules.rs:example}}
```

### Split your Code amongst Multiple Files and Folders {#code-files}

Each crate has a crate root file{{hi:Crate root file}} (typically `main.rs` or `lib.rs` in the `src` folder). You may write all your code in that file, if it is very short. For non-trivial projects, you will split your code amongst multiple files and/or folders.

Write `mod module_name;` to declare a module that has its code in a separate file (note the semicolon at the end and the lack of `{ }`). The compiler then looks for specific files:

- `module_name.rs`,
- `module_name/mod.rs` (older style),

and insert the contents in the parent module, as if it were an inline module.

You can nest (inline and external-file) modules as you wish.

Note the following:

- Adding `.rs` files to your source code folder does not automatically incorporate the code in your crate. You must add a explicit `mod` statement.
  - Editors like VS Code will not analyze your code or display hints if you forget to do so.
- The `mod` statement must be added to their _parent_ module / file, not to the module itself.

## `use` Keyword {#skip}

### Avoid writing full paths with the `use` Keyword {#use-keyword}

[![book-rust-by-example-use][book-rust-by-example-use-badge]][book-rust-by-example-use]

The [`use`][book-rust-reference-use]{{hi:use}}⮳ keyword creates a shortcut for a path. The shorter name everywhere else in the scope.

```rust,editable
{{#include ../../crates/language/tests/modules/use1.rs:example}}
```

### Import a Function with the `use` Keyword {#import-function-with-use}

It is idiomatic to bring the function's parent module into scope, not the function itself:

```rust,editable
{{#include ../../crates/language/tests/modules/use2.rs:example}}
```

### Import a Struct or Enum with the `use` Keyword {#import-struct-or-enum-with-use}

On the other hand, when bringing in [structs][p-structs], [enums][p-enums], and other items with `use`, it is idiomatic to specify the full path.

```rust,editable
{{#include ../../crates/language/tests/modules/use3.rs:example}}
```

## Typical Code Organization {#skip}

### Simple Library Code Organization {#library-code-organization}

A simple library crate may consist of its crate root (e.g. `lib.rs`) and several modules in separate files.

```txt
- src
  - lib.rs
  - module1.rs
  - module2.rs
  - ...
```

where `lib.rs` includes:

```rust,editable,noplayground
mod module1;
mod module2;
```

It is common for `lib.rs` to only contain `mod` and `pub use` statements.

In Rust, there are no requirements to store a single `struct` or `enum` and associated code per file.
It is common for a module to contain multiple functions, `struct` or `enum` declarations, and `impl` blocks with related functionality.
For example, put all configuration items in a `config` module.

### More Complex Library Code Organization {#complex-library-code-organization}

As needs grow, you may create submodules:

```txt
- src
  - lib.rs
  - module1/
    - mod.rs
    - submodule1.rs
    - submodule2.rs
  - module2.rs
  - ...
```

where `mod.rs` contains:

```rust,editable,noplayground
pub mod submodule1;
pub mod submodule2;

// ...
```

Code e.g. in `lib.rs` can then refer to public items in the submodules:

```rust,editable,noplayground
use module1::submodule1;
use module1::submodule2::Struct1;

fn main() {
  submodule1::public_function();

  let _ = Struct1;
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

### Less common code organizations {#less-common-code-organization}

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

Finally, `module1` can also be defined inline in `lib.rs` - especially if the module only contains `mod` statements.

```rust,editable,noplayground
mod module1 {
  pub mod submodule1;
  pub mod submodule2;
}
```

### Flatten the Module Hierarchy using Reexports {#flatten-module-hierarchy}

It is convenient to keep submodules private and reexport items of interest:

```rust,editable,noplayground
mod submodule1;
mod submodule2;

pub use submodule1::public_function;
pub use submodule2::Struct1;
```

This flattens the module hierarchy (to one level in this case) and hides implementation details.
Code in a parent module then refers to:

```rust,editable,noplayground
use module1::public_function;
use module1::Struct1;
```

### Code Organization for Binary Crates {#binary-crate-organization}

Similarily, a simple binary crate may consist of a crate root (e.g. `main.rs`) and several modules in separate files and/or folders.

```txt
- src
  - main.rs
  - module1.rs
  - module2
    - mod.rs
    - submodule1.rs
    - submodule2.rs
```

Commonly, `main.rs` is kept minimal and refers to a library crate in the same `src` folder.

```txt
- src
  - lib.rs     # Library crate root.
  - main.rs    # Binary crate root.
  - module1.rs # Module of the library.
  - ...
```

In that case, `main.rs` imports the contents of the library crate via `use crate_name::module_name;` and only contain a short `main()` function. This code organization exposes a public crate API, which has the advantages of being more easily testable (via integration tests in the `tests` folder). However, if the crate is published on `crates.io`, you must make sure to update your crate version according to Cargo's SemVer (semantic versioning) rules, every time you change the now public API. You code will also break if you decide to rename your crate.

A variation of this code organization puts the command-line argument parsing or UI code in a module under `main.rs` and keeps the non-CLI code in the associated library crate.

```rust
mod cli; // Command-line argument parsing.

use crate_name::lib_module::*; // Business logic.

fn main() {
  // ...
}
```

### Organize Large Projects using Multiple Packages / Crates {#large-projects}

If your project is huge, split it into several crates, which you then depend on in your main project. For example, you may create a `xyz-core` crate, a `xyz-derive` crate for procedural macros that let you `#[derive(...)]` traits defined in your core crate, a `xyz` main crate that binds all subcrates together.

You will most often create a 'Cargo workspace' to tie together your project's crates. A 'workspace' is a set of 'packages' developed in tandem that share the same `Cargo.lock` and output (e.g. `target`) directory - and therefore share the same dependencies. A package is a bundle of one or more crates with `Cargo.toml` file that describes how to build those crates. A package include at least one crate, as many binary crates as you like, but at most only one library crate. Note that the concept of a 'package' is often conflated with that of a 'crate' and the latter word is often used to describe the former. Pratically speaking, a package is a subfolder of your workspace that contains a `Cargo.toml` file.

A typical organization may look as follows:

```txt
# The root folder of your worskpace.
- lib1/ # First package subfolder (library crate).
  - src/
    - lib.rs
    - ...
  - tests/ # Optional integration tests and examples.
  - examples/
  - Cargo.toml
- lib2/ # Second package (library crate + several binary crates).
  - src/
    - lib.rs
    - bin
      - tool1.rs # Optional binary, perhaps a tool for e.g. database migration.
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
- Cargo.toml # Worskpace Cargo.toml that references the packages.
- Cargo.lock # Shared lock file (and dependencies).
```

You may use feature flags in the main library crate's `Cargo.toml` to selectively build subcrates and their dependencies. Each crate, of course, should be further split into modules (and submodules) as described above.

In projects that mix multiple technologies (a web project or a `mdbook` that combines markdown and Rust code, like this book), it is common to create a "crates" subdirectory that contains all Rust packages.

The main `Cargo.toml` file in the root of the workspace should contain a 'workspace' section that references its packages:

```toml
[workspace]
members = [ "lib1", "lib2", "main_lib" ]
```

Technically, a worskpace `Cargo.toml` can also include a 'root package' in addition to members. That lets you place the code of the main library or executable in e.g. a `src` folder directly under the workspace root.

### Refer to External Crates {#external-crates}

To refer to a (public) item of an external crate, first make sure to include said external crate as a dependency in your crate's `Cargo.toml` file, for example:

```toml
[dependencies]
anyhow = "1.0.95"
```

You can then refer to items using a path that starts with the external crate's name:

```rust,editable,noplayground
use anyhow::Error;  // A struct.
use anyhow::Result; // A type alias.
use anyhow::anyhow; // The `anyhow!` macro.
```

## Related Topics {#skip}

- [[package_layout | Package Layout]].

## References {#skip1}

- A [clear explanation of Rust's module system][rust-module-system-website]⮳.
- [![book-rust-by-example-visibility-rules][book-rust-by-example-visibility-rules-badge]][book-rust-by-example-visibility-rules]

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[modules: finish NOW; split in multiple files?](https://github.com/john-cd/rust_howto/issues/553)
</div>
