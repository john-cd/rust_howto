# Main Function

{{#include main.incl.md}}{{hi:main}}

In Rust, the main function is special. It serves as the mandatory entry point for every executable program.
The following is the most basic form of a main function:

```rust,editable
{{#include ../../crates/language/tests/feat/main_fn.rs:example}}
```

It's typically found in the `src/main.rs` file of a binary crate project. Note that library crates don't need one.

You can also have the `main` function return a `Result<(), E>` (where `E `is atype that implements the `std::error::Error` trait).

```rust,editable
{{#include ../../crates/language/tests/feat/main_fn_with_result.rs:example}}
```

## Async Main Function {#async-main-function}

Rust also supports asynchronous main functions for writing concurrent applications using the `async`/`await` syntax. To use `async main`, you typically need an async runtime like `tokio`, often enabled via an attribute macro.

```rust,editable
{{#include ../../crates/language/tests/feat/async_main.rs:example}}
```

## Related Topics {#skip}

- [[asynchronous | Asynchronous]].
- [[functions | Functions]].
- [[package_layout | Package Layout]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
