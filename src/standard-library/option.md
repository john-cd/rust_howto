# Option

{{#include option.incl.md}}

## `Option` {#option}

[![std][c-std-badge]][c-std]{{hi:std}}

Rust has no `null`{{hi:null}}. Instead, use [`std::option::Option`][c-std::option::Option]{{hi:std::option::Option}}⮳:

```rust,editable
enum Option<T> {
  None,
  Some(T),
}
```

Every [`std::option::Option`][c-std::option::Option]{{hi:std::option::Option}}⮳ is either [`std::option::Option::Some`][c-std::option::Option::Some]{{hi:std::option::Option::Some}}⮳ and contains a value, or [`std::option::Option::None`][c-std::option::Option::None]{{hi:std::option::Option::None}}⮳, and does not.

```rust,editable
{{#include ../../deps/tests/standard_library/options1.rs:example}}
```

It is often used with [`match`][book-rust-reference-match]{{hi:match}}⮳, [`if let`][book-rust-reference-if]{{hi:if let}}, or [`while let`][book-rust-reference-while-let]{{hi:while let}}:

```rust,editable
{{#include ../../deps/tests/standard_library/options2.rs:example}}
```

## Adapters for working with references {#adapters-for-working-with-references}

[![std][c-std-badge]][c-std]

- [`std::convert::AsRef`][c-std::convert::AsRef]{{hi:std::convert::AsRef}}⮳ converts from `&Option<T>` to `Option<&T>`
- [`std::convert::AsMut`][c-std::convert::AsMut]{{hi:std::convert::AsMut}}⮳ converts from `&mut Option<T>` to `Option<&mut T>`
- [`std::option::Option::as_deref`][c-std::option::Option::as_deref]{{hi:std::option::Option::as_deref}}⮳ converts from `&Option<T>` to `Option<&T::Target>`
- [`std::option::Option::as_deref_mut`][c-std::option::Option::as_deref_mut]{{hi:std::option::Option::as_deref_mut}}⮳ converts from `&mut Option<T>` to `Option<&mut T::Target>`

## Extracting the value contained in Option {#extracting-the-value-contained-in-option}

![std][c-std-badge]][c-std]

These methods extract the contained value in an [`std::option::Option`][c-std::option::Option]{{hi:std::option::Option}} when it is the `Some` variant. If the [`std::option::Option`][c-std::option::Option]{{hi:std::option::Option}}⮳ is `None`:

- [`std::option::Option::expect`][c-std::option::Option::expect]{{hi:std::option::Option::expect}}⮳ panics with a provided custom message
- [`std::option::Option::unwrap`][c-std::option::Option::unwrap]{{hi:std::option::Option::unwrap}}⮳ panics with a generic message
- [`std::option::Option::unwrap_or`][c-std::option::Option::unwrap_or]{{hi:std::option::Option::unwrap_or}}⮳ returns the provided default value
- [`std::option::Option::unwrap_or_default`][c-std::option::Option::unwrap_or_default]{{hi:std::option::Option::unwrap_or_default}}⮳ returns the default value of the type T (which must implement the [`std::default::Default`][c-std::default::Default]{{hi:std::default::Default}}⮳ trait)
- [`std::option::Option::unwrap_or_else`][c-std::option::Option::unwrap_or_else]{{hi:std::option::Option::unwrap_or_else}}⮳ returns the result of evaluating the provided function

## Combinators {#combinators}

[![std][c-std-badge]][c-std]

```rust,editable
{{#include ../../deps/tests/standard_library/options3.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

{{hi:Options}}
<div class="hidden">
TODO P1: finish Option page map, unwrap_or
</div>
