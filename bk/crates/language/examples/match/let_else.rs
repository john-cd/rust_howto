#![allow(dead_code)]
// ANCHOR: example
fn parse_number(input: &str) -> u32 {
    // Try to parse the input string.
    let Ok(num) = input.parse::<u32>() else {
        // If the binding fails, the `else` block is triggered and the function
        // panics.
        panic!("Failed to parse number: {input}");
    };
    // If it succeeds, `num` is available in the surrounding scope.
    num
}

fn main() {
    let value = parse_number("42");
    println!("Parsed value: {}", value);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
