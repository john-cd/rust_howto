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

[![visibility-rules-rust-by-example-badge]][visibility-rules-rust-by-example]

[A clear explanation of Rust’s module system][rust-module-system]⮳

## Use keyword

Create a shortcut to a path with the `use` keyword once, and then use the shorter name everywhere else in the scope.

[![use-rust-by-example-badge]][use-rust-by-example]

```rust,editable
{{#include ../../deps/examples/modules.rs}}
```

Idiomatic - bringing the function’s parent module into scope, not the function itself:

```rust,editable
{{#include ../../deps/examples/modules2.rs}}
```

On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path.

```rust,editable
{{#include ../../deps/examples/modules3.rs}}
```

{{#include ../refs/link-refs.md}}
