// ANCHOR: example
//! This example demonstrates converting a byte slice representing an IPv6
//! address to an `IpAddr` type.

use std::net::IpAddr;
use std::str;

// This function includes calls that can fail.
// - Errors are caught and propagated back to the caller with the `?` operator.
// - Note how the function returns a `Result` type.
// - We have chosen to use the flexible `Result` type alias from the very
//   commonly used `anyhow` library.
// - This function does not panic.
fn example() -> anyhow::Result<()> {
    // A byte slice representing an IPv6 address:
    let bytes = b"2001:db8::1";

    // Convert the byte slice to a string slice.
    // This operation can fail if the bytes are not UTF-8. Therefore it returns
    // a `Result`. The `?` operator causes an early return in case of error:
    let s = str::from_utf8(bytes)?;

    // Parse the String to an IP address.
    // This only executes if the previous function call returns `Ok`.
    // This operation can fail as well.
    let addr: IpAddr = s.parse()?;

    // This only executes if `parse` returns `Ok`:
    println!("{addr:?}");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    example()?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
