#![allow(dead_code)]
// ANCHOR: example
use url::ParseError;
use url::Url;

/// Demonstrates URL parsing.
///
/// This function parses a URL string and prints the path part of the URL.
fn main() -> Result<(), ParseError> {
    // Define a URL string.
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), ParseError> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
