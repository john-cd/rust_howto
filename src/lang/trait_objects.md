# Trait Objects

In Rust, {{i:traits}} are types, but they are "{{i:unsized}}", which roughly means that they are only allowed to show up behind a pointer like [`Box`][std::boxed::Box]⮳ (which points onto the heap) or `&` (which can point anywhere).

A type like `&ClickCallback` or `Box<dyn ClickCallback>` where `ClickCallback` is a Trait is called a "trait object", and includes a pointer to an instance of a type `T` implementing `ClickCallback`, and a {{i:vtable}}: a pointer to `T`'s implementation of each method in the trait.

```rust,editable
{{#include ../../deps/tests/trait_objects.rs}}
```

The set of traits after [`{{i:dyn}}`][dyn] is made up of an [object-safe-reference][book-rust-reference-object-safe]⮳ base trait plus any number of {{i:autotraits}} (one of [`Send`][std::marker::Send]⮳, [`Sync`][std::marker::Sync]⮳, [`Unpin`][std::marker::Unpin]⮳, [`UnwindSafe`][std::panic::UnwindSafe]⮳, and [`RefUnwindSafe`][std::panic::RefUnwindSafe]⮳ - see [special traits][book-rust-reference-special-traits]⮳).

```rust,editable,ignore
dyn Trait
dyn Trait + Send
dyn Trait + Send + Sync
dyn Trait + 'static
```

## See also

[Trait Objects (docs)][book-rust-trait-objects]⮳

{{#include ../refs/link-refs.md}}
