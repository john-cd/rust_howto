# Custom Errors

{{#include error_customization.incl.md}}

Use [`Anyhow`][anyhow]⮳ if you don't care what error type your functions return, you just want it to be easy. This is common in application code. Use [`thiserror`][thiserror]⮳ if you are a library that wants to design your own dedicated error type(s) so that on failures the caller gets exactly the information that you choose.

## Anyhow

[![anyhow][anyhow-badge]][anyhow]  [(crates.io)][anyhow-crate]  [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]

Use `Result<T, anyhow::Error>` or equivalently [`anyhow::Result<T>`][anyhow::Result]⮳ as the return type of any fallible function.

```rust,editable,no_run,mdbook-runnable
{{#include ../../../../deps/tests/anyhow.rs}}
```

Anyhow works with any error type that has an impl of `std::error::Error`, including ones defined in your crate e.g. using [`thiserror`][thiserror]⮳.

## thisError

[![thiserror][thiserror-badge]][thiserror]

[`thisError`][thisError]⮳ provides a convenient [`derive`][book-rust-reference-derive]⮳ macro for the standard library’s `std::error::Error` trait.

```rust,editable,no_run,mdbook-runnable
{{#include ../../../../deps/tests/thiserror.rs}}
```

The `#[error(...)]` messages support a shorthand for interpolating fields from the error.

```rust,editable,ignore
#[error("{var}")] ⟶ write!("{}", self.var)
#[error("{0}")] ⟶ write!("{}", self.0)
#[error("{var:?}")] ⟶ write!("{:?}", self.var)
#[error("{0:?}")] ⟶ write!("{:?}", self.0)
```

```rust,editable,mdbook-runnable
{{#include ../../../../deps/tests/thiserror2.rs}}
```

## Miette

[![miette][miette-badge]][miette]  [(lib.rs)][miette-librs] prints fancy diagnostics upon error.

```rust,editable,ignore
{{#include ../../../../deps/tests/miette/mylib.rs}}
```

```rust,editable,ignore
{{#include ../../../../deps/tests/miette/main.rs}}
```

## See also

[![eyre][eyre-badge]][eyre]

[![error-chain][error-chain-badge]][error-chain]

Do not use [Error Chain][error-chain]⮳, which is deprecated.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
