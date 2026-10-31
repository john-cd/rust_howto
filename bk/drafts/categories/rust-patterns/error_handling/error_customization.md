<!-- TODO Reviewed ??-??-2026 -->

# Custom Errors

{{#include error_customization.incl.md}}

Rust provides [`Result`][c~std::result::Result~docs]↗{{hi:Result}} and the [`std::error::Error`][c~std::error::Error~docs]↗{{hi:Error}} trait. Choose a representation based on who needs to inspect the failure and how the application presents it.

- **Library API:** define a specific public error enum with [`thiserror`][c~thiserror~docs]↗{{hi:thiserror}} so callers can match variants and act on them.
- **Application flow:** use [`anyhow`][c~anyhow~docs]↗{{hi:anyhow}} to propagate different causes and add context where operations fail.
- **Application reports:** use [`eyre`][c~eyre~docs]↗{{hi:eyre}} for a flexible report type with a customizable handler; [`color-eyre`][c~color-eyre~docs]↗{{hi:color-eyre}} adds formatted reports and panic hooks.
- **Source diagnostics:** use [`miette`][c~miette~docs]↗{{hi:miette}} when diagnostics need codes, help, or highlighted source snippets.

These approaches can be combined: a library can expose a `thiserror` type while an application adds context or formats it as a report. Avoid exposing an application report type as a library's public error contract when callers need stable, matchable failure cases.

## Simplify Error Handling and Attach Context to Errors with `anyhow` {#anyhow}

{{#badges anyhow no-std="No standard library" rust-patterns="Rust patterns"}}{{hi:Error}}{{hi:Error-handling}}

[`anyhow`][c~anyhow~docs]↗{{hi:anyhow}} provides `anyhow::Error`, a flexible concrete Error type built on [`std::error::Error`][c~std::error::Error~docs]↗{{hi:std::error::Error}}.

Use `Result<T, anyhow::Error>` or equivalently [`anyhow::Result{{hi:anyhow::Result}}<T>`][c~anyhow::Result~docs]↗ as the return type of any fallible function.

You can use [`anyhow`][c~anyhow~docs]↗{{hi:anyhow}} to:

- Create errors easily.
- Attach context to help the person troubleshooting the error understand where things went wrong.
- Work with dynamic error types (`Box<dyn Error>`) conveniently.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/anyhow.rs:example}}
```

[`anyhow`][c~anyhow~docs]↗{{hi:anyhow}} works with any error type that has an impl of [`std::error::Error`][c~std::error::Error~docs]↗{{hi:std::error::Error}}, including ones defined in your crate e.g. using [`thiserror`][c~thiserror~docs]↗{{hi:thiserror}}.

## Create Custom Error Types Declaratively with `thisError` {#thiserror}

{{#badges thiserror rust-patterns="Rust patterns"}}{{hi:Derive}}{{hi:Error}}{{hi:Error-handling}}

[`thiserror`][c~thiserror~docs]↗{{hi:thiserror}} provides a convenient [`derive`][book~rust-reference~derive]↗{{hi:derive}} macro for the standard library's [`std::error::Error`][c~std::error::Error~docs]↗{{hi:std::error::Error}} trait.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/thiserror.rs:example}}
```

Use `#[error("...")]` to define a `Display` message. Named fields can be interpolated by name, tuple fields by position, and `:?` requests debug formatting. The included examples show field interpolation and transparent source wrapping.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/thiserror2.rs:example}}
```

## Provide Human-readable Diagnostics with `miette` {#miette}

{{#badges miette rust-patterns="Rust patterns"}}

[`miette`][c~miette~docs]↗{{hi:miette}} is a fancy diagnostic reporting library and protocol.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/miette.rs:example}}
```

## Create Colorful and User-friendly Error Reports with `color-eyre` {#color-eyre}

{{#badges color-eyre}}

[`color-eyre`][c~color-eyre~docs]↗{{hi:color-eyre}} installs panic and report hooks for [`eyre`][c~eyre~docs]↗{{hi:eyre}}. It captures diagnostic context and formats error chains for terminal applications. Choose it when the application owns top-level error presentation; use `anyhow` when contextual propagation is enough and a custom report handler is not needed.

```rust,editable
{{#include ../../../../crates/cats/rust_patterns/examples/error/color_eyre.rs:example}}
```

## Related Topics {#related-topics .skip}

[![eyre][c~eyre~docs~badge]][c~eyre~docs]{{hi:eyre}}

- [Error handling basics][ex~error_handling~result] and the [`Result`][c~std::result::Result~docs]↗{{hi:Result}} type.
- [Derive macros][p~macros] used by `thiserror` and `miette`.
- [Error Chain][c~error-chain~docs]↗{{hi:error-chain}} is deprecated; prefer maintained crates such as those described above.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
