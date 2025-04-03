// ANCHOR: example
//! This example demonstrates converting a byte slice representing an IPv6 address
//! to an `IpAddr` type.

use std::net::IpAddr;
use std::str;

use anyhow::Result;

fn main() -> Result<()> {
    // A byte slice representing an IPv6 address.
    let bytes = b"2001:db8::1";

    // Convert the byte slice to a string slice.
    let s = str::from_utf8(bytes)?;

    // Parse the String to an IP address.
    let addr: IpAddr = s.parse()?;

    println!("{:?}", addr);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
