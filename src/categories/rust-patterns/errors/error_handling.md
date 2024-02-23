# Error Handling

## Irrecoverable panics

```rust,editable,should_panic
fn main() {
    panic!("crash and burn");
}
```

## Recoverable errors with `Result`

```rust,editable,should_panic
{{#include ../../../../deps/tests/error_handling.rs}}
```

### unwrap_or_else

```rust,editable
{{#include ../../../../deps/tests/unwrap_or_else.rs}}
```

## A Shortcut for Propagating Errors: the ? Operator

```rust,editable
{{#include ../../../../deps/tests/question_mark.rs}}
```

If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

This error points out that we’re only allowed to use the `?` operator in a function that returns Result, Option, or another type that implements `FromResidual`.

Another example:

```rust,editable
{{#include ../../../../deps/tests/question_mark2.rs}}
```

`std::io` defines the type alias `type Result<T> = std::result::Result<T, std::io::Error>;`

{{#include ../../../refs/link-refs.md}}
