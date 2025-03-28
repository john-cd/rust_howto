# Modules {#modules}

{{#include modules.incl.md}}

[![Rust by example - Modules][book-rust-by-example-mod-badge]][book-rust-by-example-mod]{{hi:mod}}

Crates can contain modules{{hi:Modules}}.

Declaring modules: In the crate root file{{hi:Crate root file}} (`main.rs` or `lib.rs`), you can declare new modules; say, you declare a "garden" module with `mod garden;` (or `pub mod garden;` for public); The compiler will look for the module's code in these places:

- Inline, within curly brackets that replace the semicolon following mod garden.
- In the file src/garden.rs.
- In the file src/garden/mod.rs (older style).

In any file other than the crate root, you can declare sub-modules{{hi:Sub-modules}}. For example, you might declare `mod vegetables;` in ``src/garden.rs`. The compiler will look for the submodule's code within the directory named for the parent module{{hi:Parent module}} in these places:

- Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon.
- In the file src/garden/vegetables.rs.
- In the file src/garden/vegetables/mod.rs (older style).

In Rust, all items ([functions][p-functions], methods, [structs][p-structs], [enums][p-enums], modules, and constants) are private to parent modules by default. Items can access other items in the same module, even when private.

Items in a parent module can't use the private items{{hi:Private items}} inside child modules, but items in child modules can use the items in their ancestor modules.

[![book-rust-by-example-visibility-rules][book-rust-by-example-visibility-rules-badge]][book-rust-by-example-visibility-rules]

[A clear explanation of Rust's module system][rust-module-system-website]⮳.

## `use` Keyword {#use-keyword}

Create a shortcut to a path with the [`use`][book-rust-reference-use]{{hi:use}}⮳ keyword once, and then use the shorter name everywhere else in the scope.

[![book-rust-by-example-use][book-rust-by-example-use-badge]][book-rust-by-example-use]

```rust,editable
{{#include ../../crates/language/tests/feat/modules.rs:example}}
```

Idiomatic - bringing the function's parent module into scope, not the function itself:

```rust,editable
{{#include ../../crates/language/tests/feat/modules2.rs:example}}
```

On the other hand, when bringing in [structs][p-structs], [enums][p-enums], and other items with use, it's idiomatic to specify the full path.

```rust,editable
{{#include ../../crates/language/tests/feat/modules3.rs:example}}
```

## Related Topics {#skip}

- [[package_layout | Package Layout]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[modules: review](https://github.com/john-cd/rust_howto/issues/553)
add rust-module-system
</div>
