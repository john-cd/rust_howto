# Error Handling

{{#include error_handling.incl.md}}

## Irrecoverable panics

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

```rust,should_panic
{{#include ../../../../deps/tests/cats/rust_patterns/panic.rs:example}}
```

## Recoverable errors with `Result`

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

```rust,should_panic
{{#include ../../../../deps/tests/cats/rust_patterns/error_handling.rs:example}}
```

### unwrap_or_else

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}

```rust
{{#include ../../../../deps/tests/cats/rust_patterns/unwrap_or_else.rs:example}}
```

## A Shortcut for Propagating Errors: the ? Operator

[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

```rust
{{#include ../../../../deps/tests/cats/rust_patterns/question_mark.rs:example}}
```

If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

This error points out that we’re only allowed to use the `?` operator in a function that returns Result, Option, or another type that implements [`std::ops::FromResidual`][c-std::ops::FromResidual]{{hi:std::ops::FromResidual}}⮳.

Another example:

```rust
{{#include ../../../../deps/tests/cats/rust_patterns/question_mark2.rs:example}}
```

`std::io` defines the type alias `type Result<T> = std::result::Result<T, std::io::Error>;`

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
</div>
