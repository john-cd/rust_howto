// ANCHOR: example

/// This example demonstrates the use of the `clone` method to create a deep
/// copy of a `String`.
fn main() {
    // Create a String on the heap.
    let s1 = String::from("hello");
    // Clone the String, creating a new String with the same content on the
    // heap.
    let s2 = s1.clone();
    // `clone` deeply copies the heap data of the `String`,
    // not just the stack data.
    println!("{s2}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
