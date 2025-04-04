// ANCHOR: example

/// This example demonstrates the concept of ownership and move semantics in
/// Rust.
fn main() {
    // Strings have move semantics.
    // `s1` owns the string "hello".
    let s1 = String::from("hello");
    // `s1` is MOVED into `s2`.
    // This is NOT a shallow copy.
    // `s2` now owns the string "hello".
    let s2 = s1;
    println!("{}, world!", s2);
    // Rust has invalidated `s1` because the String has been moved to `s2`.
    // ERROR: println!("{}, world!", s1);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
