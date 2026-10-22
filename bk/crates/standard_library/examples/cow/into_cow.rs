#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Returning a borrowed or an owned string with `Cow`.

use std::borrow::Cow;

/// An HTTP status.
enum HttpStatus {
    /// The request succeeded.
    Ok,
    /// The resource was not found.
    NotFound,
    /// A status with a custom message.
    Custom(u16, String),
}

/// Call `into` to convert a string slice or a `String` into a `Cow` with
/// minimum fuss.
fn describe_status(status: &HttpStatus) -> Cow<'static, str> {
    // Note the return type.
    match status {
        // Convert string literals, of type `&'static str`, into a `Cow`.
        HttpStatus::Ok => "Status: 200 OK".into(),
        HttpStatus::NotFound => "Status: 404 Not Found".into(),
        HttpStatus::Custom(code, message) => {
            // Convert a `String`, which is dynamically built by `format!`, into
            // a `Cow`.
            format!("Status: {code} {message}").into()
        }
    }
}

/// Describes several statuses.
fn main() {
    let status1 = HttpStatus::Ok;
    let status2 = HttpStatus::NotFound;
    let status3 = HttpStatus::Custom(500, "Internal Server Error".to_owned());

    println!("{}", describe_status(&status1)); // Output: Status: 200 OK
    println!("{}", describe_status(&status2)); // Output: Status: 404 Not Found
    println!("{}", describe_status(&status3)); // Output: Status: 500 Internal Server Error
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
