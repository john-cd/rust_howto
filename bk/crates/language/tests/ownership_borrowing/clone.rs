// ANCHOR: example
/// This example demonstrates the use of the `clone` method
/// to create a deep copy of a `String`.
fn main() {
    // Create a String on the heap.
    let mut s1 = String::from("hello");

    // Clone the String, creating a new String with the same content on the
    // heap.
    let s2 = s1.clone();
    // The implementation of `Clone` for `String` allocates a new memory chunk
    // on the heap, deep copies the pointed-to string buffer of `s1` into
    // it, and stores a pointer to the new heap location.

    // Both `s1` and `s2` remain accessible after the clone.
    println!("{s1}");
    println!("{s2}");

    assert_eq!(s1, s2); // `s1` and `s2` have the same contents...

    // ...but `s1` and `s2` do NOT point to the same heap memory location.
    assert!(!std::ptr::eq(s1.as_ptr(), s2.as_ptr()));

    // `s1` and `s2` are independent of each other. Let's modify `s1`:
    s1.push('!');
    assert_ne!(s1, s2);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
