# Error Handling

{{#include error_handling.incl.md}}

## Irrecoverable panics {#irrecoverable-panics}

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

```rust,should_panic
{{#include ../../../deps/tests/cats/rust_patterns/panic.rs:example}}
```

## Recoverable errors with `Result` {#recoverable-errors-with-result}

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

```rust,should_panic
{{#include ../../../deps/tests/cats/rust_patterns/error_handling.rs:example}}
```

### `unwrap_or_else` {#unwrap-or-else}

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}

```rust
{{#include ../../../deps/tests/cats/rust_patterns/unwrap_or_else.rs:example}}
```

## A Shortcut for propagating errors: the ? Operator {#question-mark-operator}

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

```rust
{{#include ../../../deps/tests/cats/rust_patterns/question_mark.rs:example}}
```

If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

This error points out that we’re only allowed to use the `?` operator in a function that returns Result, Option, or another type that implements [`std::ops::FromResidual`][c-std::ops::FromResidual]{{hi:std::ops::FromResidual}}⮳.

Another example:

```rust
{{#include ../../../deps/tests/cats/rust_patterns/question_mark2.rs:example}}
```

`std::io` defines the type alias `type Result<T> = std::result::Result<T, std::io::Error>;`

## Handle errors correctly in main {#handle-errors-correctly-in-main}

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

Handles error that occur when trying to open a file that does not exist. It is achieved by using [`error_chain`][c-error_chain]{{hi:error_chain}}⮳, a library that takes care of a lot of boilerplate code needed in order to [handle errors in Rust][book-rust-handle-errors-in-Rust]⮳.

`Io(std::io::Error)` inside [`foreign_links`][c-error_chain-foreign_links]⮳ allows automatic conversion from [`std::io::Error`][c-std::io::Error]{{hi:std::io::Error}}⮳ into [`error_chain::error_chain`][c-error_chain::error_chain]{{hi:error_chain::error_chain}}⮳ defined type implementing the [`std::error::Error`][c-std::error::Error]{{hi:std::error::Error}}⮳ trait.

The below recipe will tell how long the system has been running by opening the Unix file `/proc/uptime` and parse the content to get the first number. Returns uptime unless there is an error.

Other recipes in this book will hide the [`error_chain`][c-error_chain]{{hi:error_chain}}⮳ boilerplate, and can be seen by expanding the code with the ⤢ button.

```rust
{{#include ../../../deps/tests/lang/main_test.rs:example}}
```

## Avoid discarding errors during error conversions {#avoid-discarding-errors-during-error-conversions}

[![error_chain][c-error_chain-badge]][c-error_chain]{{hi:error_chain}}  [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

The [`error_chain`][c-error_chain]{{hi:error_chain}}⮳ crate makes [`matching`][c-error_chain-matching-error]{{hi:matching}}⮳ on different error types returned by a function possible and relatively compact. [`error_chain::example_generated::ErrorKind`][c-error_chain::example_generated::ErrorKind]{{hi:error_chain::example_generated::ErrorKind}}⮳ determines the error type.

Uses [`reqwest`][c-reqwest]{{hi:reqwest}}⮳::[blocking][c-reqwest::blocking]⮳ to query a random integer generator web service. Converts the string response into an integer. The Rust standard library,
[`reqwest`][c-reqwest]{{hi:reqwest}}⮳, and the web service can all generate errors. Well defined Rust errors use [`foreign_links`][c-error_chain-foreign_links]⮳ An additional [`error_chain::example_generated::ErrorKind`][c-error_chain::example_generated::ErrorKind]{{hi:error_chain::example_generated::ErrorKind}}⮳ variant for the web service error uses `errors` block of the `error_chain!` macro.

```rust
{{#include ../../../deps/tests/cats/rust_patterns/retain.rs:example}}
```

## Obtain backtrace of complex error scenarios {#obtain-backtrace}

[![error_chain][c-error_chain-badge]][c-error_chain]{{hi:error_chain}}  [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

This recipe shows how to handle a complex error scenario and then print a backtrace. It relies on [`chain_err`][c-error_chain-chaining-errors]⮳ to extend errors by appending new errors. The error stack can be unwound, thus providing a better context to understand why an error was raised.

The below recipes attempts to deserialize the value `256` into a
[`u8`][primitive-u8]{{hi:u8}}⮳. An error will bubble up from Serde then csv and finally up to the user code.

```rust
{{#include ../../../deps/tests/cats/rust_patterns/backtrace.rs:example}}
```

Backtrace error rendered:

```text
Error level - description
└> 0 - Cannot read CSV data
└> 1 - Cannot deserialize RGB color
└> 2 - CSV deserialize error: record 1 (line: 2, byte: 15): field 1: number too large to fit in target type
└> 3 - field 1: number too large to fit in target type
```

Run the recipe with `RUST_BACKTRACE=1` to display a detailed [`error_chain::ChainedError::backtrace`][c-error_chain::ChainedError::backtrace]{{hi:error_chain::ChainedError::backtrace}}⮳ associated with this error.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO fix!!
</div>
