// ANCHOR: example
use std::error::Error;

/// Parses a string slice into a `u16` representing a port number.
///
/// # Errors
///
/// This function will return an error if the string cannot be parsed as a `u16`
/// or if the parsed value is zero.
fn parse_port(s: &str) -> Result<u16, Box<dyn Error>> {
    // We need to use `Box<dyn Error>` as the error type, because the returned
    // error type cannot be determined during compile time: It will either
    // contain an instance of `std::num::ParseIntError` (from the parse
    // method, when parsing fails), or a string (when the port is zero).
    // `Box` encapsulates the `dyn Error` trait object, because it is
    // dynamically sized. The trait object enables "late binding" a.k.a.
    // virtual dispatch at runtime, depending on the actual Error type.
    // Alternatively, you may return `anyhow::Result` - the `anyhow` crate
    // handles the complexity for you.
    let port: u16 = s.parse()?;
    if port == 0 {
        Err(Box::from(format!("Invalid port: {}", port)))
    } else {
        Ok(port)
    }
}

fn main() {
    match parse_port("123") {
        Ok(port) => println!("Port: {}", port),
        Err(err) => panic!("{}", err),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
