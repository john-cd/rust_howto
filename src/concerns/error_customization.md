# Custom Errors

Use `Anyhow` if you don't care what error type your functions return, you just want it to be easy. This is common in application code.
Use `thiserror` if you are a library that wants to design your own dedicated error type(s) so that on failures the caller gets exactly the information that you choose.

## Anyhow

[Anyhow (crates.io)][anyhow-crate]⮳

Use `Result<T, anyhow::Error>` or equivalently `anyhow::Result<T>` as the return type of any fallible function.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/anyhow.rs}}
```

Anyhow works with any error type that has an impl of `std::error::Error`, including ones defined in your crate e.g. using `thiserror`.

## thisError

[thisError][thisError]⮳ provides a convenient `derive` macro for the standard library’s `std::error::Error` trait.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/thiserror.rs}}
```

The `#[error(...)]` messages support a shorthand for interpolating fields from the error.

```rust,editable,ignore
#[error("{var}")] ⟶ write!("{}", self.var)
#[error("{0}")] ⟶ write!("{}", self.0)
#[error("{var:?}")] ⟶ write!("{:?}", self.var)
#[error("{0:?}")] ⟶ write!("{:?}", self.0)
```

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/thiserror2.rs}}
```

## Miette

[Miette (lib.rs)][miette-librs]⮳ prints fancy diagnostics upon error.

```rust,editable,ignore
# extern crate miette;
{{#include ../../deps/examples/miette/mylib.rs}}
```

```rust,editable,ignore
# extern crate miette;
{{#include ../../deps/examples/miette/main.rs}}
```

## See also

[eyre][eyre]⮳

{{#include ../link-refs.md}}
