# Option

{{#include option.incl.md}}

[![std][std-badge]][std]

Rust has no `{{i:null}}`. Instead, use [`{{i:std::option::Option}}`][c-std::option::Option]⮳:

```rust,editable
# #![allow(unused)]
enum Option<T> {
    None,
    Some(T),
}
# fn main() {}
```

Every [`{{i:Option}}`][c-std::option::Option]⮳ is either [`{{i:Some}}`][c-std::option::Option::Some]⮳ and contains a value, or [`{{i:None}}`][c-std::option::Option::None]⮳, and does not.

```rust,editable
{{#include ../../deps/tests/options.rs}}
```

It is often used with [`{{i:match}}`][book-rust-reference-match]⮳, [`{{i:if let}}`][book-rust-reference-if], or [`{{i:while let}}`][book-rust-reference-while-let]:

```rust,editable
{{#include ../../deps/tests/options2.rs}}
```

## Adapters for working with references

- [`{{i:as_ref}}`][c-std::convert::AsRef]⮳ converts from `&Option<T>` to `Option<&T>`
- [`{{i:as_mut}}`][c-std::convert::AsMut]⮳ converts from `&mut Option<T>` to `Option<&mut T>`
- [`{{i:as_deref}}`][c-std::option::Option::as_deref]⮳ converts from `&Option<T>` to `Option<&T::Target>`
- [`{{i:as_deref_mut}}`][c-std::option::Option::as_deref_mut]⮳ converts from `&mut Option<T>` to `Option<&mut T::Target>`

## Extracting the value contained in Option

These methods extract the contained value in an [`{{i:Option<T>}}`][c-std::option::Option] when it is the `Some` variant. If the [`{{i:Option}}`][c-std::option::Option]⮳ is `None`:

- [`{{i:expect}}`][c-std::option::Option::expect]⮳ panics with a provided custom message
- [`{{i:unwrap}}`][c-std::option::Option::unwrap]⮳ panics with a generic message
- [`{{i:unwrap_or}}`][c-std::option::Option::unwrap_or]⮳ returns the provided default value
- [`{{i:unwrap_or_default}}`][c-std::option::Option::unwrap_or_default]⮳ returns the default value of the type T (which must implement the [`{{i:Default}}`][c-std::default::Default]⮳ trait)
- [`{{i:unwrap_or_else}}`][c-std::option::Option::unwrap_or_else]⮳ returns the result of evaluating the provided function

## Combinators

```rust,editable,no_run
{{#include ../../deps/tests/options3.rs}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
