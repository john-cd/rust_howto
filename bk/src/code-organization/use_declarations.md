<!-- TODO Reviewed ??-??-2026. -->

# `use` Declarations

{{#include use_declarations.incl.md}}

Within a scope, the [`use`][keyword~use]↗{{hi:use}} keyword creates shortcuts to items to reduce repetition of long paths.

## Avoid Writing Full Paths with the `use` Keyword {#use-keyword}

[![book~rust-by-example~use][book~rust-by-example~use~badge]][book~rust-by-example~use]

The [`use`][book~rust-reference~use]↗{{hi:use}} keyword creates a shortcut for a path{{hi:Path}}. The shorter name can be used everywhere else in the scope.

```rust,editable
{{#include ../../crates/code_organization/examples/use/use1.rs:example}}
```

## Bring an Item from an External Crate into Scope {#bring-item-from-external-crate-into-scope}

To bring items from an external crate into scope, without having to write their full paths every time, write a [`use`][book~rust-reference~use-declarations]↗{{hi:use}} declaration with a path that begins with the external crate name. See also the [[dependencies | Dependencies]] chapter.

```rust,editable
{{#include ../../crates/code_organization/examples/use/use_external_crate.rs:example}}
```

## Bring a Function in Scope Idiomatically {#bring-function-in-scope}

When calling a function from another module, conventionally import its parent module and call `a_module::a_function()`. The qualifier shows where the function comes from.

```rust,editable
{{#include ../../crates/code_organization/examples/use/use2.rs:example}}
```

## Bring a Struct or Enum in Scope Idiomatically {#bring-struct-or-enum-in-scope}

For [structs][p~structs] and [enums][p~enums], conventionally import the type itself with a [`use`][keyword~use]↗{{hi:use}} declaration. You can then refer to the type by name.

```rust,editable
{{#include ../../crates/code_organization/examples/use/use3.rs:example}}
```

If two imports would introduce the same type name, qualify each type with its module or give one import an alias with `as`.

## Reexport Items with `pub use` {#pub-use}

[`use`][book~rust-reference~use-declarations]↗{{hi:use}} declarations are private to their containing module by default. A `use` declaration can be made public by the `pub` keyword. Such a declaration re-exports a name. See also the [[visibility | Visibility]] chapter.

```rust,editable
{{#include ../../crates/code_organization/examples/use/pub_use.rs:example}}
```

## Make `use` Declarations More Compact {#use-declaration-shortcuts}

Use declarations support a number of convenient shortcuts. You may use globs, define aliases, and combine multiple [`use`][keyword~use]↗{{hi:use}} statements.

```rust,editable
{{#include ../../crates/code_organization/examples/use/use_shortcuts.rs:example}}
```

Note that, while these shortcuts make your code more compact and readable, they can be inconvenient during early development, when you need to add or remove [`use`][keyword~use]↗{{hi:use}} statements frequently. Consider adding a [`rustfmt.toml`][rustfmt.toml]↗ configuration file to your project and adding `imports_granularity = "Item"` to flatten imports, so that each has its own `use` statement.

## References {#references .skip}

- [Bringing Paths into Scope with the use Keyword (Rust book)][book~rust~ch07-04-bringing-paths-into-scope-with-the-use-keyword]↗.
- [Use declarations (Rust reference)][book~rust-reference~use-declarations]↗.

## Related Topics {#related-topics .skip}

- [[dependencies | Dependencies]].
- [[modules_and_paths | Modules and Paths]].
- [[visibility | Visibility]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
