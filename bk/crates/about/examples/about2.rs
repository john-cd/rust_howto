// ANCHOR: example
//! This example demonstrates how to parse a URL and extract a portion of it.

use anyhow::Result;
use url::Position;
use url::Url;

fn main() -> Result<()> {
    let parsed = Url::parse("https://httpbin.org/cookies/set?k2=v2&k1=v1")?;
    // Extract the portion of the URL up to the end of the path.
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {cleaned}");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
