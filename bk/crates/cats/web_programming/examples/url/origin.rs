// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates parsing a URL and extracting its origin components.
//!
//! A web origin is the scheme, host and port of a URL. Browsers use it for the
//! same-origin policy, which stops a script on one site from reading data from
//! another unless that site allows it (see CORS).

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

pub fn run() -> Result<(), ParseError> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the origin example through its entry point.
    #[test]
    fn test_origin_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
