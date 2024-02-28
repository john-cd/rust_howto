# Option

{{#include option.incl.md}}

[![std][std-badge]][std]

Rust has no {{i:`null`}}. Instead, use [`std::option::Option`][std::option::Option]⮳:

```rust,editable
# #![allow(unused)]
enum Option<T> {
    None,
    Some(T),
}
# fn main() {}
```

Every {{i:`Option`}} is either {{i:`Some`}} and contains a value, or {{i:`None`}}, and does not.

```rust,editable
{{#include ../../deps/tests/options.rs}}
```

It is often used with {{i:`match`}}, {{i:`if let`}}, or {{i:`while let`}}:

```rust,editable
{{#include ../../deps/tests/options2.rs}}
```

## Adapters for working with references

- {{i:`as_ref`}} converts from `&Option<T>` to `Option<&T>`
- {{i:`as_mut`}} converts from `&mut Option<T>` to `Option<&mut T>`
- {{i:`as_deref`}} converts from `&Option<T>` to `Option<&T::Target>`
- {{i:`as_deref_mut`}} converts from `&mut Option<T>` to `Option<&mut T::Target>`

## Extracting the value contained in Option

These methods extract the contained value in an `Option<T>` when it is the Some variant. If the `Option` is None:

- {{i:`expect`}} panics with a provided custom message
- {{i:`unwrap`}} panics with a generic message
- {{i:`unwrap_or`}} returns the provided default value
- {{i:`unwrap_or_default`}} returns the default value of the type T (which must implement the `Default` trait)
- {{i:`unwrap_or_else`}} returns the result of evaluating the provided function

## Combinators

```rust,editable,no_run
{{#include ../../deps/tests/options3.rs}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
