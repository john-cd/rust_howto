# `use` Declarations

{{#include use_keyword.incl.md}}

Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths.

## Avoid Writing Full Paths with the `use` Keyword {#use-keyword}

[![book~rust-by-example~use][book~rust-by-example~use~badge]][book~rust-by-example~use]

The [`use`][book~rust-reference~use]{{hi:use}}↗ keyword creates a shortcut for a path. The shorter name can be used everywhere else in the scope.

```rust,editable
{{#include ../../crates/code_organization/examples/use/use1.rs:example}}
```

## Bring an Item from an External Crate into Scope {#bring-item-from-external-crate-into-scope}

To bring items from an external crate into scope, without having to write their full paths every time, write a `use` declaration with a path that begins with the external crate name. See also the [[dependencies | Dependencies]] chapter.

```rust,editable
{{#include ../../crates/code_organization/examples/use/use_external_crate.rs:example}}
```

## Bring a Function in Scope Idiomatically {#bring-function-in-scope}

It is idiomatic to bring the function's parent module into scope, not the function itself, then refer to the function via its shortened path `a_module::a_function()`.

```rust,editable
{{#include ../../crates/code_organization/examples/use/use2.rs:example}}
```

## Bring a Struct or Enum in Scope Idiomatically {#bring-struct-or-enum-in-scope}

On the other hand, when bringing in [structs][p~structs], [enums][p~enums], and other items in scope, it is idiomatic to specify the full path in the `use` statement and refer to the type via its name.

```rust,editable
{{#include ../../crates/code_organization/examples/use/use3.rs:example}}
```

Note: Rust doesn't allow importing two items with the same name into one scope. In that case, bring their modules into scope or write their full paths.

## Reexport Items with `pub use` {#pub-use}

`use` declarations are private to their containing module by default. A `use` declaration can be made public by the `pub` keyword. Such a declaration re-exports a name. See also the [[visibility | Visibility]] chapter.

```rust,editable
{{#include ../../crates/code_organization/examples/use/pub_use.rs:example}}
```

## Make `use` Declarations More Compact {#use-declaration-shortcuts}

Use declarations support a number of convenient shortcuts. You may use globs, define aliases, and combine multiple `use` statements.

```rust,editable
{{#include ../../crates/code_organization/examples/use/use_shortcuts.rs:example}}
```

Note that, while these shortcuts make your code more compact and readable, they can be inconvenient during early development, when you need to add or remove `use` statements frequently. Consider adding a [`rustfmt.toml`](https://rust-lang.github.io/rustfmt) configuration file to your project and adding `imports_granularity = "Item"` to flatten imports, so that each has its own `use` statement.

## References {#references}

- [Bringing Paths into Scope with the use Keyword (Rust book)](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)↗.
- [Use declarations (Rust reference)](https://doc.rust-lang.org/reference/items/use-declarations.html)↗.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
