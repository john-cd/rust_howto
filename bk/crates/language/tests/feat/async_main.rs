// ANCHOR: example
use anyhow::Result;

/// This is an example of an async main function.
///
/// It uses the `tokio::main` macro to create an async runtime.
/// It returns a `Result` to allow for error handling.
#[tokio::main]
async fn main() -> Result<()> {
    println!("I'm async!");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
