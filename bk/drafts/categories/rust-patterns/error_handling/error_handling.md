# Error Handling

{{#include error_handling.incl.md}}

## Trigger Irrecoverable Panics {#irrecoverable-panics}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

The [`panic!(...)`][c~std::panic~docs] macro allows a program to terminate immediately and provide feedback to the caller of the program.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/panic.rs:example}}
```

## Generate and Handle Recoverable Errors with `Result` {#result}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} is an enum used to represent the outcome of operations that might fail. It is a flexible way to handle errors in a type-safe manner. The enum has two variants: `Ok`{{hi:Ok}} and `Err`{{hi:Err}}.

```rust,editable
enum Result<T, E> {
    Ok(T),
    Err(E),
}
// T and E are generic.
```

Simply return one of the two variants as needed. Note that [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} is in the prelude.

```rust,editable
fn divide_numbers(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("Division by zero")
    } else {
        Ok(x / y)
    }
}
```

You can handle the [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} via a simple [`match`][keyword~match]↗{{hi:match}} or [`if let`](https://doc.rust-lang.org/book/ch19-01-all-the-places-for-patterns.html#conditional-if-let-expressions)↗{{hi:if let}} expression:

```rust,editable
fn main() {
    let _result1: Result<i32, &str> = Ok(10);

    let result2: Result<i32, &str> = Err("Something went wrong.");

    // Here, we just print the status.
    match result {
        Ok(value) => println!("Success: {value}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}
```

## Convert a `Result` or `Option` into an Irrecoverable Panic {#convert-result-or-option-into-panic}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`panic!`][c~std::panic::panic~docs]↗{{hi:panic!}} is closely tied with the `unwrap`{{hi:unwrap}} method of both [`Option`][c~std::option::Option~docs]↗{{hi:std::option::Option}} and [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} [enums][p~enums]. Both implementations call `panic!` when they are set to the `None` or `Err` variants. `expect` is frequently used instead of `unwrap`.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/unwrap.rs:example}}
```

## Provide a Fallback Value with `unwrap_or` or `unwrap_or_else` {#unwrap-or-else}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/unwrap_or_else.rs:example}}
```

## Chain operations with `and_then` {#and_then}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`and_then`][c~std::result::Result::and_then~docs]↗{{hi:and_then}} is used to chain the operations. It runs the next function only if the previous `Result` is `Ok`. If any step produces an `Err`, the chain stops, and the `Err` is returned.

```rust,editable
fn divide_by_two(x: i32) -> Result<i32, &'static str> {
    if x % 2 == 0 {
        Ok(x / 2)
    } else {
        Err("Not divisible by 2")
    }
}

fn divide_by_three(x: i32) -> Result<i32, &'static str> {
    if x % 3 == 0 {
        Ok(x / 3)
    } else {
        Err("Not divisible by 3")
    }
}

fn main() {
    let result = divide_by_two(12)
        .and_then(divide_by_three); // Chain the second operation

    match result {
        Ok(value) => println!("Success: {value}"),
        Err(error) => println!("Error: {error}"),
    }
}
```

## Transform `Result` Values {#map}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

The [`map`][c~std::result::Result::map~docs]↗{{hi:map}} and [`map_err`][c~std::result::Result::map_err~docs]↗{{hi:map_err}} methods let you transform the contents of `Ok` and `Err` respectively.

```rust,editable
fn main() {
    let result: Result<i32, &str> = Ok(10);

    let doubled = result.map(|value| value * 2); // Applies a function to the `Ok` value
    println!("Doubled: {doubled:?}");

    let error_mapped = result.map_err(|err| format!("Error: {}", err)); // Maps the `Err` value
    println!("Mapped error: {error_mapped:?}");
}

#[test]
fn test() {
  main();
}
```

## Propagate Recoverable Errors with the `?` Operator {#question-mark-operator}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

Use the [`?`][book~rust~ch09-02-recoverable-errors-with-result-?]↗{{hi:?}} operator to propagate errors from a function call 'up the stack'.

If the value of the [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} is an `Ok`, the value inside the `Ok` will get returned, and the program will continue. If the value is an `Err`, the `Err` will be returned from the whole function, as if we had used the `return` keyword, so the error value gets propagated to the calling code.

Note that we're only allowed to use the `?`{{hi:?}} operator in a function that returns [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}}, [`Option`][c~std::option::Option~docs]↗{{hi:std::option::Option}}, or another type that implements [`std::ops::FromResidual`][c~std::ops::FromResidual~docs]{{hi:std::ops::FromResidual}}↗.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/question_mark.rs:example}}
```

Note that we could have used the common type alias `type Result<T> = std::result::Result<T, std::io::Error>;` as the return type of `read_username_from_file`.

The following example highlights the need to return `Result<..., Box<dyn Error>>` (or a similar type) when multiple `?`{{hi:?}} operators are used in a given method and their error types are not the same:

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/question_mark2.rs:example}}
```

## Handle Errors in `main` {#handle-errors-in-main}

[![std][c~std~docs~badge]][c~std~docs] [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

To handle a [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} in the `main` function, you may:

- use a [`match`][keyword~match]↗{{hi:match}}, `if let`{{hi:if let}}, or `while let`{{hi:while let}} expression
  - to display or log the error, as described above,
  - to attempt to recover from the error (for example by retrying the last operation).
- ignore the [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} by assigning it to `let _ = ...` (rarely the right solution),
- return a [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}} from the `main` function.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/error_handling.rs:example}}
```

## Avoid Discarding Errors During Error Conversions {#avoid-discarding-errors-during-error-conversions}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs] [![reqwest~crates.io][c~reqwest~crates.io~badge]][c~reqwest~crates.io] [![reqwest~github][c~reqwest~github~badge]][c~reqwest~github] [![reqwest~lib.rs][c~reqwest~lib.rs~badge]][c~reqwest~lib.rs]{{hi:reqwest}}{{hi:Client}}{{hi:Http}}{{hi:Request}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:HTTP client}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

Uses [`reqwest::blocking`][c~reqwest::blocking~docs]↗ to query a random integer generator web service. Converts the string response into an integer.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/retain.rs:example}}
```

## Obtain the Backtrace in Complex Error Scenarios {#obtain-backtrace}

[![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

This recipe shows how to handle a complex error scenario and then print a backtrace.

The example attempts to deserialize the value `256` into a [`u8`][primitive~u8]{{hi:u8}}↗. An error will bubble up from [`serde`][c~serde~docs]↗{{hi:serde}} to [`csv`][c~csv~docs]↗{{hi:csv}} and finally up to the user code.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/backtrace.rs:example}}
```

Backtrace error rendered:

```text
Error level - description
└> 0 - Cannot read CSV data
└> 1 - Cannot deserialize RGB color
└> 2 - CSV deserialize error: record 1 (line: 2, byte: 15): field 1: number too large to fit in target type
└> 3 - field 1: number too large to fit in target type
```

Run the recipe with [`RUST_BACKTRACE=1`]( ){{hi: }} to display a detailed backtrace associated with this error.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[error_handling: fix / organize NOW](https://github.com/john-cd/rust_howto/issues/465)
review https://doc.rust-lang.org/rust-by-example/error.html
FIXME rename examples; move example above to separate file
credit https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

[`std::io::Error`][c~std::io::Error~docs]{{hi:std::io::Error}}↗ defined type implementing the [`std::error::Error`][c~std::error::Error~docs]{{hi:std::error::Error}}↗ trait.

- [human-panic: Panic messages for humans.][c~human-panic~github]↗.

</div>
