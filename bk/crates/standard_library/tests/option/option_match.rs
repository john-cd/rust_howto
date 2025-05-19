// ANCHOR: example
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result1 = divide(10.0, 2.0);
    match result1 {
        Some(value) => println!("Result: {}", value), /* Handles the `Some`
                                                        * case */
        None => println!("Cannot divide by zero!"), // Handles the `None` case
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
