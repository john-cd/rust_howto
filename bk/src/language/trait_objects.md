# Trait Objects

{{#include trait_objects.incl.md}}

In Rust, [traits][p-traits]{{hi:Traits}} are types, but they are "unsized"{{hi:Unsized}}, which roughly means that they are only allowed to show up behind a pointer like [`std::boxed::Box`][c-std::boxed::Box]{{hi:std::boxed::Box}}⮳ (which points onto the heap) or `&` (which can point anywhere).

A type like `&dyn ClickCallback` or `Box<dyn ClickCallback>` where `ClickCallback` is a Trait, is called a "trait object", and includes a pointer to an instance of a type `T` implementing `ClickCallback`, and a 'vtable'{{hi:Vtable}}: a pointer to `T`'s implementation of each method in the trait.

```rust,editable
{{#include ../../crates/language/tests/feat/trait_objects.rs:example}}
```

The set of [traits][p-traits] after [`dyn`][keyword-dyn]{{hi:dyn}} can be made up of an [object-safe][book-rust-reference-object-safe]⮳ base trait plus any number of autotraits{{hi:Autotraits}} (one of [`std::marker::Send`][c-std::marker::Send]{{hi:std::marker::Send}}⮳, [`std::marker::Sync`][c-std::marker::Sync]{{hi:std::marker::Sync}}⮳, [`std::marker::Unpin`][c-std::marker::Unpin]{{hi:std::marker::Unpin}}⮳, [`std::panic::UnwindSafe`][c-std::panic::UnwindSafe]{{hi:std::panic::UnwindSafe}}⮳, and [`std::panic::RefUnwindSafe`][c-std::panic::RefUnwindSafe]{{hi:std::panic::RefUnwindSafe}}⮳ - see [special traits][book-rust-reference-special-traits]⮳).

```rust,editable,compile_fail,noplayground
dyn Trait
dyn Trait + Send
dyn Trait + Send + Sync
dyn Trait + 'static
```

## See Also {#skip}

[Trait Objects (docs)][book-rust-trait-objects]{{hi:Trait objects}}⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[trait_objects: review NOW](https://github.com/john-cd/rust_howto/issues/562)
</div>
