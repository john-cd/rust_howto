# Custom Errors

{{#include error_customization.incl.md}}

Use [`anyhow`][c-anyhow]{{hi:anyhow}}⮳ if you don't care what error type your functions return, you just want it to be easy. This is common in application code. Use [`thiserror`][c-thiserror]{{hi:thiserror}}⮳ if you are a library that wants to design your own dedicated error type(s) so that on failures the caller gets exactly the information that you choose.

## `anyhow` {#anyhow}

[![anyhow][c-anyhow-badge]][c-anyhow] [![anyhow-crates.io][c-anyhow-crates.io-badge]][c-anyhow-crates.io] [![anyhow-github][c-anyhow-github-badge]][c-anyhow-github] [![anyhow-lib.rs][c-anyhow-lib.rs-badge]][c-anyhow-lib.rs]{{hi:anyhow}}{{hi:Error}}{{hi:Error-handling}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

`anyhow` provides a flexible concrete Error type built on `std::error::Error`.

Use `Result<T, anyhow::Error>` or equivalently [`anyhow::Result{{hi:anyhow::Result}}<T>`][c-anyhow::Result]⮳ as the return type of any fallible function.

```rust,editable
{{#include ../../../crates/ex/cats/rust_patterns/tests/error/anyhow.rs:example}}
```

Anyhow works with any error type that has an impl of `std::error::Error`{{hi:std::error::Error}}, including ones defined in your crate e.g. using [`thiserror`][c-thiserror]{{hi:thiserror}}⮳.

## `thisError` {#thiserror}

[![thiserror][c-thiserror-badge]][c-thiserror] [![thiserror-crates.io][c-thiserror-crates.io-badge]][c-thiserror-crates.io] [![thiserror-github][c-thiserror-github-badge]][c-thiserror-github] [![thiserror-lib.rs][c-thiserror-lib.rs-badge]][c-thiserror-lib.rs]{{hi:thiserror}}{{hi:Derive}}{{hi:Error}}{{hi:Error-handling}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

[`thiserror`][c-thiserror]{{hi:thiserror}}⮳ provides a convenient [`derive`][book-rust-reference-derive]{{hi:derive}}⮳ macro for the standard library’s `std::error::Error` trait.

```rust,editable
{{#include ../../../crates/ex/cats/rust_patterns/tests/error/thiserror.rs:example}}
```

The `#[error(...)]` messages support a shorthand for interpolating fields from the error.

```rust,editable,compile_fail,noplayground
#[error("{var}")]  //⟶ write!("{}", self.var)
#[error("{0}")]   //⟶ write!("{}", self.0)
#[error("{var:?}")] //⟶ write!("{:?}", self.var)
#[error("{0:?}")]  //⟶ write!("{:?}", self.0)
```

```rust,editable
{{#include ../../../crates/ex/cats/rust_patterns/tests/error/thiserror2.rs:example}}
```

## `miette` {#miette}

[![miette][c-miette-badge]][c-miette] [![miette-crates.io][c-miette-crates.io-badge]][c-miette-crates.io] [![miette-github][c-miette-github-badge]][c-miette-github] [![miette-lib.rs][c-miette-lib.rs-badge]][c-miette-lib.rs]{{hi:miette}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

`miette` is a fancy diagnostic reporting library and protocol.

```rust,editable
{{#include ../../../crates/ex/cats/rust_patterns/tests/error/miette.rs:example}}
```

## `color-eyre` {#color-eyre}

[![color-eyre][c-color_eyre-badge]][c-color_eyre] [![color-eyre-crates.io][c-color_eyre-crates.io-badge]][c-color_eyre-crates.io] [![color-eyre-github][c-color_eyre-github-badge]][c-color_eyre-github] [![color-eyre-lib.rs][c-color_eyre-lib.rs-badge]][c-color_eyre-lib.rs]{{hi:color-eyre}}

`color-eyre` is an error report handler for panics and `eyre::Reports` for colorful, consistent, and well formatted error reports for all kinds of errors.

It is a fork of `anyhow` that gives you more control over the format of the generated error messages. It is recommended if you intend to present error messages to end users. Otherwise `anyhow` is simpler.

```rust,editable
{{#include ../../../crates/ex/cats/rust_patterns/tests/error/color_eyre.rs:example}}
```

## See also

[![eyre][c-eyre-badge]][c-eyre]{{hi:eyre}}

Do not use [Error Chain][c-error_chain]{{hi:error_chain}}⮳, which is deprecated.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[error_customization: write / organize / titles (P1)](https://github.com/john-cd/rust_howto/issues/463)

</div>
