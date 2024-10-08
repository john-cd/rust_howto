# Error Handling

{{#include handle.incl.md}}

## Handle errors correctly in main

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]

Handles error that occur when trying to open a file that does not exist. It is achieved by using [`error-chain`][error-chain]⮳, a library that takes care of a lot of boilerplate code needed in order to [handle errors in Rust][book-rust-handle-errors-in-Rust]⮳.

`Io(std::io::Error)` inside [`foreign_links`][error-chain-foreign_links]⮳ allows automatic conversion from [`std::io::Error`][std::io::Error]⮳ into [`error_chain!`][error_chain::error_chain]⮳ defined type implementing the [`Error`][std::error::Error]⮳ trait.

The below recipe will tell how long the system has been running by opening the Unix file `/proc/uptime` and parse the content to get the first number. Returns uptime unless there is an error.

Other recipes in this book will hide the [`error-chain`][error-chain]⮳ boilerplate, and can be seen by expanding the code with the ⤢ button.

```rust,editable
{{#include ../../../../deps/tests/main.rs}}
```

## Avoid discarding errors during error conversions

[![error-chain][error-chain-badge]][error-chain]  [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]

The [`error-chain`][error-chain]⮳ crate makes [`matching`][error-chain-matching-error]⮳ on different error types returned by a function possible and relatively compact. [`ErrorKind`][error_chain::example_generated::ErrorKind]⮳ determines the error type.

Uses [`reqwest`][reqwest]⮳::[blocking][reqwest::blocking]⮳ to query a random integer generator web service. Converts the string response into an integer. The Rust standard library,
[`reqwest`][reqwest]⮳, and the web service can all generate errors. Well defined Rust errors use [`foreign_links`][error-chain-foreign_links]⮳ An additional [`ErrorKind`][error_chain::example_generated::ErrorKind]⮳ variant for the web service error uses `errors` block of the `error_chain!` macro.

```rust,editable
{{#include ../../../../deps/tests/retain.rs}}
```

## Obtain backtrace of complex error scenarios

[![error-chain][error-chain-badge]][error-chain]  [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]

This recipe shows how to handle a complex error scenario and then print a backtrace. It relies on [`chain_err`][error-chain-chaining-errors]⮳ to extend errors by appending new errors. The error stack can be unwound, thus providing a better context to understand why an error was raised.

The below recipes attempts to deserialize the value `256` into a
[`u8`][u8]⮳. An error will bubble up from Serde then csv and finally up to the user code.

```rust,editable
{{#include ../../../../deps/tests/backtrace.rs}}
```

Backtrace error rendered:

```text
Error level - description
└> 0 - Cannot read CSV data
└> 1 - Cannot deserialize RGB color
└> 2 - CSV deserialize error: record 1 (line: 2, byte: 15): field 1: number too large to fit in target type
└> 3 - field 1: number too large to fit in target type
```

Run the recipe with `RUST_BACKTRACE=1` to display a detailed [`backtrace`][error_chain::ChainedError::backtrace]⮳ associated with this error.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
