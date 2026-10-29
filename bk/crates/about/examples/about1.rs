// Reviewed 10-27-2026.

// ANCHOR: example
//! This example demonstrates converting a byte slice representing an IPv6
//! address to an `IpAddr` type.

use std::net::IpAddr;
use std::str;

// The following function includes calls that can fail:
// - Errors are caught and propagated back to the caller with the `?` operator.
// - Note how the function returns a `Result` type.
// - We have chosen to use the flexible `Result` type alias from the very
//   commonly used `anyhow` library.
// - This function does not panic.

/// Converts a byte slice holding an IPv6 address into an `IpAddr`.
///
/// # Errors
/// Returns an error if the bytes are not valid UTF-8 or not a valid address.
fn example() -> anyhow::Result<IpAddr> {
    // A byte slice representing an IPv6 address:
    let bytes: &[u8] = b"2001:db8::1";

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

    // Happy path: we return `Ok`.
    Ok(addr)
}

fn main() -> anyhow::Result<()> {
    example()?;
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main().expect("main failed");
    }

    #[test]
    fn test_example_parses_ipv6_address()
    -> Result<(), Box<dyn std::error::Error>> {
        let addr = example()?;
        assert_eq!(addr, "2001:db8::1".parse::<IpAddr>()?);
        assert!(addr.is_ipv6());
        Ok(())
    }

    #[test]
    fn test_invalid_utf8_is_an_error() {
        let bytes = vec![0xff, 0xfe];
        assert!(str::from_utf8(&bytes).is_err());
    }

    #[test]
    fn test_invalid_address_is_an_error() {
        assert!("not an address".parse::<IpAddr>().is_err());
    }
}
