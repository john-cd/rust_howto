# Result

{{#include result.incl.md}}

## Handle Recoverable Errors with `Result` {#result}

[![std][c-std-badge]][c-std]

[`Result<T, E>`][c-std::result::Result]⮳{{hi:Result}} is an enum used for error handling. It has two variants:

- `Ok(T)` represents success and contains a value of type `T`.
- `Err(E)` represents an error and contains an error value of type `E`.

Functions should return `Result` when errors are expected and recoverable. Note that `Result` is annotated with the `#[must_use]` attribute: the compiler issues a warning when a `Result` value is ignored.

The following example show to return `Result` from a function, propagate it to the caller with the `?` operator, or process it with `match`. The `anyhow` and `thiserror` libraries can be used to significantly simplify error handling.

```rust,editable
{{#include ../../crates/standard_library/tests/result/result.rs:example}}
```

## Retrieve or Transform Values in `Result` {#result-methods}

[![std][c-std-badge]][c-std]

In addition to working with pattern matching, `Result` provides a wide variety of convenience methods, the most common being:

| Method       | Description | Use Case |
|-------------|------------|----------|
| `unwrap()`  | Returns the `Ok` value or panics if `Err`. | Use when you're certain the result is `Ok`. See also `expect`. |
| `unwrap_or(default)` | Returns the `Ok` value or a default if `Err`. | Use when you want a fallback value. |
| `unwrap_or_else(func)` | Returns the `Ok` value or calls a function to generate a fallback. | Use when computing a fallback dynamically. |
| `map(func)` | Applies a function to the `Ok` value. | Use to transform the result, if successful. |
| `map_err(func)` | Applies a function to the `Err` value. | Use to transform the error. |
| `and_then(func)` | Applies a function that returns `Result`, flattening the result. | Use for chaining operations that may fail. |
| `or_else(func)` | Calls a function to provide an alternative `Result`, if `Err`. | Use to recover from errors dynamically. |
| `map_or(default, func)` | Applies a function to the `Ok` value or returns a default if `Err`. | Use when you need a fallback value. |
| `map_or_else(err_func, ok_func)` | Applies a function to the `Ok` value or calls another function for `Err`. | Use when you need dynamic error handling. |
| `is_ok()` | Returns `true` if the result is `Ok`. | Use to check success. |
| `is_err()` | Returns `true` if the result is `Err`. | Use to check failure. |

The following example demonstrates a few of these methods:

```rust,editable
{{#include ../../crates/standard_library/tests/result/result2.rs:example}}
```

## Get Reference to Values Inside a `Result` {#result-references}

[![std][c-std-badge]][c-std]

The following example demonstrates the usage of the `as_ref()`, `as_deref()`, and `as_mut()` methods on the `Result` type:

- `as_ref()` converts `&Result<T, E>` into `Result<&T, &E>`, allowing access to references.
- `as_deref()` converts `&Result<T, E>` into `Result<&U, &E> where T: Deref<Target = U>`, useful for converting `Result<String, E>` to `Result<&str, E>`.
- `as_mut()` converts `&mut Result<T, E>` into `Result<&mut T, &mut E>`, allowing mutation of the inner value.

These methods are useful for working with references to the values inside a `Result` without consuming the `Result` itself.

```rust,editable
{{#include ../../crates/standard_library/tests/result/result3.rs:example}}
```

## `Result` vs `Option` {#result-vs-option}

[![std][c-std-badge]][c-std]

`Option<T>` represents an optional value. Use `Option` when a value might be missing but isn't necessarily an error (e.g., searching for an item in a list).

`Result<T, E>` represents the outcome of an operation that can succeed or fail. Use `Result` when an operation can fail and you need to handle errors explicitly (e.g., file I/O operations).

The following table lists common methods that convert `Result<T, E>` to `Option`:

| Method | Description | Use Case |
| `ok()` | Converts `Result<T, E>` into `Option<T>`, discarding the error. | Use when you only care about the success value. |
| `err()` | Converts `Result<T, E>` into `Option<E>`, discarding the success value. | Use when you only care about the error. |

See also the [[option | Option]] chapter.

## Related Topics {#skip}

- [[error_handling | Error Handling]].
- [[error_customization | Error Customization]].
- [[iterators | Iterators]].
- [[rust-patterns | Rust Patterns]].

## References {#skip}

- [What is the Rust equivalent to a `try catch` statement][stackoverflow-what-is-the-rust-equivalent-to-a-try-catch-statement]⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[result: review](https://github.com/john-cd/rust_howto/issues/626)
</div>
