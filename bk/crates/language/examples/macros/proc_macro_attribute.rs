// ANCHOR: example
// Import our custom attribute macro.
use proc_macros::log_calls;

// Apply the custom attribute macro.
// This will log the function call.
#[log_calls]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(5, 7);
    println!("Sum: {sum}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
