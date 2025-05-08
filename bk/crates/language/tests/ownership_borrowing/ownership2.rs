// ANCHOR: example
fn main() {
    // `s` is not valid here, it's not yet declared.
    {
        // `s` is valid from this point forward.
        let s = String::from("hello");
        println!("{}", s);
    } // this scope is now over, and `s` is no longer valid.
    // Rust calls `drop`.

    // ERROR println!("{}", s); // `s` is not valid here, it's no longer in
    // scope.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
