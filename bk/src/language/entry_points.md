<!-- TODO Reviewed ??-??-2026. -->

# Entry Points

{{#include entry_points.incl.md}}

{{hi:main}}

[![Rust by Example - hello][book~rust-by-example~hello~badge]][book~rust-by-example~hello]

## Define the Entry Point of your Application with the Main Function {#main-function}

In Rust, the main function is special. It serves as the mandatory entry point for every executable program.
The following is the most basic form of a main function:

```rust,editable
{{#include ../../crates/language/examples/entrypoint/main_fn.rs:example}}
```

It is typically found in the `src/main.rs` file of a binary crate project. Note that library crates don't need one.

You can also have `main` return `Result<(), E>`. Because `Result` implements [`Termination`][c~std::process::Termination~docs]↗{{hi:Termination}} when its error type `E` implements `Debug`, `E` does not have to implement `std::error::Error`.

```rust,editable
{{#include ../../crates/language/examples/entrypoint/main_fn_with_result.rs:example}}
```

It is also common to return a `Result` with a boxed error trait object, especially if multiple error types are possible:

```rust,noplayground
{{#include ../../crates/language/examples/entrypoint/boxed_main_error.rs:example}}
```

More precisely, `main` should return a type that implements `std::process::Termination`. For example, you may return `std::process::ExitCode` to provide a status code the current process can return to its parent.

## Define the Entry Point of your Asynchronous Application {#async-main-function}

Rust also supports asynchronous main functions for writing concurrent applications using the `async` / `await` syntax. To use `async main`, you will need an async runtime like `tokio`, often enabled via an attribute macro.

```rust,editable
{{#include ../../crates/language/examples/entrypoint/async_main.rs:example}}
```

For an introduction to futures and executors, see [Async Rust (draft)](../categories/asynchronous/async.md).

## Related Topics {#related-topics .skip}

- [[functions | Functions]].
- [Package Layout](https://github.com/john-cd/rust_howto/blob/develop/bk/drafts/categories/development-tools/cargo/package_layout.md).

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
