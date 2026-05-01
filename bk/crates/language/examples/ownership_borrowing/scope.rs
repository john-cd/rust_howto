#![allow(dead_code)]
// ANCHOR: example
fn main() {
    // `s` is not valid here, it is not yet declared.

    {
        // Start of the scope.

        // When a String is created, it requests memory from the heap.
        let s = String::from("hello");
        // `s` is valid from this point forward.

        println!("{s}");
    }
    // The scope is now over, and `s` is no longer valid.
    // Rust automatically returns this memory to the allocator.
    // This prevents memory leaks.

    // ERROR: println!("{s}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
