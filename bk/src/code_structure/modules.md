# Modules and Paths

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

## Paths {#paths}

Paths let you access items (like functions, structs, enums, modules, etc.) within your Rust code, when those items are defined in different modules. There are two main kinds of paths:

- Relative paths start from the current module you are writing code in.

```rust,noplayground
/// Inline module declaration.
mod a_module {
    pub fn some_function() {
        // ...
    }
}

fn call_something_in_a_module() {
  // Call `some_function` using a relative path:
  a_module::some_function();

  // Technically, relative paths starts with a `self` keyword,
  // which refers to the current module, but it is very often implied:
  // The above is equivalent to:
  self::a_module::some_function();
}
```

The `super` keyword is used to refer to the parent module:

```rust,noplayground
mod a {
  fn in_a() {
    // Call a function in the parent module of `a`:
    super::in_parent_module();
  }
}

fn in_parent_module() {
  // ...
}
```

- Absolute paths start from the root of your crate (the top-level module, usually defined in `src/lib.rs` or `src/main.rs`). You use the keyword `crate` followed by `::` to begin an absolute path.

```rust,editable,noplayground
crate::module_a::submodule_b::some_function();
```

This absolute path refers to `some_function` located inside `submodule_b`, which is inside `module_a`, starting from the crate root.Absolute paths are rarely seen in practice but useful for disambiguation, when a module name is the same than an external dependency.

## Visibility Rules {#visibility-rules}

[![book-rust-by-example-visibility-rules][book-rust-by-example-visibility-rules-badge]][book-rust-by-example-visibility-rules]

In Rust, all items ([functions][p-functions], methods, [structs][p-structs], [enums][p-enums], modules, and constants) are private to their module by default. Items can access other items in the _same_ module, even when private. Use the `pub` keyword before an item's definition (`pub fn`, `pub struct`, etc.) to make it accessible from outside its module.

Items in a parent module can't use the private items{{hi:Private items}} inside child modules, but items in child modules can use the items in their ancestor modules.

The following demonstrates the use of (inline) modules, paths to access items within modules, and visibility rules.

```rust,editable
{{#include ../../crates/code_structure/tests/modules/modules.rs:example}}
```

## Split your Code among Several Files and Folders {#code-files}

Each crate has a crate root file{{hi:Crate root file}} (typically `main.rs` or `lib.rs` in the `src` folder). You may write all your code in that file, if it is very short. For non-trivial projects, you will write your code in multiple files and/or folders.

Write `mod module_name;` to declare a module that has its code in a separate file (note the semicolon at the end and the lack of `{ }`). The compiler then looks for specific files:

- `module_name.rs`,
- `module_name/mod.rs` (older style),

and insert the contents in the parent module, as if it were an inline module.

You can nest (inline and external-file) modules as you wish.

Note the following:

- Adding `.rs` files to your source code folder does not automatically incorporate the code in your crate. You must add an explicit `mod` statement.
  - Editors like 'VS Code' will not analyze your code or display hints if you forget to do so!
- The `mod` statement must be added to the _parent_ file, not to the file that contains the module itself.

## Related Topics {#skip}

- [[package_layout | Package Layout]].

## References {#skip1}

- A [clear explanation of Rust's module system][rust-module-system-website]⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO final review
</div>
