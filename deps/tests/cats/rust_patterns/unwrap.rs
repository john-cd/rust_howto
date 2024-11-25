// ANCHOR: example
// use std::str::FromStr;

fn main() {
    let number_str = "42";

    // `parse()` attempts to convert the string into a number.
    // This operation can fail if the string isn't a valid number, so it returns
    // a Result<u32, ParseIntError>.
    let number: u32 = number_str.parse().unwrap();
    // unwrap() is called on the Result to extract the u32 value.
    // If the parsing is successful, the value is assigned to number.
    // If the parsing fails, the program panics with an error message.

    println!("The number is: {}", number);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
