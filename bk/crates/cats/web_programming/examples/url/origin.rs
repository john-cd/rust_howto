#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Demonstrates parsing a URL and extracting its origin components.

use url::Host;
use url::ParseError;
use url::Url;

/// Parses a URL and checks its scheme, host and port.
fn main() -> Result<(), ParseError> {
    let s = "ftp://rust-lang.org/examples";

    let url = Url::parse(s)?;

    assert_eq!(url.scheme(), "ftp");
    assert_eq!(url.host(), Some(Host::Domain("rust-lang.org")));
    assert_eq!(url.port_or_known_default(), Some(21));
    println!("The origin is as expected!");

    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
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
