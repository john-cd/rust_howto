#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to parse a URL and extract its origin.
//!
//! The `url::Url` struct is used to parse the URL string.
//! The `url::Host` enum is used to represent the host part of the URL.
//! The `url::Origin` enum is used to represent the origin of the URL.
use anyhow::Result;
use url::Host;
use url::Origin;
use url::Url;

fn main() -> Result<()> {
    let s = "ftp://rust-lang.org/examples";

    let url = Url::parse(s)?;

    let expected_scheme = "ftp".to_owned();
    let expected_host = Host::Domain("rust-lang.org".to_owned());
    let expected_port = 21;
    let expected = Origin::Tuple(expected_scheme, expected_host, expected_port);

    let origin = url.origin();
    assert_eq!(origin, expected);
    println!("The origin is as expected!");

    Ok(())
}
// ANCHOR_END: example_start

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
