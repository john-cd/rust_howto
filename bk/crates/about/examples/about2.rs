// ANCHOR: example
//! This example demonstrates how to parse a URL and extract a portion of it.

use url::Position;
use url::Url;

// If `parse` fails, `main` returns `Err` and the error message is displayed on
// the console.
fn example() -> anyhow::Result<()> {
    // Parse a URL. This call can fail if the URL is invalid. It early returns
    // via `?`:
    let parsed = Url::parse("https://httpbin.org/cookies/set?k2=v2&k1=v1")?;

    // Extract the portion of the URL, up to the end of the path.
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {cleaned}");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    example()
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        example()?;
        Ok(())
    }
}
