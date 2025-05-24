# Visibility

{{#include visibility.incl.md}}

## Make your Code Items Public with the `pub` Keyword {#public}

[![book-rust-by-example-visibility-rules][book-rust-by-example-visibility-rules-badge]][book-rust-by-example-visibility-rules]

In Rust, all items (modules, [functions][p-functions], methods, [structs][p-structs], [enums][p-enums], constants...) are _private by default_.

Use the `pub` keyword before an item's definition to make it public:

```rust,editable
{{#include ../../crates/code_organization/tests/visibility/pub_keyword.rs:example}}
```

There are two exceptions to the "privacy by default" rule: Enum variants in a `pub enum` are public by default. Associated items in a `pub` [[trait]] are also public by default.

Also note that, if we use `pub` before a struct definition, we make the struct public, but the fields of the struct will still be private. Mark relevant fields with `pub`:

```rust,editable
{{#include ../../crates/code_organization/tests/visibility/public_by_default.rs:example}}
```

As discussed below, the scope where an item is visible can be specified after the `pub` keyword: `pub(crate)`, `pub(super)`, `pub(in` _path_`)`...

## Make your Code Items Accessible (Visibility Rules) {#visibility-rules}

Rust visibility rules are as follows:

- If an item is _private_, it may be accessed by the _current module_ and its _descendants_ only.

```rust,editable
{{#include ../../crates/code_organization/tests/visibility/private_access.rs:example}}
```

- If an item is _public_, then it can be accessed externally from some module `m`, if you can access all the item's ancestor modules from `m`.

In other words,

- Items can access other items in the _same_ module, even if private.
- Items in a parent module can't use private items{{hi:Private items}} inside child modules (encapsulation).
- Items in child modules can use all items in their ancestor modules.

```rust,editable
{{#include ../../crates/code_organization/tests/visibility/public_access.rs:example}}
```

More precisely, _an item is accessible if all segments in its path are accessible_. That means that all enclosing modules traversed in the "parent to child" direction must be `pub` with the appropriate visibility scope (see below), or, if an enclosing module is not, there must be a suitable reexport - see the `pub use` section of the[[use_keyword | `use` Keyword]] chapter for more details.

### Make your Code Items Visible to your Library's Clients {#visible-to-library-clients}

A library developer needs to expose functionality to crates which link against their library. Anything which is usable externally must be `pub` from the crate root down to the destination item. Any private item in the chain will disallow external accesses.

In particular, that means marking the relevant modules in the crate root (e.g. `lib.rs`) as `pub`.

```rust,editable
{{#include ../../crates/code_organization/tests/visibility/visibility_external_code.rs:example}}
```

### Limit the Visibility of an Item to a Given Scope {#visibility-scope}

You can declare an item (type, function, module...) as visible only within a given scope by prefixing it with `pub(crate)`, `pub(super)` or `pub(in` _path_`)` rather than just `pub`:

- `pub(crate)` makes an item visible within the current crate only.
- `pub(super)` makes an item visible to the parent module only.
- `pub(in` _path_`)` makes an item visible within the provided _path_, which must start with `crate`, `self`, or `super` and resolves to an ancestor module of the item whose visibility is being declared. This form is seen infrequently.

The following demonstrates this syntax:

```rust,editable
{{#include ../../crates/code_organization/tests/visibility/visibility_scope.rs:example}}
```

When writing a library or in contexts where the public API must remain stable, `pub(crate)` and related forms are safer to use than `pub` alone, in the sense they are a safeguard against accidentally making an item fully public to external crates linking the library.

```rust,editable
{{#include ../../crates/code_organization/tests/visibility/visibility_pub_crate.rs:example}}
```

## References {#skip}

- [Visibility and Privacy (Rust reference)](https://doc.rust-lang.org/reference/visibility-and-privacy.html?highlight=pub#visibility-and-privacy)⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
