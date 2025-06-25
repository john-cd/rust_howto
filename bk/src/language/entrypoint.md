# Entry Points

{{#include entrypoint.incl.md}}{{hi:main}}

## Main Function {#main-function}

In Rust, the main function is special. It serves as the mandatory entry point for every executable program.
The following is the most basic form of a main function:

```rust,editable
{{#include ../../crates/language/examples/entrypoint/main_fn.rs:example}}
```

It's typically found in the `src/main.rs` file of a binary crate project. Note that library crates don't need one.

You can also have the `main` function return a `Result<(), E>` (where `E` is a type that implements the `std::error::Error` trait).

```rust,editable
{{#include ../../crates/language/examples/entrypoint/main_fn_with_result.rs:example}}
```

It is also common to return a `Result` with a boxed error trait object, especially if multiple error types are possible:

```rust,noplayground
use std::error::Error;
// ...

fn main() -> Result<(), Box<dyn Error>> {
   // ...
}
```

More precisely, `main` should return a type that implements `std::process::Termination`. For example, you may return `std::process::ExitCode` to provide a status code the current process can return to its parent.

## Async Main Function {#async-main-function}

Rust also supports asynchronous main functions for writing concurrent applications using the `async` / `await` syntax. To use `async main`, you will need an async runtime like `tokio`, often enabled via an attribute macro.

```rust,editable
{{#include ../../crates/language/examples/entrypoint/async_main.rs:example}}
```

## Related Topics {#skip}

- [[asynchronous | Asynchronous]].
- [[functions | Functions]].
- [[package_layout | Package Layout]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
