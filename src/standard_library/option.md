# Option

{{#include option.incl.md}}

[![std][c-std-badge]][c-std]

Rust has no `null`{{hi:null}}. Instead, use {{hi:std::option::Option}}[`std::option::Option`][c-std::option::Option]⮳:

```rust,editable
# #![allow(unused)]
enum Option<T> {
    None,
    Some(T),
}
# fn main() {}
```

Every {{hi:Option}}[`Option`][c-std::option::Option]⮳ is either {{hi:Some}}[`Some`][c-std::option::Option::Some]⮳ and contains a value, or {{hi:None}}[`None`][c-std::option::Option::None]⮳, and does not.

```rust,editable
{{#include ../../deps/tests/options.rs}}
```

It is often used with {{hi:match}}[`match`][book-rust-reference-match]⮳, {{hi:if let}}[`if let`][book-rust-reference-if], or {{hi:while let}}[`while let`][book-rust-reference-while-let]:

```rust,editable
{{#include ../../deps/tests/options2.rs}}
```

## Adapters for working with references

- {{hi:as_ref}}[`as_ref`][c-std::convert::AsRef]⮳ converts from `&Option<T>` to `Option<&T>`
- {{hi:as_mut}}[`as_mut`][c-std::convert::AsMut]⮳ converts from `&mut Option<T>` to `Option<&mut T>`
- {{hi:as_deref}}[`as_deref`][c-std::option::Option::as_deref]⮳ converts from `&Option<T>` to `Option<&T::Target>`
- {{hi:as_deref_mut}}[`as_deref_mut`][c-std::option::Option::as_deref_mut]⮳ converts from `&mut Option<T>` to `Option<&mut T::Target>`

## Extracting the value contained in Option

These methods extract the contained value in an {{hi:Option<T>}}[`Option<T>`][c-std::option::Option] when it is the `Some` variant. If the {{hi:Option}}[`Option`][c-std::option::Option]⮳ is `None`:

- {{hi:expect}}[`expect`][c-std::option::Option::expect]⮳ panics with a provided custom message
- {{hi:unwrap}}[`unwrap`][c-std::option::Option::unwrap]⮳ panics with a generic message
- {{hi:unwrap_or}}[`unwrap_or`][c-std::option::Option::unwrap_or]⮳ returns the provided default value
- {{hi:unwrap_or_default}}[`unwrap_or_default`][c-std::option::Option::unwrap_or_default]⮳ returns the default value of the type T (which must implement the {{hi:Default}}[`Default`][c-std::default::Default]⮳ trait)
- {{hi:unwrap_or_else}}[`unwrap_or_else`][c-std::option::Option::unwrap_or_else]⮳ returns the result of evaluating the provided function

## Combinators

```rust,editable,no_run
{{#include ../../deps/tests/options3.rs}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
