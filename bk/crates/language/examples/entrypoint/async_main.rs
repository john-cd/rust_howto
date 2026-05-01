#![allow(dead_code)]
// ANCHOR: example
use anyhow::Result;

/// This is an example of an async main function.
///
/// It uses the `tokio::main` macro to create an async runtime.
/// It returns a `Result` to allow for error handling.
///
/// Add `tokio` with at least the "macros" and
/// "rt-multi-thread" features to your `Cargo.toml`:
///
/// ```toml
/// tokio = { version = "1", features = ["full"] }
/// ```
#[tokio::main]
async fn main() -> Result<()> {
    println!("I'm async!");
    // You can .await futures here.
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
