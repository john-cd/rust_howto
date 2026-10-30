// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Propagating errors with the `?` operator through a function that parses a
//! port.

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
    // Alternatively, return `anyhow::Result` - the `anyhow` crate
    // handles the complexity for you.
    let port: u16 = s.parse()?;
    if port == 0 {
        Err(Box::from(format!("Invalid port: {port}")))
    } else {
        Ok(port)
    }
}

/// Parses valid and invalid ports and prints the result.
fn main() {
    match parse_port("123") {
        Ok(port) => println!("Port: {port}"),
        Err(err) => panic!("{err}"),
    }
    // Test with an invalid port number (zero).
    // match parse_port("0") {
    //     Ok(port) => println!("Port: {port}"),
    //     Err(err) => panic!("{err}"),
    // }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the question mark2 example through its entry point.
    #[test]
    fn test_question_mark2_example_entry_point() {
        main();
    }

    /// A valid port is returned.
    #[test]
    fn parses_a_valid_port() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(parse_port("123")?, 123);
        Ok(())
    }

    /// A non-numeric or out-of-range port is an error.
    #[test]
    fn rejects_invalid_ports() {
        assert!(parse_port("abc").is_err());
        assert!(parse_port("70000").is_err());
    }

    /// Port zero is rejected with a message.
    #[test]
    fn rejects_port_zero() {
        assert_eq!(
            parse_port("0").err().map(|e| e.to_string()).as_deref(),
            Some("Invalid port: 0")
        );
    }
}
