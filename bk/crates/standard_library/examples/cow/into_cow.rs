#![allow(dead_code)]
// ANCHOR: example
use std::borrow::Cow;

enum HttpStatus {
    Ok,
    NotFound,
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

fn main() {
    let status1 = HttpStatus::Ok;
    let status2 = HttpStatus::NotFound;
    let status3 = HttpStatus::Custom(500, "Internal Server Error".to_string());

    println!("{}", describe_status(&status1)); // Output: Status: 200 OK
    println!("{}", describe_status(&status2)); // Output: Status: 404 Not Found
    println!("{}", describe_status(&status3)); // Output: Status: 500 Internal Server Error
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
