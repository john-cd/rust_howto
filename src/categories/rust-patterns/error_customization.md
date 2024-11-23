# Custom Errors

{{#include error_customization.incl.md}}

Use [`anyhow`][c-anyhow]{{hi:anyhow}}⮳ if you don't care what error type your functions return, you just want it to be easy. This is common in application code. Use [`thiserror`][c-thiserror]{{hi:thiserror}}⮳ if you are a library that wants to design your own dedicated error type(s) so that on failures the caller gets exactly the information that you choose.

## Anyhow {#anyhow}

[![anyhow][c-anyhow-badge]][c-anyhow]{{hi:anyhow}}  [![anyhow-crates.io][c-anyhow-crates.io-badge]][c-anyhow-crates.io]  [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

Use `Result<T, anyhow::Error>` or equivalently [`anyhow::Result{{hi:anyhow::Result}}<T>`][c-anyhow::Result]⮳ as the return type of any fallible function.

```rust,editable
{{#include ../../../deps/tests/cats/rust_patterns/anyhow.rs:example}}
```

Anyhow works with any error type that has an impl of `std::error::Error`{{hi:std::error::Error}}, including ones defined in your crate e.g. using [`thiserror`][c-thiserror]{{hi:thiserror}}⮳.

## thisError {#thiserror}

[![thiserror][c-thiserror-badge]][c-thiserror]{{hi:thiserror}}

[`thiserror`][c-thiserror]{{hi:thiserror}}⮳ provides a convenient [`derive`][book-rust-reference-derive]{{hi:derive}}⮳ macro for the standard library’s `std::error::Error` trait.

```rust,editable
{{#include ../../../deps/tests/cats/rust_patterns/thiserror.rs:example}}
```

The `#[error(...)]` messages support a shorthand for interpolating fields from the error.

```rust,editable,compile_fail
#[error("{var}")]   //⟶ write!("{}", self.var)
#[error("{0}")]     //⟶ write!("{}", self.0)
#[error("{var:?}")] //⟶ write!("{:?}", self.var)
#[error("{0:?}")]   //⟶ write!("{:?}", self.0)
```

```rust,editable
{{#include ../../../deps/tests/cats/rust_patterns/thiserror2.rs:example}}
```

## `miette` {#miette}

[![miette][c-miette-badge]][c-miette]{{hi:miette}}
[![miette-lib.rs][c-miette-lib.rs-badge]][c-miette-lib.rs] prints fancy diagnostics upon error.

```rust,editable
{{#include ../../../deps/tests/cats/rust_patterns/miette.rs:example}}
```

## `color-eyre` {#color-eyre}

[![color-eyre][c-color_eyre-badge]][c-color_eyre]{{hi:color-eyre}}
[![color-eyre-crates.io][c-color_eyre-crates.io-badge]][c-color_eyre-crates.io]
[![color-eyre-github][c-color_eyre-github-badge]][c-color_eyre-github]
[![color-eyre-lib.rs][c-color_eyre-lib.rs-badge]][c-color_eyre-lib.rs]

A fork of `anyhow` that gives you more control over the format of the generated error messages. Recommended if you intend to present error messages to end users. Otherwise `anyhow` is simpler.

## See also

[![eyre][c-eyre-badge]][c-eyre]{{hi:eyre}}

[![error_chain][c-error_chain-badge]][c-error_chain]{{hi:error_chain}}

Do not use [Error Chain][c-error_chain]{{hi:error_chain}}⮳, which is deprecated.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
