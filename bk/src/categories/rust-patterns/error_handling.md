# Error Handling

{{#include error_handling.incl.md}}

## Trigger and Handle Irrecoverable Panics {#irrecoverable-panics}

[![std][c-std-badge]][c-std] [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

The [`panic!(...)`][c-std::panic] macro allows a program to terminate immediately and provide feedback to the caller of the program.

```rust,editable,should_panic
{{#include ../../../crates/cats/rust_patterns/tests/error/panic.rs:example}}
```

`panic!` is closely tied with the `unwrap` method of both `Option` and `Result` [enums][p-enums]. Both implementations call `panic!` when they are set to `None` or `Err` variants.

```rust,editable,should_panic
{{#include ../../../crates/cats/rust_patterns/tests/error/unwrap.rs:example}}
```

## Provide a Fallback Value with `unwrap_or_else` {#unwrap-or-else}

[![std][c-std-badge]][c-std] [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/error/unwrap_or_else.rs:example}}
```

## Return Recoverable Errors with `Result` {#recoverable-errors-with-result}

[![std][c-std-badge]][c-std] [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

```rust,editable,should_panic
{{#include ../../../crates/cats/rust_patterns/tests/error/error_handling.rs:example}}
```

## Propagate Errors with the `?` Operator {#question-mark-operator}

[![std][c-std-badge]][c-std] [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/error/question_mark.rs:example}}
```

If the value of the Result is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue. If the value is an `Err`, the `Err` will be returned from the whole function, as if we had used the `return` keyword, so the error value gets propagated to the calling code.

This error points out that we're only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that implements [`std::ops::FromResidual`][c-std::ops::FromResidual]{{hi:std::ops::FromResidual}}⮳.

Another example:

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/error/question_mark2.rs:example}}
```

`std::io` defines the type alias `type Result<T> = std::result::Result<T, std::io::Error>;`

## Handle Errors Correctly in `main` {#handle-errors-correctly-in-main}

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}.

[`std::io::Error`][c-std::io::Error]{{hi:std::io::Error}}⮳ defined type implementing the [`std::error::Error`][c-std::error::Error]{{hi:std::error::Error}}⮳ trait.

The below recipe will tell how long the system has been running by opening the Unix file `/proc/uptime` and [parse][p-parse] the content to get the first number. It returns the uptime, unless there is an error.

```rust,editable
{{#include ../../../crates/language/tests/feat/main_test.rs:example}}
```

## Avoid Discarding Errors During Error Conversions {#avoid-discarding-errors-during-error-conversions}

[![reqwest][c-reqwest-badge]][c-reqwest] [![reqwest-crates.io][c-reqwest-crates.io-badge]][c-reqwest-crates.io] [![reqwest-github][c-reqwest-github-badge]][c-reqwest-github] [![reqwest-lib.rs][c-reqwest-lib.rs-badge]][c-reqwest-lib.rs]{{hi:reqwest}}{{hi:Client}}{{hi:Http}}{{hi:Request}} [![cat-wasm][cat-wasm-badge]][cat-wasm]{{hi:WebAssembly}} [![cat-web-programming::http-client][cat-web-programming::http-client-badge]][cat-web-programming::http-client]{{hi:HTTP client}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

Uses [`reqwest::blocking`][c-reqwest::blocking]⮳ to query a random integer generator web service. Converts the string response into an integer.

<div class="hidden">[move somewhere else](https://github.com/john-cd/rust_howto/issues/642)</div>

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/error/retain.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
## Obtain the Backtrace in Complex Error Scenarios {#obtain-backtrace}

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

This recipe shows how to handle a complex error scenario and then print a backtrace. It relies on to extend errors by appending new errors.

The below recipes attempts to deserialize the value `256` into a
[`u8`][primitive-u8]{{hi:u8}}⮳. An error will bubble up from Serde then [`csv`][c-csv]⮳{{hi:csv}} and finally up to the user code.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/error/backtrace.rs:example}}
```

Backtrace error rendered:

```text
Error level - description
└> 0 - Cannot read CSV data
└> 1 - Cannot deserialize RGB color
└> 2 - CSV deserialize error: record 1 (line: 2, byte: 15): field 1: number too large to fit in target type
└> 3 - field 1: number too large to fit in target type
```

Run the recipe with `RUST_BACKTRACE=1` to display a detailed backtrace associated with this error.

[error_handling: fix / organize](https://github.com/john-cd/rust_howto/issues/465)

[error_handling: need examples for](https://github.com/john-cd/rust_howto/issues/466)

- unwrap.

</div>
