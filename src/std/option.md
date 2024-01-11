# Option

Rust has no `null`. Instead, use [Option][std::option::Option]⮳:

```rust,editable
enum Option<T> {
    None,
    Some(T),
}
# fn main() {}
```

Every `Option` is either `Some` and contains a value, or `None`, and does not.

```rust,editable
{{#include ../../deps/examples/options.rs}}
```

It is often used with `match`, `if let`, or `while let`:

```rust,editable
{{#include ../../deps/examples/options2.rs}}
```

## Adapters for working with references

- `as_ref` converts from `&Option<T>` to `Option<&T>`
- `as_mut` converts from `&mut Option<T>` to `Option<&mut T>`
- `as_deref` converts from `&Option<T>` to `Option<&T::Target>`
- `as_deref_mut` converts from `&mut Option<T>` to `Option<&mut T::Target>`

## Extracting the value contained in Option

These methods extract the contained value in an `Option<T>` when it is the Some variant.
If the `Option` is None:

- `expect` panics with a provided custom message
- `unwrap` panics with a generic message
- `unwrap_or` returns the provided default value
- `unwrap_or_default` returns the default value of the type T (which must implement the `Default` trait)
- `unwrap_or_else` returns the result of evaluating the provided function

## Combinators

```rust,editable,no_run
{{#include ../../deps/examples/options3.rs}}
```

{{#include ../refs/link-refs.md}}
