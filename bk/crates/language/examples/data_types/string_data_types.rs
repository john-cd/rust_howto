#![allow(dead_code)]
// ANCHOR: example
fn main() {
    // Basic string types:
    let u: &str = "Hello"; // String slice.
    let v: String = String::from("World"); // Owned string.

    println!("String slice: {u}");
    println!("Owned string: {v}");

    println!(
        "String concatenation: {} + {} = {}",
        u,
        v,
        u.to_string() + &v
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
