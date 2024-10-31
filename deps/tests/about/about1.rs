use std::net::IpAddr;
use std::str;

use anyhow::Result;

fn main() -> Result<()> {
    let bytes = b"2001:db8::1";

    // Bytes to string.
    let s = str::from_utf8(bytes)?;

    // String to IP address.
    let addr: IpAddr = s.parse()?;

    println!("{:?}", addr);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
