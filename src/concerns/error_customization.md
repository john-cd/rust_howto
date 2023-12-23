# Custom Errors

Use `Anyhow` if you don't care what error type your functions return, you just want it to be easy. This is common in application code.
Use `thiserror` if you are a library that wants to design your own dedicated error type(s) so that on failures the caller gets exactly the information that you choose.

## Anyhow

[Anyhow]( https://crates.io/crates/anyhow )

Use `Result<T, anyhow::Error>` or equivalently `anyhow::Result<T>` as the return type of any fallible function.

```rust,editable,ignore
use anyhow::{Context, Result};

fn main() -> Result<()> {
    ...
    it.detach().context("Failed to detach the important thing")?; // Attach context

    let content = std::fs::read(path)
        .with_context(|| format!("Failed to read instrs from {}", path))?;
}
```

Anyhow works with any error type that has an impl of `std::error::Error`, including ones defined in your crate e.g. using `thiserror`.

## thisError

[thisError]( https://docs.rs/thiserror/latest/thiserror/ ) provides a convenient `derive` macro for the standard library’s `std::error::Error` trait.

```rust,editable,ignore
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    // A Display impl is generated for your error if you provide #[error("...")] messages on the struct or each variant of your enum
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),  // A From impl is generated for each variant containing a #[from] attribute.

    #[error("the data for key `{0}` is not available")]
    Redaction(String),

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },

    #[error("unknown data store error")]
    Unknown,

    #[error(transparent)]  //  forward the source and Display methods straight through to an underlying error without adding an additional message.
    Other(#[from] anyhow::Error),
}

fn main() {}
```

The `#[error(...)]` messages support a shorthand for interpolating fields from the error.

```rust,editable,ignore
#[error("{var}")] ⟶ write!("{}", self.var)
#[error("{0}")] ⟶ write!("{}", self.0)
#[error("{var:?}")] ⟶ write!("{:?}", self.var)
#[error("{0:?}")] ⟶ write!("{:?}", self.0)
```

```rust,editable,ignore
#[derive(Error, Debug)]
pub struct MyError {
    msg: String,
    // The Error trait’s source() method is implemented to return whichever field has a #[source] attribute or is named source, if any. This is for identifying the underlying lower level error that caused your error. #[from] implies #[source].
    #[source]
    source: anyhow::Error,
    backtrace: std::backtrace::Backtrace,  // automatically detected to implement provide()
}

fn main() {}
```

## Miette

[Miette]( https://lib.rs/crates/miette ) prints fancy diagnostics upon error.

```rust,editable,ignore
use miette::{Diagnostic, SourceSpan, NamedSource};

// In library code, `thiserror` plays nicely with `miette` to define unique error types and error wrappers
use thiserror::Error;

// You can derive a `Diagnostic` from any `std::error::Error` type.
#[derive(Error, Diagnostic, Debug)]
pub enum MyLibError {
    #[error("A bad thing happened!")] // provided by `thisError`
    #[diagnostic(
    // Use `#[diagnostic(code(...))]` to set the unique code for this error.
    code(my_lib::bad_thing),
    // Set the URL that will be displayed as an actual link in supported terminals.
    // `url(docsrs)` automatically create a link to this diagnostic on docs.rs
    // or use a custom URK like `url("https://my_website.com/error_codes#{}", self.code)`
    url(docsrs),
    // Supply help text
    help("try doing it better next time?"))]
    BadThingHappened,

    #[error("Something went wrong!")]
    SomethingWentWrong {
        // The Source that we're gonna be printing snippets out of.
        // This can be a String if you don't have or care about file names.
        #[source_code]
        src: NamedSource,
        // Snippets and highlights can be included in the diagnostic!
        // You may also use `(usize, usize)`, the byte-offset and length into an associated SourceCode
        // or `Option<SourceSpan>`
        #[label("This bit highlighted here is the problem")]
        bad_bit: SourceSpan,

        // Programmatically supply the help text
        #[help]
        advice: Option<String>, // Can also just be `String`

        // Related errors
        #[related]
        others: Vec<MyLibError>,
    },

    // Wrap an Error
    #[error(transparent)] // forward the source and Display methods straight through to an underlying error.
    #[diagnostic(code(my_lib::io_error))]
    IoError(#[from] std::io::Error),

    // Wrap another Diagnostic
    // Use `#[diagnostic(transparent)]` to wrap another [`Diagnostic`]. You won't see labels otherwise
    #[error(transparent)]
    #[diagnostic(transparent)]
    AnotherError(#[from] AnotherError),
}

#[derive(Error, Diagnostic, Debug)]
#[error("another error")]
pub struct AnotherError {
    #[label("here")]
    pub at: SourceSpan,
}

fn this_fails() -> Result<()> {
    // You can use plain strings as a `Source`, or anything that implements
    // the one-method `Source` trait.
    let src = "source\n  text\n    here".to_string();
    // You may also use map_err(|error| { error.with_source_code(String::from("source code")) }) later.

    Err(MyLibError::SomethingWentWrong {
        src: NamedSource::new("bad_file.rs", src),
        bad_bit: (9, 4).into(),
        advice: Some("Some help text".to_string()),
        others: vec![MyLibError::BadThingHappened],
    })?;
    Ok(())
}

use miette::Result;

// To get errors printed nicely in application code, just return a `Result<()>`
// Note: You can swap out the default reporter for a custom one using
// `miette::set_hook()`
fn main() -> Result<()> {
    this_fails()?;
    Ok(())
}
```

## See also

[eyre]( https://docs.rs/eyre/latest/eyre/ )
