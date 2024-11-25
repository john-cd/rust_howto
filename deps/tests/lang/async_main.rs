// ANCHOR: example
use anyhow::Result;

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
