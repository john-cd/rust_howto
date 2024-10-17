# Custom Errors

{{#include error_customization.incl.md}}

Use [`anyhow`][c-anyhow]{{hi:anyhow}}⮳ if you don't care what error type your functions return, you just want it to be easy. This is common in application code. Use [`thiserror`][c-thiserror]{{hi:thiserror}}⮳ if you are a library that wants to design your own dedicated error type(s) so that on failures the caller gets exactly the information that you choose.

## Anyhow

[![anyhow][c-anyhow-badge]][c-anyhow]  [![anyhow-crates.io][c-anyhow-crates.io-badge]][c-anyhow-crates.io]  [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]

Use `Result<T, anyhow::Error>` or equivalently [`anyhow::Result{{hi:anyhow::Result}}<T>`][c-anyhow::Result]⮳ as the return type of any fallible function.

```rust,no_run,mdbook-runnable
{{#include ../../../../deps/tests/anyhow.rs}}
```

Anyhow works with any error type that has an impl of `std::error::Error`{{hi:std::error::Error}}, including ones defined in your crate e.g. using [`thiserror`][c-thiserror]{{hi:thiserror}}⮳.

## thisError

[![thiserror][c-thiserror-badge]][c-thiserror]

[`thiserror`][c-thiserror]{{hi:thiserror}}⮳ provides a convenient [`derive`][book-rust-reference-derive]{{hi:derive}}⮳ macro for the standard library’s `std::error::Error` trait.

```rust,no_run,mdbook-runnable
{{#include ../../../../deps/tests/thiserror.rs}}
```

The `#[error(...)]` messages support a shorthand for interpolating fields from the error.

```rust,ignore
#[error("{var}")]   //⟶ write!("{}", self.var)
#[error("{0}")]     //⟶ write!("{}", self.0)
#[error("{var:?}")] //⟶ write!("{:?}", self.var)
#[error("{0:?}")]   //⟶ write!("{:?}", self.0)
```

```rust
{{#include ../../../../deps/tests/thiserror2.rs}}
```

## Miette

[![miette][c-miette-badge]][c-miette]  [![miette-lib.rs][c-miette-lib.rs-badge]][c-miette-lib.rs] prints fancy diagnostics upon error.

```rust,ignore
{{#include ../../../../deps/tests/miette/mylib.rs}}
```

```rust,ignore
{{#include ../../../../deps/tests/miette/main.rs}}
```

## See also

[![eyre][c-eyre-badge]][c-eyre]

[![error_chain][c-error_chain-badge]][c-error_chain]

Do not use [Error Chain][c-error_chain]⮳, which is deprecated.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
