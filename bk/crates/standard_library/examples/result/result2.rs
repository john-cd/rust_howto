#![allow(dead_code)]
// ANCHOR: example
//! Example of `Result` methods for value transformation and control flow.

// A contrived example of a function returning a `Result`:
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

#[allow(clippy::bind_instead_of_map)]
fn main() {
    let result = divide(10, 2)
        // Transform the inner value if `Ok`.
        .map(|x| x * 2)
        // Chain another `Result`-returning operation.
        .and_then(|x| Ok(x + 1));

    // Check result status.
    println!("Is result Ok? {}", result.is_ok());
    println!("Is result Err? {}", result.is_err());

    println!("Final value: {}", result.unwrap_or(-1)); // Provide a default value if `Err`.
    // You could also write: .or_else(|_| Ok(-1));

    // TODO fix
    // Ok("").expect("failed to write message"); // Panic with a provided
    // custom message if `Err`.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
