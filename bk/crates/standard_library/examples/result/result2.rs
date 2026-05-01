#![allow(dead_code)]
// ANCHOR: example
//! Example of `Result` methods for value transformation and control flow.

// A contrived example of a function returning a `Result`:
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Cannot divide by zero.")
    } else {
        Ok(a / b)
    }
}

#[allow(clippy::bind_instead_of_map)]
fn main() {
    let result = divide(10, 2)
        // Transform the inner value if `Ok`:
        .map(|x| x * 2)
        // Chain another `Result`-returning operation:
        .and_then(|x| Ok(x + 1));

    // In rare cases, you may want to panic with a custom message if `Err`.
    // This converts a recoverable error into an irrecoverable one:
    result.expect("Computation failed.");
    // `expect` is preferred over `unwrap`:
    result.unwrap();

    // It is recommended to check result status before unwrapping,
    // or use an `if let` expression instead:
    println!("Is result `Ok`? {}", result.is_ok());
    println!("Is result `Err`? {}", result.is_err());

    if let Ok(ref value) = result {
        println!("Result value: {value}");
    } else {
        println!("Computation failed.");
    }

    // You can also provide a default value if `Err`:
    println!("Final value: {}", result.unwrap_or(-1));
    // You could also write: .or_else(|_| Ok(-1));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
