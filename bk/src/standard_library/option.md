# Option

{{#include option.incl.md}}

## Store Optional Values in `Option` {#option}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}{{hi:null}}

Rust has no `null`. Instead, it represents the presence or absence of a value with the [`std::option::Option`][c~std::option::Option~docs]{{hi:std::option::Option}}↗ enum:

```rust,editable,noplayground
enum Option<T> {
  None,
  Some(T),
}
```

Every [`std::option::Option`][c~std::option::Option~docs]{{hi:std::option::Option}}↗ is either [`std::option::Option::Some`][c~std::option::Option::Some~docs]{{hi:std::option::Option::Some}}↗ and contains a value, or [`std::option::Option::None`][c~std::option::Option::None~docs]{{hi:std::option::Option::None}}↗, and does not.

```rust,editable
{{#include ../../crates/standard_library/examples/option/options1.rs:example}}
```

`Option<T>` is similar to the concept of "nullable" types in other languages, but with a crucial difference: the `Option` type forces you to explicitly handle both the "value present" and "value absent" cases, preventing common `null` pointer errors.

## Use `Option` with `match`, `if let`, or `while let` {#option-match-if-let-while-let}

`Option<T>` is often used with [`match`][book~rust-reference~match]{{hi:match}}↗, [`if let`][book~rust-reference~if]{{hi:if let}}, or [`while let`][book~rust-reference~while-let]{{hi:while let}}:

```rust,editable
{{#include ../../crates/standard_library/examples/option/options2.rs:example}}
```

```rust,editable
{{#include ../../crates/standard_library/examples/option/option_match.rs:example}}
```

## Implement the "Early Return" Pattern with the `?` Operator {#return-early-pattern}

[![std][c~std~docs~badge]][c~std~docs]

"Early Exit" (also called "Early Return") is the technique of immediately returning from a function (or exiting a loop) as soon as an invalid or exceptional condition is detected. For this purpose, Rust offers the `?` operator, which is essentially a shortcut for a `match` expression:

```rust,noplayground
fn func() -> Option<...> {
   let maybe_val: Option<...> = ...;
   let val = match maybe_val {
      Some(v) => v,
      _ => return None,  // Early exit.
   }
 // ...
 Some(val)
}
```

In the example below, when `s.chars().next()?` encounters `None`, the function is exited early (returning `None`), guarding the subsequent uppercase conversion logic from operating on an empty string.

This pattern removes the need for explicit, convoluted `match` or `if else` statements, making the "Happy Path" clearer to the reader.

```rust,editable
{{#include ../../crates/standard_library/examples/option/option_question_mark.rs:example}}
```

## Common `Option` Methods {#option-methods}

[![std][c~std~docs~badge]][c~std~docs]

The following table provides an overview of the most commonly used `Option` methods:

| Combinator | Description | Example Use Case |
| :--- | :--- | :--- |
| `is_some` | Returns `true` if the `Option` is `Some`, `false` otherwise. | Checking if a value exists without unwrapping it. |
| `is_none` | Returns `true` if the `Option` is `None`, `false` otherwise. | Checking if a value is absent. |
| `map` | Transforms the `Some` value into a new `Option` containing the result of applying a function `f`. If `self` is `None`, it remains `None`. | Converting a `Some(String)` to `Some(usize)` (e.g., length). |
| `and_then` | Chains operations that *themselves* return an `Option`. If `self` is `Some(v)`, applies `f` to `v` and returns the resulting `Option`. If `self` is `None`, returns `None`. Prevents `Option<Option<T>>` (nesting). | Chaining database lookups where each step might return `None` (e.g., `get_user_id.and_then(get_user_profile)`). |
| `or` | Returns `self` if it is `Some`. Otherwise, returns `other`. Useful for providing a fallback `Option`. | Providing a default configuration if the primary configuration is missing. |
| `or_else` | Similar to `or`, but the fallback `Option` is computed by a closure `f` *only if* `self` is `None`. Useful for expensive fallback computations. | Fetching data from a backup source only if the primary source is unavailable. |
| `unwrap_or` | Returns the contained `Some` value, or a provided `default` value if `self` is `None`. | Getting a user-provided name or defaulting to "Guest". |
| `unwrap_or_else` | Returns the contained `Some` value, or computes it from a closure `f` if `self` is `None`. Useful when the default value is expensive to compute. | Generating a new unique ID only if an existing one isn't found. |
| `ok_or` | Converts `Option<T>` into a `Result<T, E>`. If `self` is `Some(v)`, returns `Ok(v)`. If `self` is `None`, returns `Err(err)`. | Converting an `Option` of a parsed number to a `Result` indicating success or a parsing error. |
| `ok_or_else` | Similar to `ok_or`, but the error value `E` is computed by a closure `err_fn` *only if* `self` is `None`. | Converting `None` to a detailed error message that needs to be generated.  |
| `flatten` | Removes one level of nesting from an `Option` of an `Option` (i.e., `Option<Option<T>>` becomes `Option<T>`). | After a `map` operation that returns an `Option`, use `flatten` to simplify the result. |
| `filter` | Returns `Some(t)` if `self` is `Some(t)` AND the `predicate` function returns `true`. Otherwise, returns `None`. | Checking if a `Some(age)` is old enough to vote (`age.filter(|&a| a >= 18)`). |
| `zip` | If both `self` and `other` are `Some`, returns a `Some` containing a tuple of their values. Otherwise, returns `None`. | Combining `Option<FirstName>` and `Option<LastName>` into `Option<(String, String)>`. |

### Extract the Value Contained in an `Option` with `unwrap*` and `expect` {#extracting-the-value-contained-in-option}

[![std][c~std~docs~badge]][c~std~docs]

The following methods extract the contained value in an [`std::option::Option`][c~std::option::Option~docs]{{hi:std::option::Option}} when it is the `Some` variant. If the [`std::option::Option`][c~std::option::Option~docs]{{hi:std::option::Option}}↗ is `None`:

- [`std::option::Option::unwrap`][c~std::option::Option::unwrap~docs]{{hi:std::option::Option::unwrap}}↗ panics with a generic message.
- [`std::option::Option::expect`][c~std::option::Option::expect~docs]{{hi:std::option::Option::expect}}↗ panics with a provided custom message.
- [`std::option::Option::unwrap_or`][c~std::option::Option::unwrap_or~docs]{{hi:std::option::Option::unwrap_or}}↗ returns the provided default value.
- [`std::option::Option::unwrap_or_default`][c~std::option::Option::unwrap_or_default~docs]{{hi:std::option::Option::unwrap_or_default}}↗ returns the default value of the type T (which must implement the [`std::default::Default`][c~std::default::Default~docs]{{hi:std::default::Default}}↗ trait).
- [`std::option::Option::unwrap_or_else`][c~std::option::Option::unwrap_or_else~docs]{{hi:std::option::Option::unwrap_or_else}}↗ returns the result of evaluating the provided closure.

```rust,editable
{{#include ../../crates/standard_library/examples/option/option_unwrap.rs:example}}
```

### Transform Values Contained Within an `Option` {#combinators}

[![std][c~std~docs~badge]][c~std~docs]

The following example demonstrates the use of the `map`, `and_then` combinators and the `unwrap_or_else` method.
The `and_then` and `*or_else` methods take a closure as input, and only evaluate the closure when they need to produce a new value.

```rust,editable
{{#include ../../crates/standard_library/examples/option/option_combinators.rs:example}}
```

### Convert `&Option<T>` into `Option<&T>` or Similar {#adapters-for-working-with-references}

[![std][c~std~docs~badge]][c~std~docs]

When working with references to `Option`, use:

- `std::option::Option::as_ref` and `std::option::Option::as_mut` to convert from `&Option<T>` to `Option<&T>` and `Option<&mut T>`.
- [`std::option::Option::as_deref`][c~std::option::Option::as_deref~docs]{{hi:std::option::Option::as_deref}}↗ to convert from `&Option<T>` to `Option<&T::Target>`.
- [`std::option::Option::as_deref_mut`][c~std::option::Option::as_deref_mut~docs]{{hi:std::option::Option::as_deref_mut}}↗ to convert from `&mut Option<T>` to `Option<&mut T::Target>`.

See also the [[asref | AsRef]] chapter.

```rust,editable
{{#include ../../crates/standard_library/examples/option/option_ref.rs:example}}
```

## See Also {#skip}

- The [ShowOption](https://lib.rs/crates/show-option)↗ crate provides extension methods and macros for formatting Options.

## Related Topics {#related-topics}

- [[iterators | Iterators]].
- [[match | Match]].
- [[result | Result]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
