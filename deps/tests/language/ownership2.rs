// ANCHOR: example
fn main() {
    {
        let _s = String::from("hello");
    } // Variable out of scope - Rust calls `drop`

    // ERROR println!("{}", s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
