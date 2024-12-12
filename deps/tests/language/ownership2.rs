// ANCHOR: example
fn main() {
    {
        let s = String::from("hello");
        println!("{}", s);
    } // The `s` variable is now out of scope - Rust calls `drop`

    // ERROR println!("{}", s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
