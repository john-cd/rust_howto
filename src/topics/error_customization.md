# Custom Errors

Use `Anyhow` if you don't care what error type your functions return, you just want it to be easy. This is common in application code.
Use `thiserror` if you are a library that wants to design your own dedicated error type(s) so that on failures the caller gets exactly the information that you choose.

## Anyhow

[Anyhow]( https://crates.io/crates/anyhow )

Use `Result<T, anyhow::Error>` or equivalently `anyhow::Result<T>` as the return type of any fallible function.

```rust,ignore
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

```rust,ignore
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

```rust,ignore
#[error("{var}")] ⟶ write!("{}", self.var)
#[error("{0}")] ⟶ write!("{}", self.0)
#[error("{var:?}")] ⟶ write!("{:?}", self.var)
#[error("{0:?}")] ⟶ write!("{:?}", self.0)
```

```rust,ignore
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

See also [eyre]( https://docs.rs/eyre/latest/eyre/ )
