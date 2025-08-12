# Custom Errors

{{#include error_customization.incl.md}}

Use [`anyhow`][c~anyhow~docs]{{hi:anyhow}}↗ if you don't care what error type your functions return, you just want it to be easy. This is common in application code. Use [`thiserror`][c~thiserror~docs]{{hi:thiserror}}↗ if you are a library that wants to design your own dedicated error type(s) so that on failures the caller gets exactly the information that you choose.

## Simplify Error Handling and Attach Context to Errors with `anyhow` {#anyhow}

[![anyhow][c~anyhow~docs~badge]][c~anyhow~docs] [![anyhow~crates.io][c~anyhow~crates.io~badge]][c~anyhow~crates.io] [![anyhow~github][c~anyhow~github~badge]][c~anyhow~github] [![anyhow~lib.rs][c~anyhow~lib.rs~badge]][c~anyhow~lib.rs]{{hi:anyhow}}{{hi:Error}}{{hi:Error-handling}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`anyhow`][c~anyhow~docs]↗{{hi:anyhow}} provides `anyhow::Error`, a flexible concrete Error type built on [`std::error::Error`][c~std::error::Error~docs]↗{{hi:std::error::Error}}.

Use `Result<T, anyhow::Error>` or equivalently [`anyhow::Result{{hi:anyhow::Result}}<T>`][c~anyhow::Result~docs]↗ as the return type of any fallible function.

You can use [`anyhow`][c~anyhow~docs]↗{{hi:anyhow}} to:

- Create errors easily.
- Attach context to help the person troubleshooting the error understand where things went wrong.
- Work with dynamic error types (`Box<dyn Error>`) conveniently.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/anyhow.rs:example}}
```

[`anyhow`][c~anyhow~docs]↗{{hi:anyhow}} works with any error type that has an impl of [`std::error::Error`][c~std::error::Error~docs]↗{{hi:std::error::Error}}, including ones defined in your crate e.g. using [`thiserror`][c~thiserror~docs]{{hi:thiserror}}↗.

## Create Custom Error Types Declaratively with `thisError` {#thiserror}

[![thiserror][c~thiserror~docs~badge]][c~thiserror~docs] [![thiserror~crates.io][c~thiserror~crates.io~badge]][c~thiserror~crates.io] [![thiserror~github][c~thiserror~github~badge]][c~thiserror~github] [![thiserror~lib.rs][c~thiserror~lib.rs~badge]][c~thiserror~lib.rs]{{hi:thiserror}}{{hi:Derive}}{{hi:Error}}{{hi:Error-handling}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`thiserror`][c~thiserror~docs]{{hi:thiserror}}↗ provides a convenient [`derive`][book~rust-reference~derive]{{hi:derive}}↗ macro for the standard library's [`std::error::Error`][c~std::error::Error~docs]↗{{hi:std::error::Error}} trait.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/thiserror.rs:example}}
```

The [`#[error(...)]`]( )↗{{hi: }} messages support a shorthand for interpolating fields from the error.

```rust,editable,compile_fail,noplayground
#[error("{var}")]  //⟶ write!("{}", self.var)
#[error("{0}")]   //⟶ write!("{}", self.0)
#[error("{var:?}")] //⟶ write!("{:?}", self.var)
#[error("{0:?}")]  //⟶ write!("{:?}", self.0)
```

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/thiserror2.rs:example}}
```

## Provide Human-readable Diagnostics with `miette` {#miette}

[![miette][c~miette~docs~badge]][c~miette~docs] [![miette~crates.io][c~miette~crates.io~badge]][c~miette~crates.io] [![miette~github][c~miette~github~badge]][c~miette~github] [![miette~lib.rs][c~miette~lib.rs~badge]][c~miette~lib.rs]{{hi:miette}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`miette`][c~miette~docs]↗{{hi:miette}} is a fancy diagnostic reporting library and protocol.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/miette.rs:example}}
```

## Create Colorful and User-friendly Error Reports with `color-eyre` {#color-eyre}

[![color-eyre][c~color-eyre~docs~badge]][c~color-eyre~docs] [![color-eyre~crates.io][c~color-eyre~crates.io~badge]][c~color-eyre~crates.io] [![color-eyre~github][c~color-eyre~github~badge]][c~color-eyre~github] [![color-eyre~lib.rs][c~color-eyre~lib.rs~badge]][c~color-eyre~lib.rs]{{hi:color-eyre}}

[`color-eyre`][c~color-eyre~docs]↗{{hi:color-eyre}} is an error report handler for panics and `eyre::Reports` for colorful, consistent, and well formatted error reports for all kinds of errors.

It is a fork of [`anyhow`][c~anyhow::Result~docs]↗{{hi:anyhow}} [`anyhow`][c~anyhow~docs]↗{{hi:anyhow}} that gives you more control over the format of the generated error messages. It is recommended if you intend to present error messages to end users. Otherwise [`anyhow`][c~anyhow~docs]↗{{hi:anyhow}} is simpler.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/color_eyre.rs:example}}
```

## Related Topics {#related-topics}

[![eyre][c~eyre~docs~badge]][c~eyre~docs]{{hi:eyre}}

Do not use [Error Chain][c~error-chain~docs]{{hi:error-chain}}↗, which is deprecated.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[error_customization: write / organize](https://github.com/john-cd/rust_howto/issues/463)
color-eyre is archived -> eyre.

- [eyre-rs][c~eyre~github]↗.

</div>
