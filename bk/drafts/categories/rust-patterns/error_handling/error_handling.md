<!-- TODO Reviewed ??-??-2026 -->
# Error Handling

{{#include error_handling.incl.md}}

Operations that can fail should return a value that describes the failure. Use [`Option`][c~std::option::Option~docs]↗{{hi:Option}} when the only question is whether a value exists, and [`Result`][c~std::result::Result~docs]↗{{hi:Result}} when callers need to know why an operation failed. Handle an error, propagate it to a caller that can decide what to do, or deliberately recover with a documented fallback.

Reserve panics for broken invariants and other situations where continuing would be incorrect. For application and library design choices, see [[error_customization | Custom Errors]].

## Trigger Irrecoverable Panics {#irrecoverable-panics}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

The [`panic!(...)`][c~std::panic~docs]↗ macro stops the current thread. A panic is appropriate for a violated invariant or an unrecoverable programming error. For invalid input, failed I/O, and other expected failures, return a `Result` so the caller can handle the problem.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/panic.rs:example}}
```

## Represent Recoverable Failures with `Result` {#result}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`Result<T, E>`][c~std::result::Result~docs]↗{{hi:std::result::Result}} represents either a successful value of type `T` or an error of type `E`. Its variants are `Ok(T)` and `Err(E)`. It is in the prelude, so you can write `Result` without importing it.

```rust,editable
enum Result<T, E> {
    Ok(T),
    Err(E),
}
// T and E are generic.
```

Return `Ok(value)` on success and `Err(error)` on failure. A caller can handle both cases with [`match`][keyword~match]↗{{hi:match}} or handle one case with [`if let`][book~rust~if-let]↗{{hi:if let}}. The example uses `match` to report both successful and failed divisions.

```rust,editable
fn main() {
{{#include ../../../../crates/cats/rust_patterns/examples/error/result.rs:example}}
```

## Convert a `Result` or `Option` into an Irrecoverable Panic {#convert-result-or-option-into-panic}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

Use `unwrap` or `expect` when failure proves a bug or violates an invariant that the program must uphold. Avoid them for ordinary user input, network, and file errors: propagate or handle those errors instead.

[`panic!`][c~std::panic::panic~docs]↗{{hi:panic!}} is closely tied with the `unwrap`{{hi:unwrap}} method of both [`Option`][c~std::option::Option~docs]↗{{hi:std::option::Option}} and [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} [enums][p~enums]. Both implementations call `panic!` when they are set to the `None` or `Err` variants. `expect` is frequently used instead of `unwrap`.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/unwrap.rs:example}}
```

## Provide a Fallback Value with `unwrap_or` or `unwrap_or_else` {#unwrap-or-else}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/unwrap_or_else.rs:example}}
```

`unwrap_or` evaluates its fallback argument before the call. Use `unwrap_or_else` when computing the fallback is expensive or should happen only when the value is absent or erroneous.

## Chain operations with `and_then` {#and_then}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`and_then`][c~std::result::Result::and_then~docs]↗{{hi:and_then}} is used to chain the operations. It runs the next function only if the previous `Result` is `Ok`. If any step produces an `Err`, the chain stops, and the `Err` is returned.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/and_then.rs:example}}
```

## Transform `Result` Values {#map}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

The [`map`][c~std::result::Result::map~docs]↗{{hi:map}} method transforms a successful value while leaving an error unchanged. [`map_err`][c~std::result::Result::map_err~docs]↗{{hi:map_err}} transforms the error while leaving a successful value unchanged.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/map.rs:example}}
```

## Propagate Recoverable Errors with the `?` Operator {#question-mark-operator}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

Use the [`?`][book~rust~ch09-02-recoverable-errors-with-result-?]↗{{hi:?}} operator to propagate a failure to the caller. It keeps a successful value available to the rest of the function and returns early on failure.

With `Result`, `?` returns an `Err` from the current function, converting it to the function's error type through [`From`][c~std::convert::From~docs]↗{{hi:From}} when that conversion is available. With `Option`, it returns `None`. This keeps error handling at the layer that has enough context to decide how to respond.

The operator is available in functions whose return type supports the corresponding residual conversion, commonly `Result` and `Option`.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/question_mark.rs:example}}
```

The standard library also provides the type alias `std::io::Result<T>` for `Result<T, std::io::Error>`, as used by many I/O APIs.

The following example uses `Box<dyn Error>` so errors from different operations can be propagated from one function. A library often benefits from a dedicated error enum; an application can use a flexible report type such as [`anyhow::Error`][c~anyhow::Error~docs]↗{{hi:anyhow}}. See [[error_customization | Custom Errors]] for the tradeoffs.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/question_mark2.rs:example}}
```

## Handle Errors in `main` {#handle-errors-in-main}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

At the program boundary, either handle a [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} explicitly or return it from `main`:

- use a [`match`][keyword~match]↗{{hi:match}}, `if let`{{hi:if let}}, or `while let`{{hi:while let}} expression
  - to display or log the error, as described above,
  - to attempt to recover from the error (for example by retrying the last operation).
- return a [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} from `main`. The runtime reports an error returned this way and exits with a failure status.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/error_handling.rs:example}}
```

## Avoid Discarding Errors During Error Conversions {#avoid-discarding-errors-during-error-conversions}

[![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

When a function returns [`anyhow::Result`][c~anyhow::Result~docs]↗, `?` converts errors such as [`std::io::Error`][p~std::io::Error]↗ and [`std::num::ParseIntError`][p~std::num::ParseIntError]↗ into `anyhow::Error` while retaining the source chain. For a library's public API, prefer an error type that preserves the distinctions callers need; see [[error_customization | Custom Errors]].

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/retain.rs:example}}
```

## Obtain the Backtrace in Complex Error Scenarios {#obtain-backtrace}

[![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

This recipe shows how a low-level deserialization failure can be wrapped with context as it travels through several layers. An error chain explains the sequence of failures; a captured backtrace can add the code locations where the error was created.

The example attempts to deserialize the value `256` into a [`u8`][primitive~u8]↗{{hi:u8}}. An error will bubble up from [`serde`][c~serde~docs]↗{{hi:serde}} to [`csv`][c~csv~docs]↗{{hi:csv}} and finally up to the user code.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/backtrace.rs:example}}
```

The example prints the error with `Debug` formatting. Its context chain identifies the layer where each failure was reported.

```text
Error: Cannot read CSV data

Caused by:
   0: Cannot deserialize RGB color
   1: CSV deserialize error: record 1 (line: 2, byte: 15): field 1: number too large to fit in target type
   2: field 1: number too large to fit in target type
```

For `anyhow`, setting `RUST_BACKTRACE=1` before running the example enables backtrace capture. An error backtrace is distinct from the panic backtrace controlled by the same variable.

## Related Topics {#related-topics .skip}

For custom error types, diagnostic reports, and application-level context, see [[error_customization | Custom Errors]]. For an introduction to recoverable failures with `Result`, see the [Rust Book chapter on error handling][book~rust~ch09-02-recoverable-errors-with-result].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[error_handling: fix / organize](https://github.com/john-cd/rust_howto/issues/465)
review [![Rust by example - error][book~rust-by-example~error~badge]][book~rust-by-example~error]

[`std::io::Error`][c~std::io::Error~docs]↗{{hi:std::io::Error}} defined type implementing the [`std::error::Error`][c~std::error::Error~docs]↗{{hi:std::error::Error}} trait.

- [`human-panic`][c~human-panic~repo]↗: Panic messages for humans.

[content audit: content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1904)
</div>
