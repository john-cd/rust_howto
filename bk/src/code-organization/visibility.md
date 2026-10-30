<!-- TODO Reviewed ??-??-2026 -->

# Visibility

{{#include visibility.incl.md}}

## Make your Code Items Public with the `pub` Keyword {#public}

[![book~rust-by-example~visibility-rules][book~rust-by-example~visibility-rules~badge]][book~rust-by-example~visibility-rules]

In Rust, all items (modules, [functions][p~functions], methods, [structs][p~structs], [enums][p~enums], constants...) are _private by default_.

Use the [`pub`][keyword~pub]↗{{hi:pub}} keyword before an item's definition to make it public:

```rust,editable
{{#include ../../crates/code_organization/examples/visibility/pub_keyword.rs:example}}
```

There are two exceptions to the "privacy by default" rule: Enum variants in a [`pub enum`][keyword~enum]↗{{hi:pub enum}} are public by default. Associated items (incl. functions and methods) in a `pub` [[traits | trait]] are also public by default.

Also note that, if we use [`pub`][keyword~pub]↗{{hi:pub}} before a struct definition, we make the struct public, but the fields of the struct will still be private. Mark relevant fields with `pub`:

```rust,editable
{{#include ../../crates/code_organization/examples/visibility/public_by_default.rs:example}}
```

As discussed below, the scope where an item is visible can be specified after the [`pub`][keyword~pub]↗{{hi:pub}} keyword: `pub(crate)`, `pub(super)`, `pub(in _path_)`...

## Make your Code Items Accessible (Visibility Rules) {#visibility-rules}

Rust visibility rules are as follows:

- If an item is _private_, it may be accessed by the _current module_ and its _descendants_ only.

```rust,editable
{{#include ../../crates/code_organization/examples/visibility/private_access.rs:example}}
```

- An item is accessible from a module `m` if all the item's ancestor modules are visible from `m`. For code in another crate, each module between the crate root and the item must be public or re-exported.

In other words,

- Items can access other items in the _same_ module, even if private.
- Items in a parent module can't use private items{{hi:Private items}} inside child modules (encapsulation).
- Items in child modules can use all items in their ancestor modules.

```rust,editable
{{#include ../../crates/code_organization/examples/visibility/public_access.rs:example}}
```

More precisely, _an item is accessible if all segments in its path are accessible_. When a path goes from a parent module to a child, each enclosing module must have a visibility scope that includes the caller (see below). If an enclosing module is private, a suitable re-export can expose the item; see the [`pub use`][book~rust-reference~use-declarations-visibility]↗ section of the [[use_declarations | `use` keyword]] chapter for details.

### Make your Code Items Visible to your Library's Clients {#visible-to-library-clients}

A library developer needs to expose functionality to crates which link against their library. Anything which is usable externally must be [`pub`][keyword~pub]↗{{hi:pub}} from the crate root down to the destination item. Any private item in the chain will disallow external accesses.

In particular, that means marking the relevant modules in the crate root (e.g. `lib.rs`) as `pub`.

```rust,editable
{{#include ../../crates/code_organization/examples/visibility/visibility_external_code.rs:example}}
```

### Limit the Visibility of an Item to a Given Scope {#visibility-scope}

You can declare an item (type, function, module...) as visible only within a given scope by prefixing it with `pub(crate)`, `pub(super)` or `pub(in` _path_`)` rather than just `pub`:

- `pub(crate)` makes an item visible within the current crate only.
- `pub(super)` makes an item visible to the parent module only.
- `pub(in _path_)` makes an item visible within the provided _path_, which must start with `crate`, `self`, or `super` and resolves to an ancestor module of the item whose visibility is being declared. This form is seen infrequently.

The following demonstrates this syntax:

```rust,editable
{{#include ../../crates/code_organization/examples/visibility/visibility_scope.rs:example}}
```

When writing a library or in contexts where the public API must remain stable, `pub(crate)` and related forms are safer to use than `pub` alone, in the sense they are a safeguard against accidentally making an item fully public to external crates linking the library.

```rust,editable
{{#include ../../crates/code_organization/examples/visibility/visibility_pub_crate.rs:example}}
```

## References {#references .skip}

- [Visibility and Privacy (Rust reference)][book~rust-reference~visibility-and-privacy]↗.

## Related Topics {#related-topics .skip}

- [[modules_and_paths | Modules and Paths]].
- [[use_declarations | Use Declarations]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
  [accuracy review: code organization](https://github.com/john-cd/rust_howto/issues/1890)
</div>
