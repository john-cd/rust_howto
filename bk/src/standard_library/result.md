# Result

{{#include result.incl.md}}

## Handle Recoverable Errors with `Result` {#result}

[![std][c~std~docs~badge]][c~std~docs]

[`Result<T, E>`][c~std::result::Result~docs]↗{{hi:Result}} is an `enum` very frequently used for error handling - specifically when errors are _expected_ and _recoverable_.

`Result<T, E>` has two variants:

- [`Ok(T)`][c~std::result::Result::Ok~docs]↗{{hi:std::result::Result::Ok}} represents success and contains a value of type `T`.
- [`Err(E)`][c~std::result::Result::Err~docs]↗{{hi:std::result::Result::Err}} represents an error and contains an error value of type `E`.

`E` above often implements the [`std::error::Error`][c~std::error::Error~docs]↗{{hi:std::error::Error}} trait, which is a standard trait for reporting errors in Rust. You can define your own error types by implementing this trait (but this is verbose) or use libraries like `thiserror` or `anyhow` to greatly simplify error handling.

`Result` is annotated with the `#[must_use]` attribute: the compiler issues a warning when a `Result` value is ignored, prompting you if you forgot to handle an error.

The following example show to return `Result` from a function, propagate it to the caller with the `?` operator, or process it with `match`:

```rust,editable
{{#include ../../crates/standard_library/examples/result/result.rs:example}}
```

## Retrieve or Transform Values in `Result` {#result-methods}

[![std][c~std~docs~badge]][c~std~docs]

In addition to working with pattern matching, [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} provides a wide variety of convenience methods, the most common being:

| Method       | Description | Use Case |
|-------------|------------|----------|
| [`unwrap()`][c~std::result::Result::unwrap~docs]↗{{hi:std::result::Result::unwrap}} | Returns the `Ok` value or panics if `Err`. | Use when you're certain the result is `Ok`. See also `expect`. |
| [`unwrap_or(default)`][c~std::result::Result::unwrap_or_default~docs]↗{{hi:std::result::Result::unwrap_or}} | Returns the `Ok` value or a default if `Err`. | Use when you want a fallback value. |
| [`unwrap_or_else(func)`][c~std::result::Result::unwrap_or_else~docs]↗{{hi:std::result::Result::unwrap_or_else}}  | Returns the `Ok` value or calls a function to generate a fallback. | Use when computing a fallback dynamically. |
| [`map(func)`][c~std::result::Result::map~docs]↗{{hi:std::result::Result::map}} | Applies a function to the `Ok` value. | Use to transform the result, if successful. |
| [`map_err(func)`][c~std::result::Result::map_err~docs]↗{{hi:std::result::Result::map_err}} | Applies a function to the `Err` value. | Use to transform the error. |
| [`and_then(func)`][c~std::result::Result::and_then~docs]↗{{hi:std::result::Result::and_then}} | Applies a function that returns `Result`, flattening the result. | Use for chaining operations that may fail. |
| [`or_else(func)`][c~std::result::Result::or_else~docs]↗{{hi:std::result::Result::or_else}} | Calls a function to provide an alternative `Result`, if `Err`. | Use to recover from errors dynamically. |
| [`map_or(default, func)`][c~std::result::Result::map_or~docs]↗{{hi:std::result::Result::map_or}} | Applies a function to the `Ok` value or returns a default if `Err`. | Use when you need a fallback value. |
| [`map_or_else(err_func, ok_func)`][c~std::result::Result::map_or_else~docs]↗{{hi:std::result::Result::map_or_else}} | Applies a function to the `Ok` value or calls another function for `Err`. | Use when you need dynamic error handling. |
| [`is_ok()`][c~std::result::Result::is_ok~docs]↗{{hi:std::result::Result::is_ok}} | Returns `true` if the result is `Ok`. | Use to check success. |
| [`is_err()`][c~std::result::Result::is_err~docs]↗{{hi:std::result::Result::is_err}} | Returns `true` if the result is `Err`. | Use to check failure. |

The following example demonstrates a few of these methods:

```rust,editable
{{#include ../../crates/standard_library/examples/result/result2.rs:example}}
```

## Get Reference to Values Inside a `Result` {#result-references}

[![std][c~std~docs~badge]][c~std~docs]

The following example demonstrates the usage of the `as_ref()`, `as_deref()`, and `as_mut()` methods on the `Result` type:

- [`as_ref()`][c~std::result::Result::as_ref~docs]↗{{hi:std::result::Result::as_ref}} converts `&Result<T, E>` into `Result<&T, &E>`, allowing access to references.
- [`as_deref()`][c~std::result::Result::as_deref~docs]↗{{hi:std::result::Result::as_deref}} converts `&Result<T, E>` into `Result<&U, &E> where T: Deref<Target = U>`, useful for converting `Result<String, E>` to `Result<&str, E>`.
- [`as_mut()`][c~std::result::Result::as_mut~docs]↗{{hi:std::result::Result::as_mut}} converts `&mut Result<T, E>` into `Result<&mut T, &mut E>`, allowing mutation of the inner value.

These methods are useful for working with references to the values inside a `Result` without consuming the `Result` itself.

```rust,editable
{{#include ../../crates/standard_library/examples/result/result3.rs:example}}
```

## Choose between `Result` and `Option` {#result-vs-option}

[![std][c~std~docs~badge]][c~std~docs]

[`Option`][c~std::option::Option~docs]↗{{hi:std::option::Option}} represents an optional value. Use `Option` when a value might be missing but isn't necessarily an error (e.g., searching for an item in a list).

[`Result<T, E>`][c~std::result::Result~docs]↗{{hi:std::result::Result}} represents the outcome of an operation that can succeed or fail. Use `Result` when an operation can fail and you need to handle errors explicitly (e.g., file I/O operations).

The following table lists common methods that convert `Result<T, E>` to `Option`:

| Method | Description | Use Case |
| [`ok()`][c~std::result::Result::ok~docs]↗{{hi:std::result::Result::ok}} | Converts `Result<T, E>` into `Option<T>`, discarding the error. | Use when you only care about the success value. |
| [`err()`][c~std::result::Result::err~docs]↗{{hi:std::result::Result::err}} | Converts `Result<T, E>` into `Option<E>`, discarding the success value. | Use when you only care about the error. |

See also the [[option | Option]] chapter.

## Related Topics {#related-topics .skip}

- [[iterators | Iterators]].

## References {#references .skip}

- [What is the Rust equivalent to a `try catch` statement][stackoverflow~what-is-the-rust-equivalent-to-a-try-catch-statement]↗.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

- [[error_handling | Error Handling]].
- [[error_customization | Error Customization]].
- [[rust-patterns | Rust Patterns]].

</div>
